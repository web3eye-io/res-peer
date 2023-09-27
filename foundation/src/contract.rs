#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_trait::async_trait;
use foundation::{ApplicationCall, Operation};
use linera_sdk::{
    base::{SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Foundation);

impl WithContractAbi for Foundation {
    type Abi = foundation::FoundationAbi;
}

#[async_trait]
impl Contract for Foundation {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self.initialize(state).await;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Liquidate => self.liquidate().await,
            Operation::Reward { owner, amount } => self.reward(owner, amount).await?,
            Operation::SetRewardCallers { application_ids } => {
                // Operation could be only created by chain owner, so here we don't need to verify owner
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.set_reward_callers(application_ids).await
            }
            Operation::SetTransferCallers { application_ids } => {
                // Operation could be only created by chain owner, so here we don't need to verify owner
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.set_transfer_callers(application_ids).await
            }
            Operation::Transfer { from, to, amount } => {
                self.transfer(from, to, amount).await?;
            }
            Operation::TransferExt { to, amount } => {
                self.transfer(context.authenticated_signer.unwrap(), to, amount)
                    .await?
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
        context: &CalleeContext,
        call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        match call {
            ApplicationCall::Reward { owner, amount } => {
                log::info!(
                    "Reward owner {} amount {} caller {}",
                    owner,
                    amount,
                    context.authenticated_caller_id.unwrap()
                );
                match self
                    .reward_callers
                    .contains(&context.authenticated_caller_id.unwrap())
                    .await
                {
                    Ok(_) => {}
                    _ => return Err(ContractError::CallerNotAllowed),
                }
                self.reward(owner, amount).await?;
                Ok(ApplicationCallResult::default())
            }
            ApplicationCall::Transfer { from, to, amount } => {
                log::info!(
                    "Transfer {} from {} to {} caller {}",
                    amount,
                    from,
                    to,
                    context.authenticated_caller_id.unwrap()
                );
                match self
                    .transfer_callers
                    .contains(&context.authenticated_caller_id.unwrap())
                    .await
                {
                    Ok(_) => {}
                    _ => return Err(ContractError::CallerNotAllowed),
                }
                self.transfer(from, to, amount).await?;
                Ok(ApplicationCallResult::default())
            }
        }
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

    #[error("NOT IMPLEMENTED")]
    NotImplemented,

    #[error("Caller not allowed")]
    CallerNotAllowed,

    #[error("Operation not allowed")]
    OperationNotAllowed,
}
