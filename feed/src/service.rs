#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Feed;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use feed::Operation;
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    QueryContext, Service, ViewStateStorage,
};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(Feed);

impl WithServiceAbi for Feed {
    type Abi = feed::FeedAbi;
}

#[async_trait]
impl Service for Feed {
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
    async fn publish(&self, ccid: String) -> Vec<u8> {
        cid::Cid::try_from(ccid.clone()).expect("Invalid content cid");
        bcs::to_bytes(&Operation::Publish { cid: ccid }).unwrap()
    }

    async fn like(&self, ccid: String) -> Vec<u8> {
        cid::Cid::try_from(ccid.clone()).expect("Invalid content cid");
        bcs::to_bytes(&Operation::Like { cid: ccid }).unwrap()
    }

    async fn dislike(&self, ccid: String) -> Vec<u8> {
        cid::Cid::try_from(ccid.clone()).expect("Invalid content cid");
        bcs::to_bytes(&Operation::Like { cid: ccid }).unwrap()
    }

    async fn comment(&self, ccid: String, comment_cid: String) -> Vec<u8> {
        cid::Cid::try_from(ccid.clone()).expect("Invalid content cid");
        cid::Cid::try_from(comment_cid.clone()).expect("Invalid comment cid");
        bcs::to_bytes(&Operation::Comment {
            content_cid: ccid,
            comment_cid,
        })
        .unwrap()
    }

    async fn tip(&self, ccid: String, amount: Amount) -> Vec<u8> {
        cid::Cid::try_from(ccid.clone()).expect("Invalid content cid");
        bcs::to_bytes(&Operation::Tip { cid: ccid, amount }).unwrap()
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
