multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::structs::CreatorRoyalties;

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
    fn get_all_creator_royalties(&self) -> MultiValueEncoded<CreatorRoyalties<Self::Api>> {
        let mut summary = MultiValueEncoded::new();
        let creators = self.creators();
        for creator in creators.iter() {
            let mut royalties = ManagedVec::new();
            let tokens = self.creator_tokens(&creator);
            for token in tokens.iter() {
                let amount = self.creator_royalties(&creator, &token).get();
                royalties.push(EsdtTokenPayment::new(token.into_name().into(), 0, amount));
            }
            summary.push(CreatorRoyalties {
                creator: creator,
                tokens: royalties,
            });
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
