#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

use structs::CreatorRoyaltiesAmount;
use aggregator::{TokenAmount, AggregatorStep};

pub mod config;
pub mod helpers;
pub mod liquid_proxy;
pub mod aggregator;
pub mod manager;
pub mod query;
pub mod storage;
pub mod structs;

#[multiversx_sc::contract]
pub trait Accumulator:
    crate::storage::StorageModule + crate::helpers::HelpersModule + crate::query::QueryModule
{
    #[init]
    fn init(
        &self,
        xoxno_liquid_sc: ManagedAddress,
        burn_rate: BigUint,
        share_rate: BigUint,
        reward_token: EgldOrEsdtTokenIdentifier,
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
        reward_token: EgldOrEsdtTokenIdentifier,
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

        if token.is_esdt() && reward_token.eq(token) {
            self.forward_real_yield(&amount, &reward_token.unwrap_esdt());
            map_token_balance.clear();
            map_tokens.swap_remove(token);
            return;
        }

        let output = self.aggregate(token, &amount, gas, steps, limits);
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
        gas: u64,
        steps: ManagedVec<AggregatorStep<Self::Api>>,
        limits: ManagedVec<TokenAmount<Self::Api>>,
        creators: MultiValueEncoded<ManagedAddress>,
    ) {
        let reward_token = self.reward_token().get();
        let mut total_rewards_amount = BigUint::zero();
        let mut total_royalties = BigUint::zero();
        let mut creators_share: ManagedVec<CreatorRoyaltiesAmount<Self::Api>> = ManagedVec::new();
        let mut map_creators = self.creators();
        for creator in creators {
            let mut map_tokens = self.creator_tokens(&creator);
            if !map_tokens.contains(token) {
                continue;
            }

            let map_token_balance = self.creator_royalties(&creator, token);
            let amount = map_token_balance.get();
            if amount == BigUint::zero() {
                continue;
            }

            total_rewards_amount += &amount;
            map_token_balance.clear();
            map_tokens.swap_remove(token);
            if map_tokens.len() == 0 {
                map_creators.swap_remove(&creator);
            }
            creators_share.push(CreatorRoyaltiesAmount { creator, amount });
        }

        if total_rewards_amount > 0 {
            if reward_token.ne(token) {
                let output = self.aggregate(token, &total_rewards_amount, gas, steps, limits);
                total_royalties = total_rewards_amount;
                total_rewards_amount = output.amount;
                require!(
                    output.token_identifier.eq(&reward_token),
                    "Invalid reward token"
                );
            }

            let liquid_xoxno = self.delegate_xoxno(
                &self.xoxno_liquid_sc().get(),
                &total_rewards_amount,
                &reward_token.unwrap_esdt(),
            );
            self.forward_shares(&creators_share, &total_royalties, &liquid_xoxno.amount)
        }
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
