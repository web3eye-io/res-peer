#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Activity;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::{base::WithServiceAbi, QueryContext, Service, ViewStateStorage};
use std::sync::Arc;

use activity::{ActivityError, Operation};

linera_sdk::service!(Activity);

impl WithServiceAbi for Activity {
    type Abi = activity::ActivityAbi;
}

#[async_trait]
impl Service for Activity {
    type Error = ActivityError;
    type Storage = ViewStateStorage<Self>;

    async fn handle_query(
        self: Arc<Self>,
        _context: &QueryContext,
        request: Request,
    ) -> Result<Response, Self::Error> {
        let schema = Schema::build(self.clone(), MutationRoot, EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn operation(&self, operation: Operation) -> Vec<u8> {
        bcs::to_bytes(&operation).unwrap()
    }
}
