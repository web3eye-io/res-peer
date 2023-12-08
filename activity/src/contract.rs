#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashSet;

use self::state::Activity;
use activity::{ActivityError, AnnounceParams, CreateParams, Message, Operation, VoteType};
use async_trait::async_trait;
use feed::FeedAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{Amount, ApplicationId, ChannelName, Destination, Owner, SessionId, WithContractAbi},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use review::ReviewAbi;

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
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Create { params } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Create { params },
                )),
            Operation::Update { params } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Update { params },
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
            Operation::Announce { params } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Announce { params },
                )),
            Operation::RequestSubscribe => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RequestSubscribe,
                )),
            Operation::Finalize { activity_id } => {
                let activity = self.activity(activity_id).await?;
                if activity.host != context.authenticated_signer.unwrap() {
                    return Err(ActivityError::NotActivityHost);
                }
                Ok(ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::Finalize { activity_id },
                ))
            }
        }
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::Create { params } => {
                self._create_activity(context.authenticated_signer.unwrap(), params.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Create { params }))
            }
            Message::Update { params } => {
                self.update_activity(params.clone()).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Update { params }))
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
                match self.activity_approved(activity_id).await {
                    Ok(true) => {}
                    Ok(false) => return Err(ActivityError::ActivityNotApproved),
                    Err(err) => return Err(err),
                }
                match self.votable(activity_id).await {
                    Ok(true) => {}
                    Ok(false) => return Err(ActivityError::ActivityNotVotable),
                    Err(err) => return Err(err),
                }
                let balance = self
                    .account_balance(context.authenticated_signer.unwrap())
                    .await?;
                let activity = self.activity(activity_id).await?;
                let power = match activity.vote_type {
                    VoteType::Power => balance,
                    VoteType::Account => Amount::ONE,
                };
                if power.eq(&Amount::ZERO) {
                    return Err(ActivityError::AccountBalanceRequired);
                }
                self.vote(
                    context.authenticated_signer.unwrap(),
                    activity_id,
                    object_id.clone(),
                    power,
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
            Message::Announce { params } => {
                self.create_announcement(params.clone()).await?;
                self.announce(
                    params.activity_id,
                    params.cid.clone(),
                    params.announce_prize,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Announce { params }))
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
            Message::Finalize { activity_id } => {
                self._finalize(activity_id).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::Finalize { activity_id }))
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

impl Activity {
    fn review_app_id() -> Result<ApplicationId<ReviewAbi>, ActivityError> {
        Ok(Self::parameters().unwrap().review_app_id)
    }

    fn foundation_app_id() -> Result<ApplicationId<FoundationAbi>, ActivityError> {
        Ok(Self::parameters().unwrap().foundation_app_id)
    }

    fn feed_app_id() -> Result<ApplicationId<FeedAbi>, ActivityError> {
        Ok(Self::parameters().unwrap().feed_app_id)
    }

    async fn create_announcement(&mut self, params: AnnounceParams) -> Result<(), ActivityError> {
        let call = review::ApplicationCall::SubmitContent {
            cid: params.cid,
            title: params.title,
            content: params.content,
        };
        self.call_application(true, Self::review_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn account_balance(&mut self, owner: Owner) -> Result<Amount, ActivityError> {
        let call = foundation::ApplicationCall::Balance { owner };
        let (resp, _) = self
            .call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(resp)
    }

    async fn _create_activity(
        &mut self,
        owner: Owner,
        params: CreateParams,
    ) -> Result<(), ActivityError> {
        let activity_id = self.create_activity(owner, params.clone()).await?;
        let call = review::ApplicationCall::SubmitActivity {
            activity_id,
            activity_host: owner,
            budget_amount: params.budget_amount,
        };
        self.call_application(true, Self::review_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn activity_approved(&mut self, activity_id: u64) -> Result<bool, ActivityError> {
        let call = review::ApplicationCall::ActivityApproved { activity_id };
        let (approved, _) = self
            .call_application(true, Self::review_app_id()?, &call, vec![])
            .await?;
        Ok(approved)
    }

    async fn content_author(&mut self, cid: String) -> Result<Owner, ActivityError> {
        let call = feed::ApplicationCall::ContentAuthor { cid };
        let (author, _) = self
            .call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        match author {
            Some(author) => Ok(author),
            _ => Err(ActivityError::InvalidContentAuthor),
        }
    }

    async fn activity_rewards(
        &mut self,
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    ) -> Result<(), ActivityError> {
        let call = foundation::ApplicationCall::ActivityRewards {
            activity_id,
            winner_user,
            voter_users,
            reward_amount,
            voter_reward_percent,
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn reward_activity_host(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        let call = foundation::ApplicationCall::Reward {
            reward_user: None,
            reward_type: foundation::RewardType::Activity,
            activity_id: Some(activity_id),
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn _finalize(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        self.finalize(activity_id).await?;
        let activity = self.activity(activity_id).await?;
        for winner in activity.winners {
            let author = self.content_author(winner.clone().object_id).await?;
            let voter_users = activity.voters.get(&winner.object_id).unwrap().clone();
            let index = match activity
                .prize_configs
                .iter()
                .position(|r| r.place == winner.place)
            {
                Some(index) => index,
                _ => return Err(ActivityError::InvalidPrizeConfig),
            };
            let reward_amount = match activity.prize_configs[index].reward_amount {
                Some(amount) => amount,
                _ => Amount::ZERO,
            };
            self.activity_rewards(
                activity_id,
                author,
                voter_users,
                reward_amount,
                activity.voter_reward_percent,
            )
            .await?;
        }
        self.reward_activity_host(activity_id).await?;
        Ok(())
    }
}
