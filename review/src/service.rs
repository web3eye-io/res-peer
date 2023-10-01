#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Review;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::{
    base::{Owner, WithServiceAbi},
    QueryContext, Service, ViewStateStorage,
};
use review::Operation;
use std::{string, sync::Arc};
use thiserror::Error;

linera_sdk::service!(Review);

impl WithServiceAbi for Review {
    type Abi = review::ReviewAbi;
}

#[async_trait]
impl Service for Review {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn query_application(
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

    async fn approve_reviewer(&self, candidate: Owner) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveReviewer { candidate }).unwrap()
    }

    async fn reject_reviewer(&self, candidate: Owner) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectReviewer { candidate }).unwrap()
    }

    async fn submit_content(&self, cid: String, title: String, content: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::SubmitContent {
            cid,
            title,
            content,
        })
        .unwrap()
    }

    async fn approve_content(&self, content_cid: String, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveContent {
            content_cid,
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

    async fn approve_asset(&self, collection_id: u64, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::ApproveAsset {
            collection_id,
            reason,
        })
        .unwrap()
    }

    async fn reject_asset(&self, collection_id: u64, reason: Option<String>) -> Vec<u8> {
        bcs::to_bytes(&Operation::RejectAsset {
            collection_id,
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
