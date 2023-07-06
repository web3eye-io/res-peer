#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_graphql::{Response, Schema, Request, EmptySubscription, Object};
use async_trait::async_trait;
use linera_sdk::{base::WithServiceAbi, QueryContext, Service, ViewStateStorage};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(Credit);

impl WithServiceAbi for Credit {
    type Abi = credit::CreditAbi;
}

#[async_trait]
impl Service for Credit {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn query_application(
        self: Arc<Self>,
        _context: &QueryContext,
        request: Request,
    ) -> Result<Response, Self::Error> {
        let schema = Schema::build(
            self.clone(),
            MutationRoot {},
            EmptySubscription,
        ).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn liquidate(&self) -> Vec<u8> {
        vec![0]
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
