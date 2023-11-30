#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Market;
use async_trait::async_trait;
use credit::CreditAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Owner, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use market::{ApplicationCall, Message, Operation};
use thiserror::Error;

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

linera_sdk::contract!(Market);

impl WithContractAbi for Market {
    type Abi = market::MarketAbi;
}

#[async_trait]
impl Contract for Market {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self.initialize_market(state).await;
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
            Operation::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::MintNFT {
                    collection_id,
                    uri_index,
                    price,
                    name,
                },
            )),
            Operation::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::BuyNFT {
                    collection_id,
                    token_id,
                    credits,
                },
            )),
            Operation::UpdateCreditsPerLinera { credits_per_linera } => {
                Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::UpdateCreditsPerLinera { credits_per_linera },
                ))
            }
            Operation::UpdateNFTPrice {
                collection_id,
                token_id,
                price,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::UpdateNFTPrice {
                    collection_id,
                    token_id,
                    price,
                },
            )),
            Operation::OnSaleNFT {
                collection_id,
                token_id,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::OnSaleNFT {
                    collection_id,
                    token_id,
                },
            )),
            Operation::OffSaleNFT {
                collection_id,
                token_id,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::OffSaleNFT {
                    collection_id,
                    token_id,
                },
            )),
            Operation::SetAvatar {
                collection_id,
                token_id,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::SetAvatar {
                    collection_id,
                    token_id,
                },
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
                self.initialize_market(state).await;
                Ok(ExecutionResult::default())
            }
            Message::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => {
                self.create_collection(
                    publisher,
                    base_uri.clone(),
                    price,
                    name.clone(),
                    uris.clone(),
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::CreateCollection {
                        base_uri,
                        price,
                        name,
                        uris,
                        publisher,
                    },
                ))
            }
            Message::MintNFT {
                collection_id,
                uri_index,
                price,
                name,
            } => {
                let owner = context.authenticated_signer.unwrap();
                self.mint_nft(owner, collection_id, uri_index, price, name.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::MintNFT {
                        collection_id,
                        uri_index,
                        price,
                        name,
                    },
                ))
            }
            Message::BuyNFT {
                collection_id,
                token_id,
                credits,
            } => {
                let buyer = context.authenticated_signer.unwrap();
                if context.chain_id == system_api::current_application_id().creation.chain_id {
                    let owner = self.nft_owner(collection_id, token_id).await?;
                    let price = self.nft_price(collection_id, token_id).await?;
                    let fee = self.trading_fee(price).await?;
                    let discount = self.credits_to_tokens(credits).await?;
                    self.transfer_credits(buyer, owner, credits).await?;
                    self.transfer_tokens(
                        buyer,
                        owner,
                        price.saturating_sub(fee).saturating_sub(discount),
                    )
                    .await?;
                    self.deposit_commission(buyer, fee).await?;
                }
                self.buy_nft(buyer, collection_id, token_id).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::BuyNFT {
                        collection_id,
                        token_id,
                        credits,
                    },
                ))
            }
            Message::UpdateCreditsPerLinera { credits_per_linera } => {
                if context.chain_id != system_api::current_application_id().creation.chain_id {
                    return Err(ContractError::OperationNotAllowed);
                }
                self.credits_per_linera.set(credits_per_linera);
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::UpdateCreditsPerLinera { credits_per_linera },
                ))
            }
            Message::UpdateNFTPrice {
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
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::UpdateNFTPrice {
                        collection_id,
                        token_id,
                        price,
                    },
                ))
            }
            Message::OnSaleNFT {
                collection_id,
                token_id,
            } => {
                self.on_sale_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::OnSaleNFT {
                        collection_id,
                        token_id,
                    },
                ))
            }
            Message::OffSaleNFT {
                collection_id,
                token_id,
            } => {
                self.off_sale_nft(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::OffSaleNFT {
                        collection_id,
                        token_id,
                    },
                ))
            }
            Message::SetAvatar {
                collection_id,
                token_id,
            } => {
                self.set_avatar(
                    context.authenticated_signer.unwrap(),
                    collection_id,
                    token_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SetAvatar {
                        collection_id,
                        token_id,
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
                result = result.with_authenticated_message(
                    context.message_id.chain_id,
                    Message::InitialState {
                        state: self.initial_state().await?,
                    },
                );
                return Ok(result);
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
            ApplicationCall::CreateCollection {
                base_uri,
                price,
                name,
                uris,
                publisher,
            } => {
                let mut result = ApplicationCallResult::default();
                result.execution_result = ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::CreateCollection {
                        base_uri,
                        price,
                        name,
                        uris,
                        publisher,
                    },
                );
                Ok(result)
            }
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

impl Market {
    fn credit_app_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Ok(Self::parameters()?.credit_app_id)
    }

    fn foundation_app_id() -> Result<ApplicationId<FoundationAbi>, ContractError> {
        Ok(Self::parameters()?.foundation_app_id)
    }

    async fn transfer_credits(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        let call = credit::ApplicationCall::Transfer { from, to, amount };
        self.call_application(true, Self::credit_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn deposit_commission(
        &mut self,
        from: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        let call = foundation::ApplicationCall::Deposit { from, amount };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn transfer_tokens(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), ContractError> {
        let call = foundation::ApplicationCall::Transfer { from, to, amount };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
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

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,
}
