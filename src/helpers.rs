multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use crate::liquid_proxy::RsLiquidXoxnoProxy;
use crate::structs::{AggregatorStep, CreatorRoyaltiesAmount, TokenAmount};

#[multiversx_sc::module]
pub trait HelpersModule: crate::storage::StorageModule {

    fn forward_real_yield(&self, amount: &BigUint, reward_token: &TokenIdentifier) {
        let share_rate = self.share_rate().get();
        let burn_rate = self.burn_rate().get();
        let xoxno_liquid_sc = &self.xoxno_liquid_sc().get();

        let real_yield_before_burn = self.calculate_split(amount, &share_rate);
        let burn = self.calculate_split(&real_yield_before_burn, &burn_rate);
        let real_yield = &real_yield_before_burn - &burn;
        let protocol_revenue = amount - &real_yield_before_burn;

        self.burn(reward_token, &burn);
        self.tx()
            .to(xoxno_liquid_sc)
            .typed(RsLiquidXoxnoProxy)
            .add_rewards()
            .single_esdt(reward_token, 0, &real_yield)
            .transfer_execute();

        let liquid_received = self.delegate_xoxno(xoxno_liquid_sc, &protocol_revenue, reward_token);

        self.reserve().update(|qt| *qt += &liquid_received.amount);
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
            .delegate(OptionalValue::<ManagedAddress>::None)
            .single_esdt(token, 0, amount)
            .returns(ReturnsBackTransfers)
            .sync_call();

        let liquid_received = transfers.esdt_payments.get(0);
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

    fn forward_shares(
        &self,
        creators: &ManagedVec<CreatorRoyaltiesAmount<Self::Api>>,
        total_amount: &BigUint,
        total_shares_amount: &BigUint,
    ) {
        let liquid_identifier = self.liquid_reward_token().get();
        for creator in creators {
            let share = creator.amount.div(total_amount).mul(total_shares_amount);
            self.tx()
                .to(creator.creator)
                .single_esdt(&liquid_identifier, 0, &share)
                .transfer();
        }
    }

    #[proxy]
    fn dex_proxy(&self, sc_address: ManagedAddress) -> ash_proxy::Proxy<Self::Api>;

    fn aggregate(
        &self,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
        gas: u64,
        steps: ManagedVec<AggregatorStep<Self::Api>>,
        limits: ManagedVec<TokenAmount<Self::Api>>,
    ) -> EsdtTokenPayment<Self::Api> {
        let mut call = self.dex_proxy(self.ash_sc().get());

        if token.clone().is_esdt() {
            let (_, all_payments): (MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>>, _) = call
                .aggregate_esdt(steps, limits, false)
                .single_esdt(&token.clone().unwrap_esdt(), 0, amount)
                .gas(gas)
                .execute_on_dest_context_with_back_transfers();
            all_payments.esdt_payments.get(0)
        } else {
            let (_, all_payments): (ManagedVec<EsdtTokenPayment>, _) = call
                .aggregate_egld(steps, limits)
                .egld(amount)
                .gas(gas)
                .execute_on_dest_context_with_back_transfers();
            all_payments.esdt_payments.get(0)
        }
    }
}

mod ash_proxy {
    multiversx_sc::imports!();
    use crate::structs::*;

    #[multiversx_sc::proxy]
    pub trait AshContract {
        #[payable("*")]
        #[endpoint(aggregateEsdt)]
        fn aggregate_esdt(
            &self,
            steps: ManagedVec<AggregatorStep<Self::Api>>,
            limits: ManagedVec<TokenAmount<Self::Api>>,
            egld_return: bool,
        ) -> MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>>;

        #[payable("EGLD")]
        #[endpoint(aggregateEgld)]
        fn aggregate_egld(
            &self,
            steps: ManagedVec<AggregatorStep<Self::Api>>,
            limits: ManagedVec<TokenAmount<Self::Api>>,
        ) -> ManagedVec<EsdtTokenPayment>;
    }
}
