multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getProtocolReserve)]
    #[storage_mapper("protocolReserve")]
    fn reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardToken)]
    #[storage_mapper("rewardToken")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLiquidRewardToken)]
    #[storage_mapper("liquidRewardToken")]
    fn liquid_reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getBurnRate)]
    #[storage_mapper("burnRate")]
    fn burn_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getShareRate)]
    #[storage_mapper("shareRate")]
    fn share_rate(&self) -> SingleValueMapper<BigUint>;

    #[view(getAccumulatedTokens)]
    #[storage_mapper("tokens")]
    fn tokens(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getAccumulatedTokenBalance)]
    #[storage_mapper("tokenBalance")]
    fn token_balance(&self, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getCreators)]
    #[storage_mapper("creators")]
    fn creators(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getCreatorTokens)]
    #[storage_mapper("creatorTokens")]
    fn creator_tokens(
        &self,
        creator: &ManagedAddress,
    ) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getCreatorRoyalties)]
    #[storage_mapper("creatorRoyalties")]
    fn creator_royalties(
        &self,
        creator: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[view(getLiquidSC)]
    #[storage_mapper("xoxnoLiquidSc")]
    fn xoxno_liquid_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getAggregatorSC)]
    #[storage_mapper("aggregatorSC")]
    fn ash_sc(&self) -> SingleValueMapper<ManagedAddress>;
}
