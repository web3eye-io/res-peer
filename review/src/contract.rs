#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Review;
use async_trait::async_trait;
use feed::FeedAbi;
use linera_sdk::{
    base::{ApplicationId, Owner, SessionId, WithContractAbi},
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
        context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self.initialize(context.authenticated_signer.unwrap(), state).await?;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::ApplyReviewer { resume } => {
                self._apply_reviewer(context.authenticated_signer.unwrap(), resume)
                    .await?;
            }
            Operation::UpdateReviewerResume { resume } => {
                self._update_reviewer_resume(context.authenticated_signer.unwrap(), resume)
                    .await?;
            }
            Operation::ApproveReviewer { candidate } => {
                self._approve_reviewer(context.authenticated_signer.unwrap(), candidate)
                    .await?;
            }
            Operation::RejectReviewer { candidate } => {
                self._reject_reviewer(context.authenticated_signer.unwrap(), candidate)
                    .await?;
            }
            Operation::ApproveContent {
                content_cid,
                reason,
            } => {
                self._approve_content(context.authenticated_signer.unwrap(), content_cid, reason)
                    .await?;
            }
            Operation::RejectContent {
                content_cid,
                reason,
            } => {
                self._reject_content(context.authenticated_signer.unwrap(), content_cid, reason)
                    .await?;
            }
            Operation::ApproveAsset {
                collection_id,
                reason,
            } => {
                self._approve_asset(context.authenticated_signer.unwrap(), collection_id, reason)
                    .await?;
            }
            Operation::RejectAsset {
                collection_id,
                reason,
            } => {
                self._reject_asset(context.authenticated_signer.unwrap(), collection_id, reason)
                    .await?;
            }
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

    async fn _apply_reviewer(
        &mut self,
        candidate: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.apply_reviewer(candidate, resume).await?;
        Ok(())
    }

    async fn _update_reviewer_resume(
        &mut self,
        reviewer: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.update_reviewer_resume(reviewer, resume).await?;
        Ok(())
    }

    async fn _approve_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
    ) -> Result<(), ContractError> {
        self.approve_reviewer(owner, candidate).await?;
        // TODO: if approved, subscribe submitted content
        // TODO: notify reviewer
        Ok(())
    }

    async fn _reject_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
    ) -> Result<(), ContractError> {
        self.reject_reviewer(owner, candidate).await?;
        // TODO: notify reviewer
        Ok(())
    }

    async fn _approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.approve_content(reviewer, content_cid).await?;
        // TODO: add reason
        // TODO: if already approved, publish content
        // TODO: notify author
        Ok(())
    }

    async fn _reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.reject_content(reviewer, content_cid).await?;
        // TODO: add reason
        // TODO: notify author
        Ok(())
    }

    async fn _approve_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.approve_asset(reviewer, collection_id).await?;
        // TODO: add reason
        // TODO: if already approved, publish asset
        // TODO: notify author
        Ok(())
    }

    async fn _reject_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
        _reason: Option<String>,
    ) -> Result<(), ContractError> {
        self.reject_asset(reviewer, collection_id).await?;
        // TODO: add reason
        // TODO: notify author
        Ok(())
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
