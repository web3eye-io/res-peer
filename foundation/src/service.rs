#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use foundation::Operation;
use linera_sdk::{
    base::{Amount, WithServiceAbi},
    QueryContext, Service, ViewStateStorage,
};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(Foundation);

impl WithServiceAbi for Foundation {
    type Abi = foundation::FoundationAbi;
}

#[async_trait]
impl Service for Foundation {
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
    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe).unwrap()
    }

    async fn user_deposit(&self, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&Operation::UserDeposit { amount }).unwrap()
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
