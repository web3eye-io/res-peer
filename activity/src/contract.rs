#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Activity;
use activity::{ActivityError, Message, Operation};
use async_trait::async_trait;
use linera_sdk::{
    base::{ChannelName, Destination, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};

linera_sdk::contract!(Activity);

impl WithContractAbi for Activity {
    type Abi = activity::ActivityAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

#[async_trait]
impl Contract for Activity {
    type Error = ActivityError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        _argument: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Create { params } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Create { params },
                )),
            Operation::Register {
                activity_id,
                object_id,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Register {
                    activity_id,
                    object_id,
                },
            )),
            Operation::Vote {
                activity_id,
                object_id,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Vote {
                    activity_id,
                    object_id,
                },
            )),
            Operation::Announce {
                activity_id,
                title,
                content,
                announce_prize,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::Announce {
                    activity_id,
                    title,
                    content,
                    announce_prize,
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
            Message::Create { params } => {
                self.create_activity(context.authenticated_signer.unwrap(), params.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Create { params }))
            }
            Message::Register {
                activity_id,
                object_id,
            } => {
                self.register(activity_id, object_id.clone()).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Register {
                        activity_id,
                        object_id,
                    },
                ))
            }
            Message::Vote {
                activity_id,
                object_id,
            } => {
                match self.votable(activity_id).await {
                    Ok(true) => {}
                    Ok(false) => return Err(ActivityError::ActivityNotVotable),
                    Err(err) => return Err(err),
                }
                // TODO: get account balance in foundation
                self.vote(
                    context.authenticated_signer.unwrap(),
                    activity_id,
                    object_id.clone(),
                    1,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::Vote {
                        activity_id,
                        object_id,
                    },
                ))
            }
            Message::Announce {
                activity_id,
                title,
                content,
                announce_prize,
            } => Ok(ExecutionResult::default()),
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
        Err(ActivityError::SessionsNotSupported)
    }
}
