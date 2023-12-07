use std::collections::HashMap;

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ApplicationId, ChainId, ContractAbi, Owner, ServiceAbi, Timestamp};
use serde::{Deserialize, Serialize};

pub struct ReviewAbi;

impl ContractAbi for ReviewAbi {
    type Parameters = ReviewParameters;
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ApplicationCall;
    type SessionCall = ();
    type SessionState = ();
    type Response = bool;
}

impl ServiceAbi for ReviewAbi {
    type Parameters = ReviewParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReviewParameters {
    pub feed_app_id: ApplicationId<feed::FeedAbi>,
    pub credit_app_id: ApplicationId<credit::CreditAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
    pub market_app_id: ApplicationId<market::MarketAbi>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub content_approved_threshold: u16,
    pub content_rejected_threshold: u16,
    pub asset_approved_threshold: u16,
    pub asset_rejected_threshold: u16,
    pub reviewer_approved_threshold: u16,
    pub reviewer_rejected_threshold: u16,
    pub activity_approved_threshold: u16,
    pub activity_rejected_threshold: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Review {
    pub reviewer: Owner,
    pub approved: bool,
    pub reason: String,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Reviewer {
    pub chain_id: ChainId,
    pub reviewer: Owner,
    pub resume: Option<String>,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Content {
    /// Here cid is the content cid::Cid store in ipfs
    pub cid: String,
    pub comment_to_cid: Option<String>,
    pub author: Owner,
    pub title: String,
    pub content: String,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Asset {
    pub cid: String,
    pub base_uri: String,
    pub uris: Vec<String>,
    pub author: Owner,
    pub price: Option<Amount>,
    pub name: String,
    pub reviewers: HashMap<Owner, Review>,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Activity {
    pub activity_id: u64,
    pub activity_host: Owner,
    pub budget_amount: Amount,
    pub approved: u16,
    pub rejected: u16,
    pub created_at: Timestamp,
    pub reviewers: HashMap<Owner, Review>,
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
        reason: Option<String>,
    },
    RejectReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    ApproveContent {
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    },
    RejectContent {
        content_cid: String,
        reason: Option<String>,
    },
    SubmitComment {
        cid: String,
        comment_cid: String,
        comment: String,
    },
    ApproveAsset {
        cid: String,
        reason: Option<String>,
    },
    RejectAsset {
        cid: String,
        reason: Option<String>,
    },
    SubmitAsset {
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    },
    RequestSubscribe,
    ApproveActivity {
        activity_id: u64,
        reason: Option<String>,
    },
    RejectActivity {
        activity_id: u64,
        reason: String,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    GenesisReviewer,
    ExistReviewer {
        reviewer: Reviewer,
    },
    ApplyReviewer {
        resume: String,
    },
    UpdateReviewerResume {
        resume: String,
    },
    ApproveReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    RejectReviewer {
        candidate: Owner,
        reason: Option<String>,
    },
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    ApproveContent {
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    },
    RejectContent {
        content_cid: String,
        reason: Option<String>,
    },
    SubmitComment {
        cid: String,
        comment_cid: String,
        comment: String,
    },
    ApproveAsset {
        cid: String,
        reason: Option<String>,
    },
    RejectAsset {
        cid: String,
        reason: Option<String>,
    },
    SubmitAsset {
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    },
    RequestSubscribe,
    InitialState {
        state: InitialState,
    },
    SubmitActivity {
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    },
    ApproveActivity {
        activity_id: u64,
        reason: Option<String>,
    },
    RejectActivity {
        activity_id: u64,
        reason: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationCall {
    SubmitContent {
        cid: String,
        title: String,
        content: String,
    },
    SubmitActivity {
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    },
    ActivityApproved {
        activity_id: u64,
    },
}
