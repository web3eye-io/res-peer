use async_graphql::SimpleObject;
use linera_sdk::base::{ContractAbi, ServiceAbi, Amount};
use serde::{Deserialize, Serialize};

pub struct MallAbi;

impl ContractAbi for MallAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = ();
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for MallAbi {
    type Parameters = ();
    type Query = ();
    type QueryResponse = ();
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct NFT {
    /// Storage location of http or ipfs
    pub uri: String,
    /// Price in Linera Token
    pub price: Amount
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub credits_per_linera: Amount
}
