#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Mall;
use async_trait::async_trait;
use credit::CreditAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, SessionId, WithContractAbi},
    contract::system_api,
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
        match context.authenticated_signer {
            Some(_) => {}
            _ => return Err(ContractError::InvalidOwner),
        }
        match operation {
            Operation::CreateCollection {
                base_uri,
                price,
                name,
            } => {
                self.create_collection(context.authenticated_signer.unwrap(), base_uri, price, name)
                    .await?
            }
            Operation::MintNFT {
                collection_id,
                uri,
                price,
                name,
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
                    name,
                )
                .await?;
            }
            Operation::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => {
                let owner = self.nft_owner(collection_id, token_id).await?;
                self.transfer_credits(
                    context,
                    context.authenticated_signer.unwrap(),
                    owner,
                    credits,
                )
                .await?;
                self.buy_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                    credits,
                )
                .await?
            }
            Operation::UpdateCreditsPerLinera { credits_per_linera } => {
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.credits_per_linera.set(credits_per_linera)
            }
            Operation::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => {
                self.update_nft_price(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                    price,
                )
                .await?
            }
            Operation::OnSaleNFT {
                collection_id,
                token_id,
            } => {
                self.on_sale_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?
            }
            Operation::OffSaleNFT {
                collection_id,
                token_id,
            } => {
                self.off_sale_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?
            }
            Operation::Deposit { amount } => {
                self.deposit(context.authenticated_signer.unwrap(), amount)
                    .await?
            }
            Operation::SetAvatar {
                collection_id,
                token_id,
            } => {
                self.set_avatar(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?
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

impl Mall {
    fn credit_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Self::parameters()
    }

    async fn transfer_credits(
        &mut self,
        _context: &OperationContext,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        log::info!("Transfer {:?} credits from {:?} to {:?}", amount, from, to);
        let call = credit::ApplicationCall::Transfer { from, to, amount };
        self.call_application(true, Self::credit_id()?, &call, vec![])
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
    // Add more error variants here.
    #[error("NOT IMPLEMENTED")]
    NotImplemented,

    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("Operation not allowed")]
    OperationNotAllowed,

    #[error("Invalid owner")]
    InvalidOwner,
}
