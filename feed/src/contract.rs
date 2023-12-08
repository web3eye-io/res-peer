#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Feed;
use async_trait::async_trait;
use credit::CreditAbi;
use feed::{ApplicationCall, Content, Message, Operation};
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Owner, SessionId, WithContractAbi},
    contract::system_api::{self, current_system_time},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Feed);

impl WithContractAbi for Feed {
    type Abi = feed::FeedAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

#[async_trait]
impl Contract for Feed {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self.initialize_feed(state).await;
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Like { cid } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Like { cid },
            )),
            Operation::Dislike { cid } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Dislike { cid },
                )),
            Operation::Tip { cid, amount } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Tip { cid, amount },
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
            Message::Like { cid } => {
                self.like(
                    cid.clone(),
                    context.authenticated_signer.unwrap(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Like { cid }))
            }
            Message::Dislike { cid } => {
                self.dislike(
                    cid.clone(),
                    context.authenticated_signer.unwrap(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Dislike { cid }))
            }
            Message::Tip { cid, amount } => {
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Tip { cid, amount }))
            }
            Message::Publish {
                cid,
                title,
                content,
                author,
            } => {
                self.publish(
                    cid.clone(),
                    None,
                    title.clone(),
                    content.clone(),
                    author,
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Publish {
                        cid,
                        title,
                        content,
                        author,
                    },
                ))
            }
            Message::Recommend {
                cid,
                reason_cid,
                reason,
            } => {
                let author = context.authenticated_signer.unwrap();
                self.publish(
                    reason_cid.clone(),
                    Some(cid.clone()),
                    String::default(),
                    reason.clone(),
                    author,
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                self.recommend_content(cid.clone(), reason_cid.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Recommend {
                        cid,
                        reason_cid,
                        reason,
                    },
                ))
            }
            Message::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            } => {
                self.publish(
                    comment_cid.clone(),
                    Some(cid.clone()),
                    String::default(),
                    comment.clone(),
                    commentor,
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                self.comment_content(cid.clone(), comment_cid.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Comment {
                        cid,
                        comment_cid,
                        comment,
                        commentor,
                    },
                ))
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
        match call {
            ApplicationCall::Recommend {
                cid,
                reason_cid,
                reason,
            } => Ok(ApplicationCallResult {
                value: None,
                execution_result: ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Recommend {
                        cid,
                        reason_cid,
                        reason,
                    },
                ),
                create_sessions: vec![],
            }),
            ApplicationCall::Comment {
                cid,
                comment_cid,
                comment,
                commentor,
            } => Ok(ApplicationCallResult {
                value: None,
                execution_result: ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Comment {
                        cid,
                        comment_cid,
                        comment,
                        commentor,
                    },
                ),
                create_sessions: vec![],
            }),
            ApplicationCall::Publish {
                cid,
                title,
                content,
                author,
            } => Ok(ApplicationCallResult {
                value: None,
                execution_result: ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Publish {
                        cid,
                        title,
                        content,
                        author,
                    },
                ),
                create_sessions: vec![],
            }),
            ApplicationCall::ContentAuthor { cid } => Ok(ApplicationCallResult {
                value: Some(self.content_author(cid).await?),
                execution_result: ExecutionResult::default(),
                create_sessions: vec![],
            }),
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
        Err(ContractError::SessionsNotSupported)
    }
}

impl Feed {
    fn credit_app_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Ok(Self::parameters().unwrap().credit_app_id)
    }

    fn foundation_app_id() -> Result<ApplicationId<FoundationAbi>, ContractError> {
        Ok(Self::parameters().unwrap().foundation_app_id)
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        let call = credit::ApplicationCall::Reward { owner, amount };
        self.call_application(true, Self::credit_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn reward_tokens(&mut self, author: Owner) -> Result<(), ContractError> {
        let call = foundation::ApplicationCall::Reward {
            reward_user: Some(author),
            reward_type: foundation::RewardType::Publish,
            activity_id: None,
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn publish(
        &mut self,
        cid: String,
        comment_to_cid: Option<String>,
        title: String,
        content: String,
        author: Owner,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        match self
            .create_content(
                Content {
                    cid,
                    comment_to_cid,
                    title,
                    content,
                    author,
                    likes: 0,
                    dislikes: 0,
                    accounts: HashMap::default(),
                    created_at: current_system_time(),
                },
                author,
            )
            .await
        {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                self.reward_credits(author, Amount::from_tokens(500))
                    .await?;
                self.reward_tokens(author).await?;
                Ok(())
            }
            Err(err) => Err(ContractError::StateError(err)),
        }
    }

    async fn like(
        &mut self,
        cid: String,
        owner: Owner,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        match self.like_content(cid, owner, true).await {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                return self.reward_credits(owner, Amount::from_tokens(100)).await;
            }
            Err(err) => return Err(ContractError::StateError(err)),
        }
    }

    async fn dislike(
        &mut self,
        cid: String,
        owner: Owner,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        match self.like_content(cid, owner, false).await {
            Ok(_) => {
                if !creation_chain {
                    return Ok(());
                }
                return self.reward_credits(owner, Amount::from_tokens(100)).await;
            }
            Err(err) => return Err(ContractError::StateError(err)),
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
