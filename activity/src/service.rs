#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Activity;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::{base::WithServiceAbi, QueryContext, Service, ViewStateStorage};
use std::sync::Arc;

use activity::{ActivityError, AnnounceParams, CreateParams, Operation, UpdateParams};

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
    async fn create(&self, params: CreateParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Create { params }).unwrap()
    }

    async fn update(&self, params: UpdateParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Update { params }).unwrap()
    }

    async fn register(&self, activity_id: u64, object_id: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::Register {
            activity_id,
            object_id,
        })
        .unwrap()
    }

    async fn vote(&self, activity_id: u64, object_id: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::Vote {
            activity_id,
            object_id,
        })
        .unwrap()
    }

    async fn announce(&self, params: AnnounceParams) -> Vec<u8> {
        bcs::to_bytes(&Operation::Announce { params }).unwrap()
    }

    async fn request_subscribe(&self) -> Vec<u8> {
        bcs::to_bytes(&Operation::RequestSubscribe).unwrap()
    }

    async fn finalize(&self, activity_id: u64) -> Vec<u8> {
        bcs::to_bytes(&Operation::Finalize { activity_id }).unwrap()
    }
}
