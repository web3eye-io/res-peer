use async_graphql::SimpleObject;
use linera_sdk::base::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct FeedAbi;

impl ContractAbi for FeedAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = ();
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for FeedAbi {
    type Parameters = ();
    type Query = ();
    type QueryResponse = ();
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    cid: String,
    /// Here cid is the cid::Cid of the reply content store in ipfs
    reply_to_cid: String,
    like: u64,
    dislike: u64
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub react_interval_ms: u64
}
