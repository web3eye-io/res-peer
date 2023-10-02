#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::{collections::HashMap, str::FromStr};

use self::state::Feed;
use async_trait::async_trait;
use credit::CreditAbi;
use feed::{ApplicationCall, Content, Message, Operation};
use linera_sdk::{
    base::{
        Amount, ApplicationId, ChainId, ChannelName, Destination, Owner, SessionId, WithContractAbi,
    },
    contract::system_api::{self, current_system_time},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(Feed);

impl WithContractAbi for Feed {
    type Abi = feed::FeedAbi;
}

const CREATION_CHAIN_ID: &str = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
const PUBLISHED_CONTENT_CHANNEL_NAME: &[u8] = b"published_contents";

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
            Operation::Like { cid } => {
                self.like(cid, context.authenticated_signer.unwrap())
                    .await?
            }
            Operation::Dislike { cid } => {
                self.dislike(cid, context.authenticated_signer.unwrap())
                    .await?
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
            Operation::RequestPublishedSubscribe => {
                return Ok(ExecutionResult::default().with_message(
                    ChainId::from_str(CREATION_CHAIN_ID).unwrap(),
                    Message::RequestPublishedSubscribe,
                ));
            }
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::Publish {
                cid,
                title,
                content,
                author,
            } => {
                self.publish(cid.clone(), title.clone(), content.clone(), author)
                    .await?;
                log::info!("Published cid {:?} sender {:?}", cid, author);
                let dest = Destination::Subscribers(ChannelName::from(
                    PUBLISHED_CONTENT_CHANNEL_NAME.to_vec(),
                ));
                log::info!(
                    "Broadcast published cid {:?} to {:?} at {}",
                    cid,
                    dest,
                    context.chain_id
                );
                return Ok(ExecutionResult::default().with_message(
                    dest,
                    Message::Publish {
                        cid,
                        title,
                        content,
                        author,
                    },
                ));
            }
            Message::RequestPublishedSubscribe => {
                let mut result = ExecutionResult::default();
                log::info!(
                    "Subscribe to {} at {} creation {}",
                    context.message_id.chain_id,
                    context.chain_id,
                    system_api::current_application_id().creation.chain_id
                );
                if context.message_id.chain_id
                    == system_api::current_application_id().creation.chain_id
                {
                    return Ok(result);
                }
                result.subscribe.push((
                    ChannelName::from(PUBLISHED_CONTENT_CHANNEL_NAME.to_vec()),
                    context.message_id.chain_id,
                ));
                return Ok(result);
            }
        }
    }

    async fn handle_application_call(
        &mut self,
        context: &CalleeContext,
        call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        match call {
            ApplicationCall::Recommend {
                cid,
                reason_cid,
                reason,
            } => {
                let author = context.authenticated_signer.unwrap();
                self.publish(reason_cid.clone(), String::default(), reason, author)
                    .await?;
                self.recommend_content(cid, reason_cid).await?;
            }
            ApplicationCall::Publish {
                cid,
                title,
                content,
                author,
            } => self.publish(cid, title, content, author).await?,
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
        Err(ContractError::SessionsNotSupported)
    }
}

impl Feed {
    fn credit_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Self::parameters()
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        log::info!("Reward owner {:?} amount {:?}", owner, amount);
        let call = credit::ApplicationCall::Reward { owner, amount };
        self.call_application(true, Self::credit_id()?, &call, vec![])
            .await?;
        log::info!("Rewarded owner {:?} amount {:?}", owner, amount);
        Ok(())
    }

    async fn publish(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ContractError> {
        log::info!("Publish cid {:?} sender {:?}", cid, author,);
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
                author,
            )
            .await
        {
            Ok(_) => self.reward_credits(author, Amount::from_tokens(500)).await,
            Err(err) => Err(ContractError::StateError(err)),
        }
    }

    async fn like(&mut self, cid: String, owner: Owner) -> Result<(), ContractError> {
        log::info!("Like cid {:?} sender {:?}", cid, owner,);
        match self.like_content(cid, owner, true).await {
            Ok(_) => {
                return self.reward_credits(owner, Amount::from_tokens(100)).await;
            }
            Err(err) => return Err(ContractError::StateError(err)),
        }
    }

    async fn dislike(&mut self, cid: String, owner: Owner) -> Result<(), ContractError> {
        log::info!("Dislike cid {:?} sender {:?}", cid, owner,);
        match self.like_content(cid, owner, false).await {
            Ok(_) => {
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
