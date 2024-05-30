#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

use structs::{AggregatorStep, TokenAmount};

pub mod config;
pub mod helpers;
pub mod liquid_proxy;
pub mod manager;
pub mod query;
pub mod storage;
pub mod structs;

#[multiversx_sc::contract]
pub trait Accumulator: crate::storage::StorageModule + crate::helpers::HelpersModule {
    #[init]
    fn init(
        &self,
        xoxno_liquid_sc: ManagedAddress,
        burn_rate: BigUint,
        share_rate: BigUint,
        reward_token: TokenIdentifier,
        liquid_reward_token: TokenIdentifier,
        ash_sc: ManagedAddress,
    ) {
        self.xoxno_liquid_sc().set(xoxno_liquid_sc);
        self.burn_rate().set(burn_rate);
        self.share_rate().set(share_rate);
        self.reward_token().set(reward_token);
        self.liquid_reward_token().set(liquid_reward_token);
        self.ash_sc().set(ash_sc);
    }

    #[upgrade]
    fn upgrade(
        &self,
        xoxno_liquid_sc: ManagedAddress,
        burn_rate: BigUint,
        share_rate: BigUint,
        reward_token: TokenIdentifier,
        liquid_reward_token: TokenIdentifier,
        ash_sc: ManagedAddress,
    ) {
        self.xoxno_liquid_sc().set(xoxno_liquid_sc);
        self.burn_rate().set(burn_rate);
        self.share_rate().set(share_rate);
        self.reward_token().set(reward_token);
        self.liquid_reward_token().set(liquid_reward_token);
        self.ash_sc().set(ash_sc);
    }

    #[payable("*")]
    #[endpoint]
    fn deposit(&self) {
        let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
        if amount > 0 {
            let mut map_tokens = self.tokens();

            let map_token_balance = self.token_balance(&token);
            map_tokens.insert(token);
            map_token_balance.update(|qt| *qt += &amount);
        }
    }

    #[payable("*")]
    #[endpoint(depositRoyalties)]
    fn deposit_royalties(&self, creator: ManagedAddress) {
        let (token, amount) = self.call_value().egld_or_single_fungible_esdt();
        if amount > 0 {
            let map_token_balance = self.creator_royalties(&creator, &token);

            map_token_balance.update(|qt| *qt += &amount);
            self.creator_tokens(&creator).insert(token);
            self.creators().insert(creator.clone());
        }
    }

    #[endpoint]
    fn distribute(
        &self,
        token: &EgldOrEsdtTokenIdentifier,
        gas: u64,
        steps: ManagedVec<AggregatorStep<Self::Api>>,
        limits: ManagedVec<TokenAmount<Self::Api>>,
    ) {
        let reward_token = self.reward_token().get();
        let mut map_tokens = self.tokens();
        require!(map_tokens.contains(token), "Token not found");

        let map_token_balance = self.token_balance(token);
        let amount = map_token_balance.get();
        require!(amount > BigUint::zero(), "No balance to distribute");

        if token.is_esdt() && reward_token.eq(&token.clone().unwrap_esdt()) {
            self.forward_real_yield(&amount, &reward_token);
            map_token_balance.clear();
            map_tokens.swap_remove(token);
            return;
        }

        let output = self.aggregate(token, amount, gas, steps, limits);
        require!(
            output.token_identifier.eq(&reward_token),
            "Invalid reward token"
        );
        map_token_balance.clear();
        map_tokens.swap_remove(token);
        self.forward_real_yield(&output.amount, &output.token_identifier);
    }

    #[endpoint(distributeRoyalties)]
    fn distribute_royalties(
        &self,
        token: &EgldOrEsdtTokenIdentifier,
        creator: &ManagedAddress,
        gas: u64,
        steps: ManagedVec<AggregatorStep<Self::Api>>,
        limits: ManagedVec<TokenAmount<Self::Api>>,
    ) {
        let reward_token = self.reward_token().get();
        let mut map_tokens = self.creator_tokens(creator);
        require!(map_tokens.contains(token), "Token not found");

        let map_token_balance = self.creator_royalties(creator, token);
        let amount = map_token_balance.get();
        require!(amount > BigUint::zero(), "No royalties to distribute");

        if token.is_esdt() && reward_token.eq(&token.clone().unwrap_esdt()) {
            self.forward_royalties(creator, &amount, &reward_token);
            map_token_balance.clear();
            map_tokens.swap_remove(token);
            return;
        }

        let output = self.aggregate(token, amount, gas, steps, limits);
        require!(
            output.token_identifier.eq(&reward_token),
            "Invalid reward token"
        );
        map_token_balance.clear();
        map_tokens.swap_remove(token);
        self.forward_royalties(creator, &output.amount, &output.token_identifier);
    }

    #[endpoint(claimProtocolReserves)]
    fn claim_protocol_reserves(&self) {
        let revenue_map = self.reserve();
        let revenue = revenue_map.get();
        if revenue > 0 {
            let token = self.liquid_reward_token().get();
            self.tx()
                .to(self.blockchain().get_owner_address())
                .esdt((token, 0, revenue))
                .transfer();
            revenue_map.clear();
        }
    }
}
