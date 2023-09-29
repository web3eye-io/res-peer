#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Review;
use async_trait::async_trait;
use feed::FeedAbi;
use linera_sdk::{
    base::{ApplicationId, SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use review::Operation;
use thiserror::Error;

linera_sdk::contract!(Review);

impl WithContractAbi for Review {
    type Abi = review::ReviewAbi;
}

#[async_trait]
impl Contract for Review {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self.initialize(state).await?;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        let (need_notify, reason) = match operation {
            Operation::ApproveContent {
                content_cid,
                reason,
            } => (
                self.approve_content(context.authenticated_signer.unwrap(), content_cid)
                    .await?,
                reason,
            ),
            Operation::RejectContent {
                content_cid,
                reason,
            } => (
                self.reject_content(context.authenticated_signer.unwrap(), content_cid)
                    .await?,
                reason,
            ),
            Operation::ApproveAsset {
                collection_id,
                reason,
            } => (
                self.approve_asset(context.authenticated_signer.unwrap(), collection_id)
                    .await?,
                reason,
            ),
            Operation::RejectAsset {
                collection_id,
                reason,
            } => (
                self.reject_asset(context.authenticated_signer.unwrap(), collection_id)
                    .await?,
                reason,
            ),
        };
        // TODO: create reason content
        if !need_notify {
            // TODO: notify content approval or rejected
            return Ok(ExecutionResult::default());
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        _message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Ok(ApplicationCallResult::default())
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Ok(SessionCallResult::default())
    }
}

impl Review {
    fn feed_id() -> Result<ApplicationId<FeedAbi>, ContractError> {
        Self::parameters()
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum ContractError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    // Add more error variants here.
    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("Invalid user")]
    InvalidUser,
}
