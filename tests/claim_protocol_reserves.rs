//! A revenue entry larger than the held balance must not make `claimProtocolReserves`
//! fail for every token.

use accumulator::{storage::StorageModule, Accumulator};
use multiversx_sc::types::*;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    whitebox_legacy::BlockchainStateWrapper,
};

const XOXNO: &[u8] = b"XOXNO-c1293a";
const LXOXNO: &[u8] = b"LXOXNO-0eb983";
const USDC: &[u8] = b"USDC-c76f1f";

#[test]
fn underfunded_revenue_is_written_off_without_touching_royalties() {
    let mut world = BlockchainStateWrapper::new();
    let owner = world.create_user_account(&rust_biguint!(0));
    let creator = world.create_user_account(&rust_biguint!(0));
    let anyone = world.create_user_account(&rust_biguint!(0));
    let accumulator = world.create_sc_account(
        &rust_biguint!(7),
        Some(&owner),
        accumulator::contract_obj,
        "src/accumulator.rs",
    );
    // LXOXNO: nothing held. USDC: 50 held, of which 30 are a creator's royalties.
    world.set_esdt_balance(accumulator.address_ref(), USDC, &rust_biguint!(50));
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
            for (token, amount) in [
                (EgldOrEsdtTokenIdentifier::esdt(LXOXNO), 1_000u64),
                (EgldOrEsdtTokenIdentifier::esdt(USDC), 100),
                (EgldOrEsdtTokenIdentifier::egld(), 7),
            ] {
                sc.revenue().insert(token, managed_biguint!(amount));
            }
            let usdc = EgldOrEsdtTokenIdentifier::esdt(USDC);
            sc.creators().insert(managed_address!(&creator));
            sc.creator_tokens(&managed_address!(&creator))
                .insert(usdc.clone());
            sc.creator_royalties(&managed_address!(&creator), &usdc)
                .set(managed_biguint!(30));
        })
        .assert_ok();

    world
        .execute_tx(&anyone, &accumulator, &rust_biguint!(0), |sc| {
            sc.claim_protocol_reserves();
        })
        .assert_ok();

    // Fully funded EGLD is paid in full; USDC pays only what is not owed to the creator.
    world.check_egld_balance(&owner, &rust_biguint!(7));
    world.check_esdt_balance(&owner, USDC, &rust_biguint!(20));
    world.check_esdt_balance(&owner, LXOXNO, &rust_biguint!(0));
    world.check_esdt_balance(accumulator.address_ref(), USDC, &rust_biguint!(30));
    world
        .execute_query(&accumulator, |sc| assert!(sc.revenue().is_empty()))
        .assert_ok();
}
