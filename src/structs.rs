use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, ManagedVecItem)]
pub struct CreatorRoyalties<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub tokens: ManagedVec<M, EsdtTokenPayment<M>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, ManagedVecItem)]
pub struct CreatorRoyaltiesAmount<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,
    pub amount: BigUint<M>,
}
