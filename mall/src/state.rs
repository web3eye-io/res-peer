use std::collections::HashMap;

use linera_sdk::{
    base::{Amount, Owner},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use mall::{Collection, InitialState};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Mall {
    pub collections: MapView<Owner, Vec<Collection>>,
    /// owner, collection_id, token_id
    pub assets: MapView<Owner, HashMap<u64, Vec<u64>>>,
    pub credits_per_linera: RegisterView<Amount>,
    pub collection_id: RegisterView<u64>,
    /// Linera token balance
    pub balances: MapView<Owner, Amount>,
}

#[allow(dead_code)]
impl Mall {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera)
    }

    pub(crate) async fn collections(&self, owner: Owner) -> Vec<Collection> {
        self.collections.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn assets(&self, owner: Owner) -> HashMap<u64, Vec<u64>> {
        self.assets.get(&owner).await.unwrap().unwrap()
    }
}
