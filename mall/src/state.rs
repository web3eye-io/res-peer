use linera_sdk::{
    base::{Amount, Owner},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use mall::{InitialState, NFT};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Mall {
    pub collections: MapView<Owner, Vec<NFT>>,
    pub assets: MapView<Owner, Vec<NFT>>,
    pub credits_per_linera: RegisterView<Amount>,
}

#[allow(dead_code)]
impl Mall {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera)
    }

    pub(crate) async fn collections(&self, owner: Owner) -> Vec<NFT> {
        self.collections.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn assets(&self, owner: Owner) -> Vec<NFT> {
        self.assets.get(&owner).await.unwrap().unwrap()
    }
}
