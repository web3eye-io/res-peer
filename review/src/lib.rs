use async_graphql::{Request, Response};
use linera_sdk::base::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct ReviewAbi;

impl ContractAbi for ReviewAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = ();
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for ReviewAbi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub content_approved_threshold: i16,
    pub content_rejected_threshold: i16,
    pub asset_approved_threshold: i16,
    pub asset_rejected_threshold: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
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
}
