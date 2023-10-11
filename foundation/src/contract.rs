#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Foundation;
use async_trait::async_trait;
use foundation::{ApplicationCall, Message, Operation, RewardType};
use linera_sdk::{
    base::{ChannelName, Destination, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

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
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::UserDeposit { amount } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::UserDeposit { amount },
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
                self.initialize(state).await?;
                Ok(ExecutionResult::default())
            }
            Message::UserDeposit { amount } => {
                self.user_deposit(context.authenticated_signer.unwrap(), amount)
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::UserDeposit { amount }))
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
                return Ok(result);
            }
            Message::Deposit { amount } => {
                self.deposit(amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Deposit { amount }))
            }
            Message::Lock {
                activity_id,
                activity_host,
                amount,
            } => {
                self.lock(activity_host, activity_id, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Lock {
                        activity_id,
                        activity_host,
                        amount,
                    },
                ))
            }
            Message::Reward {
                reward_user,
                reward_type,
                amount,
                activity_id,
            } => {
                let reward_user = match reward_type {
                    RewardType::Review => context.authenticated_signer,
                    _ => reward_user,
                };
                let _reward_user = match reward_user {
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
                self.reward(
                    _reward_user,
                    reward_type,
                    amount,
                    activity_id,
                    activity_host,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Reward {
                        reward_user,
                        reward_type,
                        amount,
                        activity_id,
                    },
                ))
            }
            Message::Transfer { from, to, amount } => {
                self.transfer(from, to, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Transfer { from, to, amount }))
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
        let execute_result = match call {
            ApplicationCall::Deposit { amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Deposit { amount },
                ),
            ApplicationCall::Lock {
                activity_id,
                activity_host,
                amount,
            } => ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Lock {
                    activity_id,
                    activity_host,
                    amount,
                },
            ),
            ApplicationCall::Reward {
                reward_user,
                reward_type,
                amount,
                activity_id,
            } => ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Reward {
                    reward_user,
                    reward_type,
                    amount,
                    activity_id,
                },
            ),
            ApplicationCall::Transfer { from, to, amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Transfer { from, to, amount },
                ),
        };
        let mut result = ApplicationCallResult::default();
        result.execution_result = execute_result;
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

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
