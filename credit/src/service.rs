#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use credit::Operation;
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, WithServiceAbi},
    QueryContext, Service, ViewStateStorage,
};
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
    async fn liquidate(&self) -> Vec<u8> {
        vec![0]
    }

    async fn set_reward_callers(&self, application_ids: Vec<ApplicationId>) -> Vec<u8> {
        bcs::to_bytes(&Operation::SetRewardCallers { application_ids }).unwrap()
    }

    async fn set_transfer_callers(&self, application_ids: Vec<ApplicationId>) -> Vec<u8> {
        bcs::to_bytes(&Operation::SetTransferCallers { application_ids }).unwrap()
    }

    async fn transfer(&self, from: Owner, to: Owner, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&Operation::Transfer { from, to, amount }).unwrap()
    }

    async fn transfer_ext(&self, to: Owner, amount: Amount) -> Vec<u8> {
        bcs::to_bytes(&Operation::TransferExt { to, amount }).unwrap()
    }

    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe).unwrap()
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
