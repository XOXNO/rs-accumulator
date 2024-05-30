multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait QueryModule: crate::storage::StorageModule {
    #[view(queryCreatorRoyalties)]
    fn get_creator_royalties(
        &self,
        creator: &ManagedAddress,
    ) -> MultiValueEncoded<EgldOrEsdtTokenPayment> {
        let mut royalties = MultiValueEncoded::new();
        let tokens = self.creator_tokens(creator);
        for token in &tokens {
            let amount = self.creator_royalties(creator, &token).get();
            royalties.push(EgldOrEsdtTokenPayment::new(token, 0, amount))
        }
        return royalties;
    }

    #[view(queryAllCreatorRoyalties)]
    fn get_all_creator_royalties(
        &self,
    ) -> MultiValueEncoded<MultiValue2<ManagedAddress, MultiValueEncoded<EgldOrEsdtTokenPayment>>>
    {
        let mut summary = MultiValueEncoded::new();
        let creators = self.creators();
        for creator in &creators {
            let mut royalties = MultiValueEncoded::new();
            let tokens = self.creator_tokens(&creator);
            for token in &tokens {
                let amount = self.creator_royalties(&creator, &token).get();
                royalties.push(EgldOrEsdtTokenPayment::new(token, 0, amount));
            }
            summary.push(MultiValue2((creator, royalties)));
        }
        return summary;
    }

    #[view(queryRealYieldPending)]
    fn get_real_yield_pending(&self) -> MultiValueEncoded<EgldOrEsdtTokenPayment> {
        let mut yield_pending = MultiValueEncoded::new();
        let tokens = self.tokens();
        for token in &tokens {
            let amount = self.token_balance(&token).get();
            yield_pending.push(EgldOrEsdtTokenPayment::new(token, 0, amount))
        }
        return yield_pending;
    }
}
