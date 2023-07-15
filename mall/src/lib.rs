use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct MallAbi;

impl ContractAbi for MallAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for MallAbi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct NFT {
    /// Sequence ID of NFT in collections
    pub token_id: u16,
    /// Storage location of http or ipfs
    pub uri: Option<String>,
    /// Price in Linera Token
    pub price: Option<Amount>,
    pub on_sale: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Collection {
    pub collection_id: u64,
    pub base_uri: String,
    pub nfts: HashMap<u16, NFT>,
    pub price: Option<Amount>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub credits_per_linera: Amount,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    OnSaleCollection {
        base_uri: String,
        price: Option<Amount>,
    },
    MintNFT {
        collection_id: u64,
        uri: Option<String>,
        price: Option<Amount>,
    },
    BuyNFT {
        collection_id: u64,
        token_id: u16,
        credits: Amount,
    },
    UpdateCreditsPerLinera {
        credits_per_linera: Amount,
    },
    UpdateNFTPrice {
        collection_id: u64,
        token_id: Option<u16>,
        price: Amount,
    },
    OnSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
    OffSaleNFT {
        collection_id: u64,
        token_id: u16,
    },
}
