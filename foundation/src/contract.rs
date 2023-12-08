#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashSet;

use self::state::Foundation;
use async_trait::async_trait;
use foundation::{ApplicationCall, Message, Operation, RewardType};
use linera_sdk::{
    base::{Amount, ChannelName, Destination, Owner, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Foundation);

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

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
        self.initialize_foundation(state).await?;
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
                self.initialize_foundation(state).await?;
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
            Message::Deposit { from, amount } => {
                self.deposit(from, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Deposit { from, amount }))
            }
            Message::Lock {
                activity_id,
                amount,
            } => {
                self.lock(activity_id, amount).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Lock {
                        activity_id,
                        amount,
                    },
                ))
            }
            Message::Reward {
                reward_user,
                reward_type,
                activity_id,
            } => {
                let reward_user = match reward_type {
                    RewardType::Review => context.authenticated_signer,
                    // TODO: activity reward should be reviewed then here will removed
                    RewardType::Activity => context.authenticated_signer,
                    _ => reward_user,
                };
                let _reward_user = match reward_user {
                    Some(user) => user,
                    None => return Err(ContractError::InvalidUser),
                };
                self.reward(_reward_user, reward_type, activity_id).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Reward {
                        reward_user,
                        reward_type,
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
            Message::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            } => {
                self._activity_rewards(
                    activity_id,
                    winner_user,
                    voter_users.clone(),
                    reward_amount,
                    voter_reward_percent,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::ActivityRewards {
                        activity_id,
                        winner_user,
                        voter_users,
                        reward_amount,
                        voter_reward_percent,
                    },
                ))
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
            ApplicationCall::Deposit { from, amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Deposit { from, amount },
                ),
            ApplicationCall::Lock {
                activity_id,
                amount,
            } => ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Lock {
                    activity_id,
                    amount,
                },
            ),
            ApplicationCall::Reward {
                reward_user,
                reward_type,
                activity_id,
            } => ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Reward {
                    reward_user,
                    reward_type,
                    activity_id,
                },
            ),
            ApplicationCall::Transfer { from, to, amount } => ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Transfer { from, to, amount },
                ),
            ApplicationCall::Balance { owner } => {
                let balance = self.balance(owner).await?;
                let mut result = ApplicationCallResult::default();
                result.value = balance;
                return Ok(result);
            }
            ApplicationCall::ActivityRewards {
                activity_id,
                winner_user,
                voter_users,
                reward_amount,
                voter_reward_percent,
            } => ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::ActivityRewards {
                    activity_id,
                    winner_user,
                    voter_users,
                    reward_amount,
                    voter_reward_percent,
                },
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

impl Foundation {
    async fn _activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), ContractError> {
        self.spend_activity_funds(activity_id, reward_amount)
            .await?;
        self.distribute_activity_rewards(
            winner_user,
            voter_users,
            reward_amount,
            voter_reward_percent,
        )
        .await?;
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

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    // Add more error variants here.
    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("Insufficient funds")]
    InsufficientFunds,
}
