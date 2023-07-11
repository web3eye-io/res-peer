#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_trait::async_trait;
use credit::{ApplicationCall, Operation};
use linera_sdk::{
    base::{SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Credit);

impl WithContractAbi for Credit {
    type Abi = credit::CreditAbi;
}

#[async_trait]
impl Contract for Credit {
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
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Liquidate => self.liquidate().await,
            Operation::Reward { owner, amount } => self.reward(owner, amount).await?,
            _ => {}
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
                match self.reward(owner, amount).await {
                    Ok(_) => return Ok(ApplicationCallResult::default()),
                    Err(err) => return Err(ContractError::StateError(err)),
                };
            }
            _ => return Err(ContractError::NotImplemented),
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
}
