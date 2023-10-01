use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{ApplicationId, ContractAbi, Owner, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct ReviewAbi;

impl ContractAbi for ReviewAbi {
    type Parameters = ApplicationId<feed::FeedAbi>;
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for ReviewAbi {
    type Parameters = ApplicationId<feed::FeedAbi>;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub content_approved_threshold: u16,
    pub content_rejected_threshold: u16,
    pub asset_approved_threshold: u16,
    pub asset_rejected_threshold: u16,
    pub reviewer_approved_threshold: u16,
    pub reviewer_rejected_threshold: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Review {
    pub reviewer: Owner,
    pub approved: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Reviewer {
    pub reviewer: Owner,
    pub resume: Option<String>,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub author: Owner,
    pub title: String,
    pub content: String,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Asset {
    pub collection_id: u64,
    pub author: Owner,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    ApplyReviewer {
        resume: String,
    },
    UpdateReviewerResume {
        resume: String,
    },
    ApproveReviewer {
        candidate: Owner,
    },
    RejectReviewer {
        candidate: Owner,
    },
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    ApproveContent {
        content_cid: String,
        reason: Option<String>,
    },
    RejectContent {
        content_cid: String,
        reason: Option<String>,
    },
    ApproveAsset {
        collection_id: u64,
        reason: Option<String>,
    },
    RejectAsset {
        collection_id: u64,
        reason: Option<String>,
    },
    RequestSubmittedSubscribe,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    RequestSubmittedSubscribe,
}
