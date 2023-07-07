#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Feed;
use async_trait::async_trait;
use feed::{Operation, Content};
use linera_sdk::{
    base::{SessionId, WithContractAbi},
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
            Operation::Publish { cid } => {
                log::info!("Publish cid {:?} sender {:?} chain {:?}", cid, context.authenticated_signer, context.chain_id);
                match context.authenticated_signer {
                    Some(owner) => {
                        match self.create_content(Content {
                                cid,
                                comment_to_cid: None,
                                likes: 0,
                                dislikes: 0,
                                accounts: HashMap::default()
                            }, owner).await {
                            Ok(_) => return Ok(ExecutionResult::default()),
                            Err(err) => return Err(ContractError::StateError(err))
                        }
                    },
                    _ => {
                        return Err(ContractError::InvalidPublisher)
                    }
                }
            },
            Operation::Like { cid } => {
                log::info!("Like cid {:?} sender {:?} chain {:?}", cid, context.authenticated_signer, context.chain_id);
                match context.authenticated_signer {
                    Some(owner) => {
                        match self.like_content(cid, owner, true).await {
                            Ok(_) => return Ok(ExecutionResult::default()),
                            Err(err) => return Err(ContractError::StateError(err))
                        }
                    },
                    _ => {
                        return Err(ContractError::InvalidPublisher)
                    }
                }
            },
            Operation::Dislike { cid } => {
                log::info!("Dislike cid {:?} sender {:?} chain {:?}", cid, context.authenticated_signer, context.chain_id);
                match context.authenticated_signer {
                    Some(owner) => {
                        match self.like_content(cid, owner, false).await {
                            Ok(_) => return Ok(ExecutionResult::default()),
                            Err(err) => return Err(ContractError::StateError(err))
                        }
                    },
                    _ => {
                        return Err(ContractError::InvalidPublisher)
                    }
                }
            },
            Operation::Comment { comment_cid, content_cid } => {
                log::info!("Comment cid {:?} to cid {:?} sender {:?} chain {:?}", comment_cid, content_cid, context.authenticated_signer, context.chain_id);
            },
            Operation::Tip { cid, amount } => {
                log::info!("Tip cid {:?} amount {:?} sender {:?} chain {:?}", cid, amount, context.authenticated_signer, context.chain_id);
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

    #[error("Failed to call state")]
    StateError(#[from] state::StateError)
}
