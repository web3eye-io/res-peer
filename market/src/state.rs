use std::collections::HashMap;

use linera_sdk::{
    base::{Amount, ArithmeticError, Owner},
    contract::system_api,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use market::{Collection, InitialState, NFT};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Market {
    pub publisher_collections: MapView<Owner, Vec<u64>>,
    /// owner, collection_id, token_id
    pub assets: MapView<Owner, HashMap<u64, Vec<u16>>>,
    pub token_owners: MapView<u16, HashMap<u64, Owner>>,
    pub token_publishers: MapView<u16, HashMap<u64, Owner>>,
    pub credits_per_linera: RegisterView<Amount>,
    pub collection_id: RegisterView<u64>,
    /// Linera token balance
    /// If user want to buy asset here, they should deposit balance firstly, then buy
    /// They balance could be withdrawed
    pub token_ids: MapView<u64, u16>,
    pub collections: MapView<u64, Collection>,
    pub collection_uris: RegisterView<Vec<String>>,
    pub max_credits_percent: RegisterView<u8>,
    pub trade_fee_percent: RegisterView<u8>,
    pub avatars: MapView<Owner, Vec<u64>>,
}

#[allow(dead_code)]
impl Market {
    pub(crate) async fn initialize_market(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera);
        self.collection_id.set(state.collection_id.unwrap_or(1000));
        self.max_credits_percent.set(state.max_credits_percent);
        self.trade_fee_percent.set(state.trade_fee_percent);
    }

    pub(crate) async fn initial_state(&self) -> Result<InitialState, StateError> {
        Ok(InitialState {
            credits_per_linera: *self.credits_per_linera.get(),
            max_credits_percent: *self.max_credits_percent.get(),
            trade_fee_percent: *self.trade_fee_percent.get(),
            collection_id: Some(*self.collection_id.get()),
        })
    }

    pub(crate) async fn collections(&self, owner: Owner) -> Vec<u64> {
        self.publisher_collections
            .get(&owner)
            .await
            .unwrap()
            .unwrap()
    }

    pub(crate) async fn assets(&self, owner: Owner) -> HashMap<u64, Vec<u16>> {
        self.assets.get(&owner).await.unwrap().unwrap()
    }

    pub(crate) async fn create_collection(
        &mut self,
        owner: Owner,
        base_uri: String,
        price: Option<Amount>,
        name: String,
        uris: Vec<String>,
    ) -> Result<(), StateError> {
        if self.collection_uris.get().contains(&base_uri) {
            return Err(StateError::BaseURIALreadyExists);
        }
        let collection_id = *self.collection_id.get();
        let collection = Collection {
            collection_id,
            base_uri,
            uris,
            price,
            name,
            nfts: HashMap::new(),
            created_at: system_api::current_system_time(),
            publisher: owner,
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
        uri_index: u16,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), StateError> {
        self.validate_collection_owner(collection_id, owner).await?;
        match self.collections.get(&collection_id).await {
            Ok(Some(mut collection)) => {
                if uri_index >= collection.uris.len() as u16 {
                    return Err(StateError::InvalidUriIndex);
                }
                if collection.price.is_none() && price.is_none() {
                    return Err(StateError::InvalidPrice);
                }
                match self.token_ids.get(&collection_id).await {
                    Ok(Some(token_id)) => {
                        collection.nfts.insert(
                            token_id,
                            NFT {
                                token_id,
                                uri_index,
                                price,
                                on_sale: true,
                                minted_at: system_api::current_system_time(),
                                name,
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
                }
            }
            _ => return Err(StateError::CollectionNotExists),
        };
        Ok(())
    }

    pub(crate) async fn buy_nft(
        &mut self,
        buyer: Owner,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), StateError> {
        match self.collections.get(&collection_id).await {
            Ok(Some(collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => {
                    if !nft.on_sale {
                        return Err(StateError::TokenNotOnSale);
                    }
                    let token_owners = match self.token_owners.get(&token_id).await {
                        Ok(Some(owners)) => owners,
                        _ => HashMap::default(),
                    };
                    let owner = match token_owners.get(&collection_id) {
                        Some(owner) => *owner,
                        _ => return Err(StateError::CollectionNotExists),
                    };
                    if owner == buyer {
                        return Err(StateError::BuyerIsOwner);
                    }
                    let mut token_owners = token_owners.clone();
                    token_owners.insert(collection_id, buyer);
                    self.token_owners.insert(&token_id, token_owners)?;
                    match self.assets.get(&owner).await {
                        Ok(Some(collections)) => {
                            let collections = collections.clone();
                            match collections.get(&collection_id) {
                                Some(token_ids) => {
                                    let mut token_ids = token_ids.clone();
                                    token_ids.push(token_id);
                                    let mut collections = collections.clone();
                                    collections.insert(collection_id, token_ids);
                                    self.assets.insert(&owner, collections)?;
                                }
                                None => {
                                    let mut collections = collections.clone();
                                    collections.insert(collection_id, vec![token_id]);
                                    self.assets.insert(&owner, collections)?;
                                }
                            }
                        }
                        _ => {
                            let mut collections = HashMap::default();
                            collections.insert(collection_id, vec![token_id]);
                            self.assets.insert(&owner, collections)?;
                        }
                    }
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
        let token_owners = match self.token_owners.get(&token_id).await {
            Ok(Some(owners)) => owners,
            _ => HashMap::default(),
        };
        match token_owners.get(&collection_id) {
            Some(owner) => Ok(*owner),
            _ => Err(StateError::NotCollectionOwner),
        }
    }

    pub(crate) async fn nft_price(
        &self,
        collection_id: u64,
        token_id: u16,
    ) -> Result<Amount, StateError> {
        match self.collections.get(&collection_id).await {
            Ok(Some(collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => match nft.price {
                    Some(price) => Ok(price),
                    _ => match collection.price {
                        Some(price) => Ok(price),
                        _ => Err(StateError::InvalidPrice),
                    },
                },
                _ => Err(StateError::TokenIDNotExists),
            },
            _ => Err(StateError::CollectionNotExists),
        }
    }

    pub(crate) async fn update_nft_price(
        &mut self,
        owner: Owner,
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    ) -> Result<(), StateError> {
        match token_id {
            Some(token_id) => {
                if self.nft_owner(collection_id, token_id).await.unwrap() != owner {
                    return Err(StateError::NotTokenOwner);
                }
                match self.collections.get(&collection_id).await {
                    Ok(Some(mut collection)) => match collection.nfts.get(&token_id) {
                        Some(nft) => {
                            let mut _nft = nft.clone();
                            _nft.price = Some(price);
                            collection.nfts.insert(nft.token_id, _nft);
                            self.collections.insert(&collection_id, collection)?
                        }
                        _ => return Err(StateError::TokenIDNotExists),
                    },
                    _ => return Err(StateError::CollectionNotExists),
                }
            }
            _ => {
                if self
                    .validate_collection_owner(collection_id, owner)
                    .await
                    .is_err()
                {
                    return Err(StateError::NotCollectionOwner);
                }
                match self.collections.get(&collection_id).await {
                    Ok(Some(mut collection)) => {
                        collection.price = Some(price);
                        self.collections.insert(&collection_id, collection)?
                    }
                    _ => return Err(StateError::CollectionNotExists),
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn on_sale_nft(
        &mut self,
        owner: Owner,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), StateError> {
        if self.nft_owner(collection_id, token_id).await.unwrap() != owner {
            return Err(StateError::NotTokenOwner);
        }
        match self.collections.get(&collection_id).await {
            Ok(Some(mut collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => {
                    let mut _nft = nft.clone();
                    _nft.on_sale = true;
                    collection.nfts.insert(nft.token_id, _nft);
                    self.collections.insert(&collection_id, collection)?
                }
                _ => return Err(StateError::TokenIDNotExists),
            },
            _ => return Err(StateError::CollectionNotExists),
        }
        Ok(())
    }

    pub(crate) async fn off_sale_nft(
        &mut self,
        owner: Owner,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), StateError> {
        if self.nft_owner(collection_id, token_id).await.unwrap() != owner {
            return Err(StateError::NotTokenOwner);
        }
        match self.collections.get(&collection_id).await {
            Ok(Some(mut collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => {
                    let mut _nft = nft.clone();
                    _nft.on_sale = false;
                    collection.nfts.insert(nft.token_id, _nft);
                    self.collections.insert(&collection_id, collection)?
                }
                _ => return Err(StateError::TokenIDNotExists),
            },
            _ => return Err(StateError::CollectionNotExists),
        }
        Ok(())
    }

    pub(crate) async fn set_avatar(
        &mut self,
        owner: Owner,
        collection_id: u64,
        token_id: u16,
    ) -> Result<(), StateError> {
        match self.nft_owner(collection_id, token_id).await {
            Ok(_owner) => {
                if _owner != owner {
                    Err(StateError::NotTokenOwner)
                } else {
                    match self
                        .avatars
                        .insert(&owner, vec![collection_id, token_id as u64])
                    {
                        Ok(_) => Ok(()),
                        Err(err) => Err(StateError::ViewError(err)),
                    }
                }
            }
            _ => Err(StateError::NotTokenOwner),
        }
    }

    pub(crate) async fn trading_fee(&self, amount: Amount) -> Result<Amount, StateError> {
        Ok(Amount::from_atto(
            Amount::from_atto(*self.trade_fee_percent.get() as u128)
                .saturating_mul(amount.into())
                .saturating_div(Amount::from_atto(100 as u128)),
        ))
    }

    pub(crate) async fn credits_to_tokens(&self, credits: Amount) -> Result<Amount, StateError> {
        Ok(Amount::from_atto(
            credits
                .saturating_mul(Amount::ONE.into())
                .saturating_div(*self.credits_per_linera.get()),
        ))
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),

    #[error(transparent)]
    ArithmeticError(#[from] ArithmeticError),

    #[error("Owner is not collection owner")]
    NotCollectionOwner,

    #[error("Owner is not token owner")]
    NotTokenOwner,

    #[error("Base uri already exists")]
    BaseURIALreadyExists,

    #[error("Collection not exists")]
    CollectionNotExists,

    #[error("Token ID not exists")]
    TokenIDNotExists,

    #[error("NFT not on sale")]
    TokenNotOnSale,

    #[error("Invalid price")]
    InvalidPrice,

    #[error("Buyer is same as owner")]
    BuyerIsOwner,

    #[error("Invalid uri index")]
    InvalidUriIndex,
}
