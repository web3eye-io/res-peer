#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Feed;
use async_trait::async_trait;
use credit::CreditAbi;
use feed::{Content, Message, Operation};
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, SessionId, WithContractAbi},
    contract::system_api::current_system_time,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Feed);

impl WithContractAbi for Feed {
    type Abi = feed::FeedAbi;
}

#[async_trait]
impl Contract for Feed {
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
            Operation::Publish {
                cid,
                title,
                content,
            } => {
                self.publish(
                    context,
                    cid,
                    title,
                    content,
                    context.authenticated_signer.unwrap(),
                )
                .await?
            }
            Operation::Like { cid } => self.like(context, cid).await?,
            Operation::Dislike { cid } => self.dislike(context, cid).await?,
            Operation::Comment {
                comment_cid,
                content_cid,
            } => {
                log::info!(
                    "Comment cid {:?} to cid {:?} sender {:?} chain {:?}",
                    comment_cid,
                    content_cid,
                    context.authenticated_signer.unwrap(),
                    context.chain_id
                );
            }
            Operation::Tip { cid, amount } => {
                log::info!(
                    "Tip cid {:?} amount {:?} sender {:?} chain {:?}",
                    cid,
                    amount,
                    context.authenticated_signer.unwrap(),
                    context.chain_id
                );
            }
            Operation::CreateChannel { name, chain_id } => {
                return Ok(ExecutionResult::default()
                    .with_message(chain_id, Message::CreateChannel { name, chain_id }));
            }
            Operation::DeleteChannel { channel_id } => self.delete_channel(channel_id).await?,
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        __context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::CreateChannel { name, chain_id } => {
                log::info!("Create channel {} at {}", name, chain_id);
                self.create_channel(name, chain_id).await?
            }
        }
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
        Err(ContractError::SessionsNotSupported)
    }
}

impl Feed {
    fn credit_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Self::parameters()
    }

    async fn reward_credits(
        &mut self,
        _context: &OperationContext,
        owner: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        log::info!("Reward owner {:?} amount {:?}", owner, amount);
        let call = credit::ApplicationCall::Reward { owner, amount };
        self.call_application(true, Self::credit_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn publish(
        &mut self,
        context: &OperationContext,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ContractError> {
        log::info!(
            "Publish cid {:?} sender {:?} chain {:?}",
            cid,
            context.authenticated_signer,
            context.chain_id
        );
        match context.authenticated_signer {
            Some(owner) => {
                match self
                    .create_content(
                        Content {
                            cid,
                            title,
                            content,
                            author,
                            likes: 0,
                            dislikes: 0,
                            accounts: HashMap::default(),
                            created_at: current_system_time(),
                        },
                        owner,
                    )
                    .await
                {
                    Ok(_) => {
                        return self
                            .reward_credits(context, owner, Amount::from_tokens(500))
                            .await;
                    }
                    Err(err) => return Err(ContractError::StateError(err)),
                }
            }
            _ => return Err(ContractError::InvalidPublisher),
        }
    }

    async fn like(&mut self, context: &OperationContext, cid: String) -> Result<(), ContractError> {
        log::info!(
            "Like cid {:?} sender {:?} chain {:?}",
            cid,
            context.authenticated_signer,
            context.chain_id
        );
        match context.authenticated_signer {
            Some(owner) => match self.like_content(cid, owner, true).await {
                Ok(_) => {
                    return self
                        .reward_credits(context, owner, Amount::from_tokens(100))
                        .await;
                }
                Err(err) => return Err(ContractError::StateError(err)),
            },
            _ => return Err(ContractError::InvalidPublisher),
        }
    }

    async fn dislike(
        &mut self,
        context: &OperationContext,
        cid: String,
    ) -> Result<(), ContractError> {
        log::info!(
            "Dislike cid {:?} sender {:?} chain {:?}",
            cid,
            context.authenticated_signer,
            context.chain_id
        );
        match context.authenticated_signer {
            Some(owner) => match self.like_content(cid, owner, false).await {
                Ok(_) => {
                    return self
                        .reward_credits(context, owner, Amount::from_tokens(100))
                        .await;
                }
                Err(err) => return Err(ContractError::StateError(err)),
            },
            _ => return Err(ContractError::InvalidPublisher),
        }
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
    #[error("Invalid publisher")]
    InvalidPublisher,

    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
