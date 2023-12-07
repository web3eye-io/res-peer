#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Review;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::{
    base::{Amount, Owner, WithServiceAbi},
    QueryContext, Service, ViewStateStorage,
};
use review::Operation;
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(Review);

impl WithServiceAbi for Review {
    type Abi = review::ReviewAbi;
}

#[async_trait]
impl Service for Review {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn handle_query(
        self: Arc<Self>,
        _context: &QueryContext,
        request: Request,
    ) -> Result<Response, Self::Error> {
        let schema = Schema::build(self.clone(), MutationRoot {}, EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn apply_reviewer(&self, resume: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApplyReviewer { resume }).unwrap()
    }

    async fn update_reviewer_resume(&self, resume: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::UpdateReviewerResume { resume }).unwrap()
    }

    async fn approve_reviewer(&self, candidate: Owner, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveReviewer { candidate, reason }).unwrap()
    }

    async fn reject_reviewer(&self, candidate: Owner, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectReviewer { candidate, reason }).unwrap()
    }

    async fn submit_content(&self, cid: String, title: String, content: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::SubmitContent {
            cid,
            title,
            content,
        })
        .unwrap()
    }

    async fn approve_content(
        &self,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
    ) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveContent {
            content_cid,
            reason_cid,
            reason,
        })
        .unwrap()
    }

    async fn reject_content(&self, content_cid: String, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectContent {
            content_cid,
            reason,
        })
        .unwrap()
    }

    async fn submit_comment(&self, cid: String, comment_cid: String, comment: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::SubmitComment {
            cid,
            comment_cid,
            comment,
        })
        .unwrap()
    }

    async fn approve_asset(&self, cid: String, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveAsset { cid, reason }).unwrap()
    }

    async fn reject_asset(&self, cid: String, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectAsset { cid, reason }).unwrap()
    }

    async fn submit_asset(
        &self,
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    ) -> Vec<u8> {
        bcs::to_bytes(&Operation::SubmitAsset {
            cid,
            base_uri,
            uris,
            price,
            name,
        })
        .unwrap()
    }

    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe {}).unwrap()
    }

    async fn approve_activity(&self, activity_id: u64, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveActivity {
            activity_id,
            reason,
        })
        .unwrap()
    }

    async fn reject_activity(&self, activity_id: u64, reason: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectActivity {
            activity_id,
            reason,
        })
        .unwrap()
    }
}

/// An error that can occur while querying the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Query not supported by the application.
    #[error("Queries not supported by application")]
    QueriesNotSupported,

    /// Invalid query argument; could not deserialize request.
    #[error("Invalid query argument; could not deserialize request")]
    InvalidQuery(#[from] serde_json::Error),
    // Add error variants here.
}
