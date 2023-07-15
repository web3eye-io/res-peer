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
    pub publisher_collections: MapView<Owner, Vec<u64>>,
    /// owner, collection_id, token_id
    pub assets: MapView<Owner, HashMap<u64, Vec<u64>>>,
    pub token_owners: MapView<u16, HashMap<u64, Owner>>,
    pub token_publishers: MapView<u16, HashMap<u64, Owner>>,
    pub credits_per_linera: RegisterView<Amount>,
    pub collection_id: RegisterView<u64>,
    /// Linera token balance
    pub balances: MapView<Owner, Amount>,
    pub token_ids: MapView<u64, u16>,
    pub collections: MapView<u64, Collection>,
    pub collection_uris: RegisterView<Vec<String>>,
}

#[allow(dead_code)]
impl Mall {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera);
        self.collection_id.set(1000);
    }

    pub(crate) async fn collections(&self, owner: Owner) -> Vec<u64> {
        self.publisher_collections
            .get(&owner)
            .await
            .unwrap()
            .unwrap()
    }

    pub(crate) async fn assets(&self, owner: Owner) -> HashMap<u64, Vec<u64>> {
        self.assets.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn create_collection(
        &mut self,
        owner: Owner,
        base_uri: String,
        price: Option<Amount>,
    ) -> Result<(), StateError> {
        if self.collection_uris.get().contains(&base_uri) {
            return Err(StateError::BaseURIALreadyExists);
        }
        let collection_id = *self.collection_id.get();
        let collection = Collection {
            collection_id,
            base_uri,
            price,
            nfts: HashMap::new(),
        };
        match self.publisher_collections.get(&owner).await {
            Ok(Some(mut collections)) => {
                collections.push(collection.collection_id);
                self.publisher_collections
                    .insert(&owner, collections)
                    .unwrap();
            }
            _ => {
                self.publisher_collections
                    .insert(&owner, vec![collection.collection_id])
                    .unwrap();
            }
        }
        self.collections.insert(&collection_id, collection).unwrap();
        self.token_ids.insert(&collection_id, 1000).unwrap();
        self.collection_id.set(self.collection_id.get() + 1);
        Ok(())
    }

    pub(crate) async fn validate_collection_owner(
        &self,
        collection_id: u64,
        owner: Owner,
    ) -> Result<(), StateError> {
        match self.publisher_collections.get(&owner).await {
            Ok(Some(collections)) => match collections.contains(&collection_id) {
                true => Ok(()),
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
    ) -> Result<(), StateError> {
        match self.collections.get(&collection_id).await {
            Ok(Some(mut collection)) => match self.token_ids.get(&collection_id).await {
                Ok(Some(token_id)) => {
                    collection.nfts.insert(
                        token_id,
                        NFT {
                            token_id,
                            uri,
                            price,
                            on_sale: true,
                        },
                    );
                    self.collections.insert(&collection_id, collection)?;
                    self.token_ids.insert(&collection_id, token_id + 1)?;
                    match self.token_owners.get(&token_id).await {
                        Ok(Some(mut collection_owners)) => {
                            collection_owners.insert(collection_id, owner);
                            self.token_owners.insert(&token_id, collection_owners)?;
                        }
                        _ => {
                            let mut collection_owners = HashMap::new();
                            collection_owners.insert(collection_id, owner);
                            self.token_owners.insert(&token_id, collection_owners)?;
                        }
                    }
                    match self.token_publishers.get(&token_id).await {
                        Ok(Some(mut collection_publisher)) => {
                            collection_publisher.insert(collection_id, owner);
                            self.token_publishers
                                .insert(&token_id, collection_publisher)?;
                        }
                        _ => {
                            let mut collection_publisher = HashMap::new();
                            collection_publisher.insert(collection_id, owner);
                            self.token_publishers
                                .insert(&token_id, collection_publisher)?;
                        }
                    }
                }
                _ => return Err(StateError::TokenIDNotExists),
            },
            _ => return Err(StateError::CollectionNotExists),
        };
        Ok(())
    }

    pub(crate) async fn buy_nft(
        &mut self,
        buyer: Owner,
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    ) -> Result<(), StateError> {
        match self.collections.get(&collection_id).await {
            Ok(Some(collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => {
                    if !nft.on_sale {
                        return Err(StateError::TokenNotOnSale);
                    }
                    let buyer_balance = self.balances.get(&buyer).await.unwrap().unwrap();
                    let mut discount_amount = Amount::zero();
                    if self.credits_per_linera.get().ge(&Amount::zero()) {
                        discount_amount = credits
                            .saturating_div(*self.credits_per_linera.get())
                            .into();
                    }
                    let mut price = Amount::zero();
                    if nft.price.is_some() {
                        price = nft.price.unwrap();
                    } else if collection.price.is_some() {
                        price = collection.price.unwrap();
                    }
                    price = price.saturating_sub(discount_amount);
                    if price.gt(&buyer_balance) {
                        return Err(StateError::InsufficientBalance);
                    }
                    let mut token_owners = self.token_owners.get(&token_id).await.unwrap().unwrap();
                    let owner = token_owners.get(&collection_id).unwrap();
                    let owner_balance = self.balances.get(owner).await.unwrap().unwrap();
                    self.balances
                        .insert(owner, owner_balance.saturating_add(price))?;
                    self.balances
                        .insert(&buyer, buyer_balance.saturating_sub(price))?;
                    token_owners.insert(collection_id, buyer);
                    self.token_owners.insert(&token_id, token_owners)?;
                }
                _ => return Err(StateError::TokenIDNotExists),
            },
            _ => return Err(StateError::CollectionNotExists),
        }
        Ok(())
    }

    pub(crate) async fn nft_owner(
        &self,
        collection_id: u64,
        token_id: u16,
    ) -> Result<Owner, StateError> {
        let token_owners = self.token_owners.get(&token_id).await.unwrap().unwrap();
        let owner = token_owners.get(&collection_id).unwrap();
        Ok(*owner)
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Owner is not collection owner")]
    NotCollectionOwner,

    #[error("Base uri already exists")]
    BaseURIALreadyExists,

    #[error("Collection not exists")]
    CollectionNotExists,

    #[error("Token ID not exists")]
    TokenIDNotExists,

    #[error("NFT not on sale")]
    TokenNotOnSale,

    #[error("Insufficient balance")]
    InsufficientBalance,
}
