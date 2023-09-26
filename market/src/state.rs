use std::{cmp::Ordering, collections::HashMap};

use linera_sdk::{
    base::{Amount, Owner},
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
    pub balances: MapView<Owner, Amount>,
    pub token_ids: MapView<u64, u16>,
    pub collections: MapView<u64, Collection>,
    pub collection_uris: RegisterView<Vec<String>>,
    pub max_credits_percent: RegisterView<u8>,
    pub trade_fee_percent: RegisterView<u8>,
    pub avatars: MapView<Owner, Vec<u64>>,
}

#[allow(dead_code)]
impl Market {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.credits_per_linera.set(state.credits_per_linera);
        self.collection_id.set(1000);
        self.max_credits_percent.set(state.max_credits_percent);
        self.trade_fee_percent.set(state.trade_fee_percent);
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
    ) -> Result<(), StateError> {
        if self.collection_uris.get().contains(&base_uri) {
            return Err(StateError::BaseURIALreadyExists);
        }
        let collection_id = *self.collection_id.get();
        let collection = Collection {
            collection_id,
            base_uri,
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
        uri: Option<String>,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), StateError> {
        match self.collections.get(&collection_id).await {
            Ok(Some(mut collection)) => {
                if collection.price.is_none() && price.is_none() {
                    return Err(StateError::InvalidPrice);
                }
                match self.token_ids.get(&collection_id).await {
                    Ok(Some(token_id)) => {
                        collection.nfts.insert(
                            token_id,
                            NFT {
                                token_id,
                                uri,
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
        credits: Amount,
    ) -> Result<(), StateError> {
        let credits = Amount::from_tokens(credits.into());
        match self.collections.get(&collection_id).await {
            Ok(Some(collection)) => match collection.nfts.get(&token_id) {
                Some(nft) => {
                    if !nft.on_sale {
                        return Err(StateError::TokenNotOnSale);
                    }
                    let buyer_balance = match self.balances.get(&buyer).await {
                        Ok(Some(balance)) => balance,
                        _ => Amount::ZERO,
                    };
                    let mut price = if nft.price.is_some() {
                        nft.price.unwrap()
                    } else if collection.price.is_some() {
                        collection.price.unwrap()
                    } else {
                        return Err(StateError::InvalidPrice);
                    };
                    let discount_amount = if self.credits_per_linera.get().ge(&Amount::ZERO) {
                        let d1 = credits.saturating_div(*self.credits_per_linera.get());
                        let d2 = price
                            .saturating_mul(*self.max_credits_percent.get() as u128)
                            .saturating_div(Amount::from_atto(100));
                        Amount::from_atto(match d1.cmp(&d2) {
                            Ordering::Less => d1,
                            _ => d2,
                        })
                    } else {
                        Amount::ZERO
                    };
                    price = price.saturating_sub(discount_amount);
                    if price.gt(&buyer_balance) {
                        return Err(StateError::InsufficientBalance);
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
                        log::info!("TODO: buyer could not be the same as owner");
                        // return Err(StateError::BuyerIsOwner);
                    }
                    let owner_balance = match self.balances.get(&owner).await {
                        Ok(Some(balance)) => balance,
                        _ => Amount::ZERO,
                    };
                    self.balances
                        .insert(&owner, owner_balance.saturating_add(price))?;
                    self.balances
                        .insert(&buyer, buyer_balance.saturating_sub(price))?;
                    log::info!(
                        "{} buy {}-{} from {} with {} Lineras and {} Credits {} discount",
                        buyer,
                        collection_id,
                        token_id,
                        owner,
                        price,
                        credits,
                        discount_amount
                    );
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

    pub(crate) async fn deposit(&mut self, owner: Owner, amount: Amount) -> Result<(), StateError> {
        match self.balances.get(&owner).await {
            Ok(Some(balance)) => match self.balances.insert(&owner, balance.saturating_add(amount))
            {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
            _ => match self.balances.insert(&owner, amount) {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
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
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),

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

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid price")]
    InvalidPrice,
    // #[error("Buyer is same as owner")]
    // BuyerIsOwner,
}
