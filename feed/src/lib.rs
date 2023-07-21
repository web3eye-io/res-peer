use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ApplicationId, ChainId, ContractAbi, Owner, ServiceAbi, Timestamp};
use serde::{Deserialize, Serialize};

pub struct FeedAbi;

impl ContractAbi for FeedAbi {
    type Parameters = ApplicationId<credit::CreditAbi>;
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for FeedAbi {
    type Parameters = ApplicationId<credit::CreditAbi>;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub author: Owner,
    pub title: String,
    pub content: String,
    pub likes: u64,
    pub dislikes: u64,
    pub accounts: HashMap<Owner, bool>,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub react_interval_ms: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Publish {
        cid: String,
        title: String,
        content: String,
    },
    Comment {
        comment_cid: String,
        content_cid: String,
    },
    Like {
        cid: String,
    },
    Dislike {
        cid: String,
    },
    Tip {
        cid: String,
        amount: Amount,
    },
    RequestSubscribe {
        chain_id: ChainId,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Publish {
        cid: String,
        title: String,
        content: String,
        author: Owner,
    },
    RequestSubscribe,
}
