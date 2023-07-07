use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{ContractAbi, Owner, ServiceAbi};
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
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    cid: String,
    /// Here cid is the cid::Cid of the reply content store in ipfs
    reply_to_cid: String,
    likes: u64,
    dislikes: u64,
    accounts: HashMap<Owner, bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub react_interval_ms: u64,
}
