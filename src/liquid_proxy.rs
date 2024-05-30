// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct RsLiquidXoxnoProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for RsLiquidXoxnoProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = RsLiquidXoxnoProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        RsLiquidXoxnoProxyMethods { wrapped_tx: tx }
    }
}

pub struct RsLiquidXoxnoProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> RsLiquidXoxnoProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        main_token: Arg0,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&main_token)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> RsLiquidXoxnoProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> RsLiquidXoxnoProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn delegate<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("delegate")
            .argument(&delegator)
            .original_result()
    }

    pub fn un_delegate(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("unDelegate")
            .original_result()
    }

    pub fn withdraw(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("withdraw")
            .original_result()
    }

    pub fn add_rewards(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("addRewards")
            .original_result()
    }

    pub fn get_ls_value_for_position<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        ls_token_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMainTokenAmountForPosition")
            .argument(&ls_token_amount)
            .original_result()
    }

    pub fn get_ls_amount_for_position<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        main_token_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsTokenAmountForMainTokenAmount")
            .argument(&main_token_amount)
            .original_result()
    }

    pub fn register_ls_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<usize>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        num_decimals: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("registerLsToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn register_unstake_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<usize>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        num_decimals: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("registerUnstakeToken")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&num_decimals)
            .original_result()
    }

    pub fn set_state_active(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setStateActive")
            .original_result()
    }

    pub fn set_state_inactive(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setStateInactive")
            .original_result()
    }

    pub fn state(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, State> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getState")
            .original_result()
    }

    pub fn ls_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsTokenId")
            .original_result()
    }

    pub fn main_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMainToken")
            .original_result()
    }

    pub fn ls_token_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLsSupply")
            .original_result()
    }

    pub fn virtual_xoxno_reserve(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getVirtualXOXNOReserve")
            .original_result()
    }

    pub fn total_withdrawn_xoxno(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalWithdrawnXOXNO")
            .original_result()
    }

    pub fn unstake_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUnstakeTokenId")
            .original_result()
    }

    pub fn unstake_token_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUnstakeTokenSupply")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum State {
    Inactive,
    Active,
}

#[type_abi]
#[derive(TopEncode)]
pub struct AddLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub ls_token_id: TokenIdentifier<Api>,
    pub ls_token_amount: BigUint<Api>,
    pub ls_token_supply: BigUint<Api>,
    pub virtual_xoxno_reserve: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[type_abi]
#[derive(TopEncode)]
pub struct RemoveLiquidityEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub ls_token_id: TokenIdentifier<Api>,
    pub ls_token_amount: BigUint<Api>,
    pub unstake_token_id: TokenIdentifier<Api>,
    pub unstake_token_amount: BigUint<Api>,
    pub ls_token_supply: BigUint<Api>,
    pub virtual_xoxno_reserve: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}

#[type_abi]
#[derive(TopEncode)]
pub struct AddRewardsEvent<Api>
where
    Api: ManagedTypeApi,
{
    pub caller: ManagedAddress<Api>,
    pub ls_token_id: TokenIdentifier<Api>,
    pub ls_token_supply: BigUint<Api>,
    pub virtual_xoxno_reserve: BigUint<Api>,
    pub rewards_amount: BigUint<Api>,
    pub block: u64,
    pub epoch: u64,
    pub timestamp: u64,
}
