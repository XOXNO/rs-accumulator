#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

use structs::CreatorRoyaltiesAmount;

pub mod config;
pub mod helpers;
pub mod liquid_proxy;
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
            let burn_rate = self.burn_rate().get();
            let share_rate = self.share_rate().get();
            let community_cut = burn_rate + share_rate;
            let community_share = self.calculate_split(&amount, &community_cut);
            let protoocl_share = amount - &community_share;

            let revenue_map = self.revenue().get(&token);
            let updated_revenue = match revenue_map {
                Some(revenue) => revenue + &protoocl_share,
                None => protoocl_share,
            };

            let map_token_balance = self.token_balance(&token);
            self.revenue().insert(token.clone(), updated_revenue);
            map_tokens.insert(token);
            map_token_balance.update(|qt| *qt += &community_share);
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

    #[only_owner]
    #[endpoint(addDistributor)]
    fn add_distributor(&self, address: ManagedAddress) {
        self.distributors().insert(address);
    }

    #[only_owner]
    #[endpoint(removeDistributor)]
    fn remove_distributor(&self, address: ManagedAddress) {
        self.distributors().swap_remove(&address);
    }

    #[endpoint]
    fn distribute(&self, token: &EgldOrEsdtTokenIdentifier, args: ManagedArgBuffer<Self::Api>) {
        self.require_distributor();
        let reward_token = self.reward_token().get();
        let mut map_tokens = self.tokens();

        require!(map_tokens.contains(token), "Token not found");

        let map_token_balance = self.token_balance(token);
        let amount = map_token_balance.get();
        require!(amount > BigUint::zero(), "No balance to distribute");

        // Clear before any external call so a re-entrant call cannot spend the same balance
        map_token_balance.clear();
        map_tokens.swap_remove(token);

        if token.is_esdt() && reward_token.eq(token) {
            self.forward_real_yield(&amount, &reward_token.unwrap_esdt());
        } else {
            let output = self.aggregate(token, &amount, args);
            require!(
                output.token_identifier.eq(&reward_token),
                "Invalid reward token"
            );
            self.forward_real_yield(&output.amount, &output.token_identifier.unwrap_esdt());
        }
    }

    #[endpoint(distributeRoyalties)]
    fn distribute_royalties(
        &self,
        token: &EgldOrEsdtTokenIdentifier,
        args: ManagedArgBuffer<Self::Api>,
        creators: MultiValueEncoded<ManagedAddress>,
    ) {
        self.require_distributor();
        let reward_token = self.reward_token().get();
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

            total_royalties += &amount;
            map_token_balance.clear();
            map_tokens.swap_remove(token);
            if map_tokens.is_empty() {
                map_creators.swap_remove(&creator);
            }
            creators_share.push(CreatorRoyaltiesAmount { creator, amount });
        }

        if total_royalties > 0 {
            // Royalties already in the reward token are staked as they are
            let reward_amount = if reward_token.ne(token) {
                let output = self.aggregate(token, &total_royalties, args);
                require!(
                    output.token_identifier.eq(&reward_token),
                    "Invalid reward token"
                );
                output.amount
            } else {
                total_royalties.clone()
            };

            let liquid_xoxno = self.delegate_xoxno(
                &self.xoxno_liquid_sc().get(),
                &reward_amount,
                &reward_token.unwrap_esdt(),
            );
            self.forward_shares(&creators_share, &total_royalties, &liquid_xoxno.amount)
        }
    }

    #[endpoint(claimProtocolReserves)]
    fn claim_protocol_reserves(&self) {
        let tokens = self.revenue();
        let owner = self.blockchain().get_owner_address();
        for (token, recorded) in tokens.iter() {
            // An underfunded entry is written off instead of blocking every other token
            let held = self.blockchain().get_sc_balance(&token, 0);
            let amount = if held >= recorded {
                recorded
            } else {
                self.unreserved_balance(&token, held)
            };
            self.tx()
                .to(&owner)
                .egld_or_single_esdt(&token, 0, &amount)
                .transfer_if_not_empty();
        }
        self.revenue().clear();
    }
}
