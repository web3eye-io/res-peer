use std::collections::HashMap;

use linera_sdk::{
    base::{Amount, Owner},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use mall::{Collection, InitialState, NFT};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Mall {
    pub collections: MapView<Owner, HashMap<u64, Collection>>,
    /// owner, collection_id, token_id
    pub assets: MapView<Owner, HashMap<u64, Vec<u64>>>,
    pub credits_per_linera: RegisterView<Amount>,
    pub collection_id: RegisterView<u64>,
    /// Linera token balance
    pub balances: MapView<Owner, Amount>,
    pub token_ids: MapView<u64, u16>,
}

#[allow(dead_code)]
impl Mall {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera);
        self.collection_id.set(1000);
    }

    pub(crate) async fn collections(&self, owner: Owner) -> HashMap<u64, Collection> {
        self.collections.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn assets(&self, owner: Owner) -> HashMap<u64, Vec<u64>> {
        self.assets.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn create_collection(
        &mut self,
        owner: Owner,
        base_uri: String,
        price: Option<Amount>,
    ) {
        let collection = Collection {
            collection_id: *self.collection_id.get(),
            base_uri,
            price,
            nfts: HashMap::new(),
        };
        match self.collections.get(&owner).await {
            Ok(Some(mut collections)) => {
                collections.insert(collection.collection_id, collection.clone());
                self.collections.insert(&owner, collections).unwrap();
            }
            _ => {
                let mut collections = HashMap::new();
                collections.insert(collection.collection_id, collection.clone());
                self.collections.insert(&owner, collections).unwrap();
            }
        }
        self.token_ids
            .insert(&collection.collection_id, 1000)
            .unwrap();
        self.collection_id.set(self.collection_id.get() + 1);
    }

    pub(crate) async fn validate_collection_owner(
        &self,
        collection_id: u64,
        owner: Owner,
    ) -> Result<(), StateError> {
        match self.collections.get(&owner).await {
            Ok(Some(collections)) => match collections.get(&collection_id) {
                Some(_) => Ok(()),
                _ => Err(StateError::NotCollectionOwner),
            },
            _ => Err(StateError::NotCollectionOwner),
        }
    }

    pub(crate) async fn mint_nft(
        &mut self,
        owner: Owner,
        collection_id: u64,
        uri: Option<String>,
        price: Option<Amount>,
    ) {
        let mut collections = self.collections.get(&owner).await.unwrap().unwrap();
        let mut collection = collections.get(&collection_id).unwrap().clone();
        let token_id = self.token_ids.get(&collection_id).await.unwrap().unwrap();
        let nft = NFT {
            token_id,
            uri,
            price,
            on_sale: true,
        };
        collection.nfts.insert(token_id, nft).unwrap();
        collections.insert(collection.collection_id, collection);
        self.token_ids.insert(&collection_id, token_id + 1).unwrap();
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Owner is not collection owner")]
    NotCollectionOwner,
}
