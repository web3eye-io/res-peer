#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Mall;
use async_trait::async_trait;
use linera_sdk::{
    base::{SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use mall::Operation;
use thiserror::Error;

linera_sdk::contract!(Mall);

impl WithContractAbi for Mall {
    type Abi = mall::MallAbi;
}

#[async_trait]
impl Contract for Mall {
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
            Operation::OnSaleCollection { base_uri, price } => {
                self.create_collection(context.authenticated_signer.unwrap(), base_uri, price)
                    .await
            }
            Operation::MintNFT {
                collection_id,
                uri,
                price,
            } => {
                self.validate_collection_owner(
                    collection_id,
                    context.authenticated_signer.unwrap(),
                )
                .await?;
                self.mint_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    uri,
                    price,
                )
                .await;
            }
            _ => return Err(ContractError::NotImplemented),
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
    #[error("NOT IMPLEMENTED")]
    NotImplemented,

    #[error(transparent)]
    StateError(#[from] state::StateError),
}
