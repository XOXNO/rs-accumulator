multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ManagerModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(changeBurnRate)]
    fn change_burn_rate(&self, burn_rate: BigUint) {
        self.burn_rate().set(burn_rate);
    }

    #[only_owner]
    #[endpoint(changeShareRate)]
    fn change_share_rate(&self, share_rate: BigUint) {
        require!(
            share_rate < BigUint::from(crate::config::PERCENTAGE_TOTAL),
            "Invalid share rate"
        );
        self.share_rate().set(share_rate);
    }

    #[only_owner]
    #[endpoint(changeRewardToken)]
    fn change_reward_token(&self, reward_token: EgldOrEsdtTokenIdentifier) {
        self.reward_token().set(reward_token);
    }

    #[only_owner]
    #[endpoint(changeLiquidSC)]
    fn change_liquid_xoxno(&self, sc: ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(&sc),
            "Invalid contract address"
        );
        self.xoxno_liquid_sc().set(sc);
    }
}
