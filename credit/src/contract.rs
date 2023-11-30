#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Credit;
use async_trait::async_trait;
use credit::{ApplicationCall, Message, Operation};
use linera_sdk::{
    base::{ChannelName, Destination, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

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
        self.initialize_credit(state).await;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Liquidate => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Liquidate,
            )),
            Operation::SetRewardCallers { application_ids } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::SetRewardCallers { application_ids },
                )),
            Operation::SetTransferCallers { application_ids } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::SetTransferCallers { application_ids },
                )),
            Operation::Transfer { from, to, amount } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Transfer { from, to, amount },
                )),
            Operation::TransferExt { to, amount } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::TransferExt { to, amount },
                )),
            Operation::RequestSubscribe => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RequestSubscribe,
                )),
        }
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::InitialState { state } => {
                self.initialize_credit(state).await;
                Ok(ExecutionResult::default())
            }
            Message::Liquidate => {
                self.liquidate().await;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(dest, Message::Liquidate))
            }
            Message::Reward { owner, amount } => {
                self.reward(owner, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Reward { owner, amount }))
            }
            Message::SetRewardCallers { application_ids } => {
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.set_reward_callers(application_ids.clone()).await;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SetRewardCallers { application_ids },
                ))
            }
            Message::SetTransferCallers { application_ids } => {
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.set_transfer_callers(application_ids.clone()).await;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SetTransferCallers { application_ids },
                ))
            }
            Message::Transfer { from, to, amount } => {
                self.transfer(from, to, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Transfer { from, to, amount }))
            }
            Message::TransferExt { to, amount } => {
                self.transfer(context.authenticated_signer.unwrap(), to, amount)
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::TransferExt { to, amount }))
            }
            Message::RequestSubscribe => {
                let mut result = ExecutionResult::default();
                if context.message_id.chain_id
                    == system_api::current_application_id().creation.chain_id
                {
                    return Ok(result);
                }
                result.subscribe.push((
                    ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()),
                    context.message_id.chain_id,
                ));
                result = result.with_authenticated_message(
                    context.message_id.chain_id,
                    Message::InitialState {
                        state: self.initial_state().await?,
                    },
                );
                Ok(result)
            }
        }
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        let execution_result = match call {
            ApplicationCall::Reward { owner, amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Reward { owner, amount },
                ),
            ApplicationCall::Transfer { from, to, amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Transfer { from, to, amount },
                ),
        };
        let mut result = ApplicationCallResult::default();
        result.execution_result = execution_result;
        Ok(result)
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        Err(ContractError::SessionsNotSupported)
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

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
