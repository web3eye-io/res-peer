#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_trait::async_trait;
use foundation::{ApplicationCall, RewardType};
use linera_sdk::{
    base::{SessionId, WithContractAbi},
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
        self.initialize(state).await?;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        _operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
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
            ApplicationCall::Deposit { amount } => self.deposit(amount).await?,
            ApplicationCall::Lock {
                activity_id,
                activity_host,
                amount,
            } => self.lock(activity_host, activity_id, amount).await?,
            ApplicationCall::Reward {
                reward_user,
                reward_type,
                amount,
                activity_id,
            } => {
                let reward_user = match reward_type {
                    RewardType::Review => context.authenticated_signer,
                    _ => reward_user,
                };
                let reward_user = match reward_user {
                    Some(user) => user,
                    None => return Err(ContractError::InvalidUser),
                };
                let activity_host = match reward_type {
                    RewardType::Activity => match context.authenticated_signer {
                        Some(user) => Some(user),
                        None => return Err(ContractError::InvalidUser),
                    },
                    _ => None,
                };
                self.reward(reward_user, reward_type, amount, activity_id, activity_host)
                    .await?
            }
            ApplicationCall::Transfer { from, to, amount } => {
                self.transfer(from, to, amount).await?
            }
        }
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
