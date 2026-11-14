//! `distribute` is restricted to trusted callers, and a swap route that re-enters it
//! cannot spend the same recorded balance twice.

use accumulator::{storage::StorageModule, Accumulator};
use multiversx_sc::types::*;
use multiversx_sc_scenario::{
    api::DebugApi,
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    whitebox_legacy::{BlockchainStateWrapper, ContractObjWrapper},
};

const TOKEN: &[u8] = b"LXOXNO-0eb983";
const REWARD: &[u8] = b"XOXNO-c1293a";

type AccumulatorObj = accumulator::ContractObj<DebugApi>;
type AccumulatorWrapper = ContractObjWrapper<AccumulatorObj, fn() -> AccumulatorObj>;

/// Swap route that answers `xo` with 10 units of the reward token. When `reenter` is set
/// it first re-enters the accumulator's `distribute` once.
mod router {
    multiversx_sc::imports!();

    #[multiversx_sc::contract]
    pub trait Router {
        #[init]
        fn init(&self) {}

        #[payable("*")]
        #[endpoint(xo)]
        fn xo(&self) {
            let caller = self.blockchain().get_caller();
            if self.reenter().get() {
                self.reenter().set(false);
                self.tx()
                    .to(&caller)
                    .raw_call("distribute")
                    .argument(&EgldOrEsdtTokenIdentifier::esdt(super::TOKEN))
                    .argument(&ManagedArgBuffer::<Self::Api>::new())
                    .sync_call();
            }
            self.tx()
                .to(&caller)
                .single_esdt(
                    &TokenIdentifier::from(super::REWARD),
                    0,
                    &BigUint::from(10u64),
                )
                .transfer();
        }

        #[storage_mapper("reenter")]
        fn reenter(&self) -> SingleValueMapper<bool>;
    }
}

mod liquid {
    multiversx_sc::imports!();

    #[multiversx_sc::contract]
    pub trait Liquid {
        #[init]
        fn init(&self) {}

        #[payable("*")]
        #[endpoint(addRewards)]
        fn add_rewards(&self) {}
    }
}

/// Accumulator physically holds 300 TOKEN but has recorded only 100 for distribution;
/// the rest belongs to revenue and royalties.
fn with_setup(
    reenter: bool,
    test: impl FnOnce(&mut BlockchainStateWrapper, &Address, &Address, &AccumulatorWrapper, &Address),
) {
    let mut world = BlockchainStateWrapper::new();
    let owner = world.create_user_account(&rust_biguint!(0));
    let outsider = world.create_user_account(&rust_biguint!(0));
    let accumulator = world.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        accumulator::contract_obj as fn() -> AccumulatorObj,
        "src/accumulator.rs",
    );
    let router = world.create_sc_account(
        &rust_biguint!(0),
        Some(&outsider),
        router::contract_obj,
        "tests/distribute_guard.rs#router",
    );
    let liquid = world.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        liquid::contract_obj,
        "tests/distribute_guard.rs#liquid",
    );
    world.set_esdt_balance(accumulator.address_ref(), TOKEN, &rust_biguint!(300));
    world.set_esdt_balance(router.address_ref(), REWARD, &rust_biguint!(100));
    world.set_esdt_local_roles(accumulator.address_ref(), REWARD, &[EsdtLocalRole::Burn]);
    world
        .execute_tx(&outsider, &router, &rust_biguint!(0), |sc| {
            use router::Router;
            sc.reenter().set(reenter);
        })
        .assert_ok();
    world
        .execute_tx(&owner, &accumulator, &rust_biguint!(0), |sc| {
            sc.init(
                managed_address!(liquid.address_ref()),
                managed_biguint!(5_000),
                managed_biguint!(5_000),
                EgldOrEsdtTokenIdentifier::esdt(REWARD),
                managed_token_id!(REWARD),
                managed_address!(router.address_ref()),
            );
            sc.tokens().insert(EgldOrEsdtTokenIdentifier::esdt(TOKEN));
            sc.token_balance(&EgldOrEsdtTokenIdentifier::esdt(TOKEN))
                .set(managed_biguint!(100));
        })
        .assert_ok();
    let router_address = router.address_ref().clone();
    test(&mut world, &owner, &outsider, &accumulator, &router_address);
}

fn distribute_token(sc: AccumulatorObj) {
    sc.distribute(
        &EgldOrEsdtTokenIdentifier::esdt(TOKEN),
        ManagedArgBuffer::new(),
    );
}

#[test]
fn owner_can_distribute() {
    with_setup(false, |world, owner, _outsider, accumulator, _router| {
        world
            .execute_tx(owner, accumulator, &rust_biguint!(0), distribute_token)
            .assert_ok();
        // Exactly the recorded 100 left; revenue and royalties stay.
        world.check_esdt_balance(accumulator.address_ref(), TOKEN, &rust_biguint!(200));
    });
}

#[test]
fn non_distributor_cannot_distribute() {
    with_setup(true, |world, _owner, outsider, accumulator, _router| {
        world
            .execute_tx(outsider, accumulator, &rust_biguint!(0), distribute_token)
            .assert_user_error("Caller is not a distributor");
        world.check_esdt_balance(accumulator.address_ref(), TOKEN, &rust_biguint!(300));
    });
}

/// Even a trusted distributor route cannot spend the recorded balance twice:
/// the balance is cleared before the swap, so the re-entrant call finds nothing.
#[test]
fn reentrant_distribute_cannot_double_spend() {
    with_setup(true, |world, owner, _outsider, accumulator, router| {
        world
            .execute_tx(owner, accumulator, &rust_biguint!(0), |sc| {
                sc.add_distributor(managed_address!(router));
            })
            .assert_ok();
        world
            .execute_tx(owner, accumulator, &rust_biguint!(0), distribute_token)
            .assert_user_error("Token not found");
        world.check_esdt_balance(accumulator.address_ref(), TOKEN, &rust_biguint!(300));
    });
}
