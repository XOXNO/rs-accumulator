//! Royalties collected in the reward token itself used to revert with "division by 0",
//! so they could never be paid out.

use accumulator::{storage::StorageModule, Accumulator};
use multiversx_sc::types::*;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    whitebox_legacy::BlockchainStateWrapper,
};

const XOXNO: &[u8] = b"XOXNO-c1293a";
const LXOXNO: &[u8] = b"LXOXNO-0eb983";

/// Liquid staking mock: returns LXOXNO 1:1 for the XOXNO delegated.
mod liquid {
    multiversx_sc::imports!();

    #[multiversx_sc::contract]
    pub trait Liquid {
        #[init]
        fn init(&self) {}

        #[payable("*")]
        #[endpoint(delegate)]
        fn delegate(&self, _to: OptionalValue<ManagedAddress>) {
            let amount = self.call_value().single_esdt().amount.clone();
            self.tx()
                .to(&self.blockchain().get_caller())
                .single_esdt(&TokenIdentifier::from(super::LXOXNO), 0, &amount)
                .transfer();
        }
    }
}

#[test]
fn reward_token_royalties_are_staked_and_split() {
    let mut world = BlockchainStateWrapper::new();
    let owner = world.create_user_account(&rust_biguint!(0));
    let creators = [
        (world.create_user_account(&rust_biguint!(0)), 750u64),
        (world.create_user_account(&rust_biguint!(0)), 250),
    ];
    let accumulator = world.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        accumulator::contract_obj,
        "src/accumulator.rs",
    );
    let liquid = world.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        liquid::contract_obj,
        "tests/distribute_royalties.rs#liquid",
    );
    world.set_esdt_balance(liquid.address_ref(), LXOXNO, &rust_biguint!(1_000));
    world.set_esdt_balance(&owner, XOXNO, &rust_biguint!(1_000));
    world
        .execute_tx(&owner, &accumulator, &rust_biguint!(0), |sc| {
            sc.init(
                managed_address!(liquid.address_ref()),
                managed_biguint!(1_000),
                managed_biguint!(3_000),
                EgldOrEsdtTokenIdentifier::esdt(XOXNO),
                managed_token_id!(LXOXNO),
                managed_address!(liquid.address_ref()),
            );
        })
        .assert_ok();
    for (creator, amount) in &creators {
        world
            .execute_esdt_transfer(
                &owner,
                &accumulator,
                XOXNO,
                0,
                &rust_biguint!(*amount),
                |sc| {
                    sc.deposit_royalties(managed_address!(creator));
                },
            )
            .assert_ok();
    }

    world
        .execute_tx(&owner, &accumulator, &rust_biguint!(0), |sc| {
            let mut list = MultiValueEncoded::new();
            for (creator, _) in &creators {
                list.push(managed_address!(creator));
            }
            sc.distribute_royalties(
                &EgldOrEsdtTokenIdentifier::esdt(XOXNO),
                ManagedArgBuffer::new(),
                list,
            );
        })
        .assert_ok();

    for (creator, amount) in &creators {
        world.check_esdt_balance(creator, LXOXNO, &rust_biguint!(*amount));
    }
    world.check_esdt_balance(accumulator.address_ref(), XOXNO, &rust_biguint!(0));
    world.check_esdt_balance(accumulator.address_ref(), LXOXNO, &rust_biguint!(0));
    world
        .execute_query(&accumulator, |sc| assert!(sc.creators().is_empty()))
        .assert_ok();
}
