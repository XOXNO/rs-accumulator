multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::aggregator::AggregatorContractProxy;
use crate::aggregator::{AggregatorStep, TokenAmount};
use crate::config::WAD;
use crate::liquid_proxy::RsLiquidXoxnoProxy;
use crate::structs::CreatorRoyaltiesAmount;

#[multiversx_sc::module]
pub trait HelpersModule: crate::storage::StorageModule {
    fn forward_real_yield(&self, amount: &BigUint, reward_token: &TokenIdentifier) {
        let share_rate = self.share_rate().get();
        let burn_rate = self.burn_rate().get();
        let total_shares = &share_rate + &burn_rate;
        let xoxno_liquid_sc = &self.xoxno_liquid_sc().get();

        let real_yield_share = self.calculate_percentage_from(&share_rate, &total_shares);
        let real_yield = self.calculate_split(amount, &real_yield_share);
        let burn = amount - &real_yield;

        self.burn(reward_token, &burn);
        self.tx()
            .to(xoxno_liquid_sc)
            .typed(RsLiquidXoxnoProxy)
            .add_rewards()
            .single_esdt(reward_token, 0, &real_yield)
            .transfer_execute();
    }

    fn delegate_xoxno(
        &self,
        sc: &ManagedAddress,
        amount: &BigUint,
        token: &TokenIdentifier,
    ) -> EsdtTokenPayment {
        let liquid_token = self.liquid_reward_token().get();
        let transfers = self
            .tx()
            .to(sc)
            .typed(RsLiquidXoxnoProxy)
            .delegate(OptionalValue::Some(self.blockchain().get_sc_address()))
            .single_esdt(token, 0, amount)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let liquid_received = transfers.esdt_payments.get(0).clone();
        require!(
            liquid_received.token_identifier.eq(&liquid_token),
            "Wrong token received during delegation"
        );
        liquid_received
    }

    fn burn(&self, token: &TokenIdentifier, amount: &BigUint) {
        self.send().esdt_local_burn(token, 0, amount);
    }

    fn calculate_split(&self, total_amount: &BigUint, cut_percentage: &BigUint) -> BigUint {
        total_amount * cut_percentage / crate::config::PERCENTAGE_TOTAL
    }

    fn calculate_percentage_from(&self, original_cut: &BigUint, total_cut: &BigUint) -> BigUint {
        original_cut * crate::config::PERCENTAGE_TOTAL / total_cut
    }

    fn forward_shares(
        &self,
        creators: &ManagedVec<CreatorRoyaltiesAmount<Self::Api>>,
        total_amount: &BigUint,
        total_shares_amount: &BigUint,
    ) {
        let liquid_identifier = self.liquid_reward_token().get();
        let wad = BigUint::from(WAD);
        let mut track_dust = total_shares_amount.clone();
        let token_liquid = EgldOrEsdtTokenIdentifier::esdt(liquid_identifier.clone());
        let revenue_map = self.revenue().get(&token_liquid);
        let mut updated_revenue = revenue_map.unwrap_or(BigUint::zero());
        for creator in creators {
            let share = &creator.amount * &wad / total_amount * total_shares_amount / &wad;
            if track_dust >= share {
                track_dust -= &share;
            }

            let is_sc = self.blockchain().is_smart_contract(&creator.creator);
            if is_sc {
                let shard = self.blockchain().get_shard_of_address(&creator.creator);
                if shard == 1 {
                    let code = self.blockchain().get_code_metadata(&creator.creator);
                    if !code.is_payable_by_sc() && !code.is_payable() {
                        // Lost royalties
                        updated_revenue += &share;
                        continue;
                    }
                }
            }
            self.tx()
                .to(&creator.creator)
                .single_esdt(&liquid_identifier, 0, &share)
                .transfer_execute();
        }
        if track_dust > 0 {
            updated_revenue += &track_dust;
        }
        self.revenue().insert(token_liquid, updated_revenue);
    }

    fn aggregate(
        &self,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        gas: u64,
        steps: ManagedVec<AggregatorStep<Self::Api>>,
        limits: ManagedVec<TokenAmount<Self::Api>>,
    ) -> EsdtTokenPayment<Self::Api> {
        let call = self
            .tx()
            .to(self.ash_sc().get())
            .typed(AggregatorContractProxy);

        if token.clone().is_esdt() {
            let result = call
                .aggregate_esdt(steps, limits, false, OptionalValue::<ManagedAddress>::None)
                .single_esdt(&token.clone().unwrap_esdt(), 0, amount)
                .gas(gas)
                .returns(ReturnsBackTransfersSingleESDT)
                .sync_call();

            result
        } else {
            let result = call
                .aggregate_egld(steps, limits, OptionalValue::<ManagedAddress>::None)
                .egld(amount)
                .gas(gas)
                .returns(ReturnsBackTransfersSingleESDT)
                .sync_call();

            result
        }
    }
}
