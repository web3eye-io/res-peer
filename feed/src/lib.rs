use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ApplicationId, ContractAbi, Owner, ServiceAbi, Timestamp};
use serde::{Deserialize, Serialize};

pub struct FeedAbi;

impl ContractAbi for FeedAbi {
    type Parameters = FeedParameters;
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ApplicationCall;
    type SessionCall = ();
    type SessionState = ();
    type Response = Option<Owner>;
}

impl ServiceAbi for FeedAbi {
    type Parameters = FeedParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedParameters {
    pub credit_app_id: ApplicationId<credit::CreditAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub comment_to_cid: Option<String>,
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
    Like { cid: String },
    Dislike { cid: String },
    Tip { cid: String, amount: Amount },
    RequestSubscribe,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
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
    Publish {
        cid: String,
        title: String,
        content: String,
        author: Owner,
    },
    Recommend {
        cid: String,
        reason_cid: String,
        reason: String,
    },
    Comment {
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    },
    RequestSubscribe,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationCall {
    Publish {
        cid: String,
        title: String,
        content: String,
        author: Owner,
    },
    Recommend {
        cid: String,
        reason_cid: String,
        reason: String,
    },
    Comment {
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    },
    ContentAuthor {
        cid: String,
    },
}
