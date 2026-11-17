//! `queryAllCreatorRoyalties` must handle EGLD royalties, and keep the output
//! encoding off-chain readers already decode (EGLD as the 4-byte "EGLD" identifier).

use accumulator::{query::QueryModule, storage::StorageModule, Accumulator};
use multiversx_sc::{codec::TopEncode, types::*};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    whitebox_legacy::BlockchainStateWrapper,
};

const XOXNO: &[u8] = b"XOXNO-c1293a";
const LXOXNO: &[u8] = b"LXOXNO-0eb983";

#[test]
fn all_creator_royalties_includes_egld() {
    let mut world = BlockchainStateWrapper::new();
    let owner = world.create_user_account(&rust_biguint!(0));
    let creator = world.create_user_account(&rust_biguint!(0));
    let accumulator = world.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        accumulator::contract_obj,
        "src/accumulator.rs",
    );
    world.set_egld_balance(&owner, &rust_biguint!(7));
    world.set_esdt_balance(&owner, XOXNO, &rust_biguint!(5));
    world
        .execute_tx(&owner, &accumulator, &rust_biguint!(0), |sc| {
            sc.init(
                managed_address!(&owner),
                managed_biguint!(1_000),
                managed_biguint!(3_000),
                EgldOrEsdtTokenIdentifier::esdt(XOXNO),
                managed_token_id!(LXOXNO),
                managed_address!(&owner),
            );
        })
        .assert_ok();
    world
        .execute_tx(&owner, &accumulator, &rust_biguint!(7), |sc| {
            sc.deposit_royalties(managed_address!(&creator));
        })
        .assert_ok();
    world
        .execute_esdt_transfer(&owner, &accumulator, XOXNO, 0, &rust_biguint!(5), |sc| {
            sc.deposit_royalties(managed_address!(&creator));
        })
        .assert_ok();

    world
        .execute_query(&accumulator, |sc| {
            let summary = sc.get_all_creator_royalties().to_vec();
            assert_eq!(summary.len(), 1);
            let entry = summary.get(0);
            assert_eq!(entry.creator, managed_address!(&creator));
            assert_eq!(entry.tokens.len(), 2);

            // Legacy layout per payment: identifier buffer, u64 nonce, BigUint amount.
            let mut legacy = Vec::new();
            legacy.extend_from_slice(creator.as_bytes());
            legacy.extend_from_slice(&2u32.to_be_bytes());
            for (token, amount) in [(&b"EGLD"[..], 7u8), (XOXNO, 5)] {
                legacy.extend_from_slice(&(token.len() as u32).to_be_bytes());
                legacy.extend_from_slice(token);
                legacy.extend_from_slice(&0u64.to_be_bytes());
                legacy.extend_from_slice(&1u32.to_be_bytes());
                legacy.push(amount);
            }
            let mut encoded = Vec::new();
            entry.top_encode(&mut encoded).unwrap();
            assert_eq!(encoded, legacy);

            assert!(sc.creators().contains(&managed_address!(&creator)));
        })
        .assert_ok();
}
