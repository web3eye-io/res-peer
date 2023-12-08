#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::collections::HashMap;

use self::state::Review;
use async_trait::async_trait;
use credit::CreditAbi;
use feed::FeedAbi;
use foundation::FoundationAbi;
use linera_sdk::{
    base::{
        Amount, ApplicationId, ChainId, ChannelName, Destination, Owner, SessionId, WithContractAbi,
    },
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
// use linera_views::views::ViewError;
use market::MarketAbi;
use review::{ApplicationCall, Asset, Content, InitialState, Message, Operation};
use thiserror::Error;

linera_sdk::contract!(Review);

impl WithContractAbi for Review {
    type Abi = review::ReviewAbi;
}

const SUBSCRIPTION_CHANNEL: &[u8] = b"subscriptions";

#[async_trait]
impl Contract for Review {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        state: Self::InitializationArgument,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        self._initialize(state).await?;
        // We should add creator here to be a reviewer, but we keep the message as an test case of application id
        Ok(ExecutionResult::default().with_authenticated_message(
            system_api::current_application_id().creation.chain_id,
            Message::GenesisReviewer,
        ))
    }

    async fn execute_operation(
        &mut self,
        _context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::ApplyReviewer { resume } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApplyReviewer { resume },
                )),
            Operation::UpdateReviewerResume { resume } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::UpdateReviewerResume { resume },
                )),
            Operation::ApproveReviewer { candidate, reason } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApproveReviewer { candidate, reason },
                )),
            Operation::RejectReviewer { candidate, reason } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RejectReviewer { candidate, reason },
                )),
            Operation::SubmitContent {
                cid,
                title,
                content,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::SubmitContent {
                    cid,
                    title,
                    content,
                },
            )),
            Operation::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::ApproveContent {
                    content_cid,
                    reason_cid,
                    reason,
                },
            )),
            Operation::RejectContent {
                content_cid,
                reason,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::RejectContent {
                    content_cid,
                    reason,
                },
            )),
            Operation::SubmitComment {
                cid,
                comment_cid,
                comment,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::SubmitComment {
                    cid,
                    comment_cid,
                    comment,
                },
            )),
            Operation::ApproveAsset { cid, reason } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::ApproveAsset { cid, reason },
                )),
            Operation::RejectAsset { cid, reason } => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RejectAsset { cid, reason },
                )),
            Operation::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::SubmitAsset {
                    cid,
                    base_uri,
                    uris,
                    price,
                    name,
                },
            )),
            Operation::RequestSubscribe => Ok(ExecutionResult::default()
                .with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::RequestSubscribe,
                )),
            Operation::ApproveActivity {
                activity_id,
                reason,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::ApproveActivity {
                    activity_id,
                    reason,
                },
            )),
            Operation::RejectActivity {
                activity_id,
                reason,
            } => Ok(ExecutionResult::default().with_authenticated_message(
                system_api::current_application_id().creation.chain_id,
                Message::RejectActivity {
                    activity_id,
                    reason,
                },
            )),
        }
    }

    async fn execute_message(
        &mut self,
        context: &MessageContext,
        message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match message {
            Message::GenesisReviewer {} => {
                let chain_id = context.chain_id;
                let reviewer = context.authenticated_signer.unwrap();
                self.genesis_reviewer(chain_id, reviewer).await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::GenesisReviewer))
            }
            Message::ExistReviewer { reviewer } => {
                self.add_exist_reviewer(reviewer).await?;
                Ok(ExecutionResult::default())
            }
            Message::ApplyReviewer { resume } => {
                let candidate = context.authenticated_signer.unwrap();
                self._apply_reviewer(context.chain_id, candidate, resume.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::ApplyReviewer { resume }))
            }
            Message::UpdateReviewerResume { resume } => {
                let candidate = context.authenticated_signer.unwrap();
                self._update_reviewer_resume(candidate, resume.clone())
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::UpdateReviewerResume { resume }))
            }
            Message::ApproveReviewer { candidate, reason } => {
                self._approve_reviewer(
                    context.authenticated_signer.unwrap(),
                    candidate,
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::ApproveReviewer { candidate, reason },
                ))
            }
            Message::RejectReviewer { candidate, reason } => {
                self._reject_reviewer(
                    context.authenticated_signer.unwrap(),
                    candidate,
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::RejectReviewer { candidate, reason },
                ))
            }
            Message::SubmitContent {
                cid,
                title,
                content,
            } => {
                let author = context.authenticated_signer.unwrap();
                self._submit_content(
                    cid.clone(),
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
                    Message::SubmitContent {
                        cid,
                        title,
                        content,
                    },
                ))
            }
            Message::ApproveContent {
                content_cid,
                reason_cid,
                reason,
            } => {
                let reviewer = context.authenticated_signer.unwrap();
                self._approve_content(
                    reviewer,
                    content_cid.clone(),
                    reason_cid.clone(),
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::ApproveContent {
                        content_cid,
                        reason_cid,
                        reason,
                    },
                ))
            }
            Message::RejectContent {
                content_cid,
                reason,
            } => {
                let reviewer = context.authenticated_signer.unwrap();
                self._reject_content(
                    reviewer,
                    content_cid.clone(),
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::RejectContent {
                        content_cid,
                        reason,
                    },
                ))
            }
            Message::SubmitComment {
                cid,
                comment_cid,
                comment,
            } => {
                let author = context.authenticated_signer.unwrap();
                self._submit_comment(
                    comment_cid.clone(),
                    cid.clone(),
                    comment.clone(),
                    author,
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SubmitComment {
                        cid,
                        comment_cid,
                        comment,
                    },
                ))
            }
            Message::ApproveAsset { cid, reason } => {
                self._approve_asset(
                    context.authenticated_signer.unwrap(),
                    cid.clone(),
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::ApproveAsset { cid, reason }))
            }
            Message::RejectAsset { cid, reason } => {
                self._reject_asset(
                    context.authenticated_signer.unwrap(),
                    cid.clone(),
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default()
                    .with_authenticated_message(dest, Message::RejectAsset { cid, reason }))
            }
            Message::SubmitAsset {
                cid,
                base_uri,
                uris,
                price,
                name,
            } => {
                self._submit_asset(
                    context.authenticated_signer.unwrap(),
                    cid.clone(),
                    base_uri.clone(),
                    uris.clone(),
                    price.clone(),
                    name.clone(),
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SubmitAsset {
                        cid,
                        base_uri,
                        uris,
                        price,
                        name,
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
                /* Will be stuck in the for each so we use indices */
                for reviewer in self.reviewers.indices().await? {
                    let reviewer = self.reviewers.get(&reviewer).await?.unwrap();
                    result = result.with_authenticated_message(
                        context.message_id.chain_id,
                        Message::ExistReviewer { reviewer },
                    );
                }
                result = result.with_authenticated_message(
                    context.message_id.chain_id,
                    Message::InitialState {
                        state: self.initial_state().await?,
                    },
                );
                Ok(result)
            }
            Message::InitialState { state } => {
                self.initialize_review(state).await?;
                Ok(ExecutionResult::default())
            }
            Message::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            } => {
                self._submit_activity(activity_id, activity_host, budget_amount)
                    .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::SubmitActivity {
                        activity_id,
                        activity_host,
                        budget_amount,
                    },
                ))
            }
            Message::ApproveActivity {
                activity_id,
                reason,
            } => {
                self._approve_activity(
                    context.authenticated_signer.unwrap(),
                    activity_id,
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::ApproveActivity {
                        activity_id,
                        reason,
                    },
                ))
            }
            Message::RejectActivity {
                activity_id,
                reason,
            } => {
                self._reject_activity(
                    context.authenticated_signer.unwrap(),
                    activity_id,
                    reason.clone(),
                    context.chain_id == system_api::current_application_id().creation.chain_id,
                )
                .await?;
                let dest =
                    Destination::Subscribers(ChannelName::from(SUBSCRIPTION_CHANNEL.to_vec()));
                Ok(ExecutionResult::default().with_authenticated_message(
                    dest,
                    Message::RejectActivity {
                        activity_id,
                        reason,
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
        match call {
            ApplicationCall::SubmitContent {
                cid,
                title,
                content,
            } => {
                let mut result = ApplicationCallResult::default();
                result.execution_result = ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::SubmitContent {
                        cid,
                        title,
                        content,
                    },
                );
                Ok(result)
            }
            ApplicationCall::SubmitActivity {
                activity_id,
                activity_host,
                budget_amount,
            } => {
                let mut result = ApplicationCallResult::default();
                result.execution_result = ExecutionResult::default().with_authenticated_message(
                    system_api::current_application_id().creation.chain_id,
                    Message::SubmitActivity {
                        activity_id,
                        activity_host,
                        budget_amount,
                    },
                );
                Ok(result)
            }
            ApplicationCall::ActivityApproved { activity_id } => {
                let mut result = ApplicationCallResult::default();
                result.value = self.activity_approved(activity_id).await?;
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

impl Review {
    fn feed_app_id() -> Result<ApplicationId<FeedAbi>, ContractError> {
        Ok(Self::parameters()?.feed_app_id)
    }

    fn credit_app_id() -> Result<ApplicationId<CreditAbi>, ContractError> {
        Ok(Self::parameters()?.credit_app_id)
    }

    fn foundation_app_id() -> Result<ApplicationId<FoundationAbi>, ContractError> {
        Ok(Self::parameters()?.foundation_app_id)
    }

    fn market_app_id() -> Result<ApplicationId<MarketAbi>, ContractError> {
        Ok(Self::parameters()?.market_app_id)
    }

    async fn reward_credits(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        let call = credit::ApplicationCall::Reward { owner, amount };
        self.call_application(true, Self::credit_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn reward_tokens(&mut self) -> Result<(), ContractError> {
        let call = foundation::ApplicationCall::Reward {
            reward_user: None,
            reward_type: foundation::RewardType::Review,
            activity_id: None,
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn publish_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
    ) -> Result<(), ContractError> {
        let call = feed::ApplicationCall::Publish {
            cid: cid.clone(),
            title,
            content,
            author,
        };
        self.call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn recommend_content(
        &mut self,
        cid: String,
        reason_cid: String,
        reason: String,
    ) -> Result<(), ContractError> {
        let call = feed::ApplicationCall::Recommend {
            cid: cid.clone(),
            reason_cid: reason_cid.clone(),
            reason,
        };
        self.call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn comment_content(
        &mut self,
        cid: String,
        comment_cid: String,
        comment: String,
        commentor: Owner,
    ) -> Result<(), ContractError> {
        let call = feed::ApplicationCall::Comment {
            cid: cid.clone(),
            comment_cid: comment_cid.clone(),
            comment,
            commentor,
        };
        self.call_application(true, Self::feed_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn create_collection(
        &mut self,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
        publisher: Owner,
    ) -> Result<(), ContractError> {
        let call = market::ApplicationCall::CreateCollection {
            base_uri: base_uri.clone(),
            price,
            name,
            uris,
            publisher,
        };
        self.call_application(true, Self::market_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn lock_activity_funds(
        &mut self,
        activity_id: u64,
        budget_amount: Amount,
    ) -> Result<(), ContractError> {
        let call = foundation::ApplicationCall::Lock {
            activity_id,
            amount: budget_amount,
        };
        self.call_application(true, Self::foundation_app_id()?, &call, vec![])
            .await?;
        Ok(())
    }

    async fn _initialize(&mut self, state: InitialState) -> Result<(), ContractError> {
        self.initialize_review(state).await?;
        Ok(())
    }

    async fn _apply_reviewer(
        &mut self,
        chain_id: ChainId,
        candidate: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.apply_reviewer(chain_id, candidate, resume).await?;
        Ok(())
    }

    async fn _update_reviewer_resume(
        &mut self,
        reviewer: Owner,
        resume: String,
    ) -> Result<(), ContractError> {
        self.update_reviewer_resume(reviewer, resume).await?;
        Ok(())
    }

    async fn _approve_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let _reviewer = self
            .approve_reviewer(reviewer, candidate, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match _reviewer {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_reviewer(
        &mut self,
        reviewer: Owner,
        candidate: Owner,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let _reviewer = self
            .reject_reviewer(reviewer, candidate, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match _reviewer {
            Some(_reviewer) => {
                // TODO: notify candidate is reviewer
            }
            _ => {
                // TODO: notify candidate is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(100))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _submit_content(
        &mut self,
        cid: String,
        title: String,
        content: String,
        author: Owner,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        self.submit_content(Content {
            // TODO: notify author
            cid,
            comment_to_cid: None,
            title,
            content,
            author,
            reviewers: HashMap::default(),
            approved: 0,
            rejected: 0,
            created_at: system_api::current_system_time(),
        })
        .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(author, Amount::from_tokens(10)).await?;
        Ok(())
    }

    async fn _submit_comment(
        &mut self,
        cid: String,
        comment_to_cid: String,
        comment: String,
        author: Owner,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        self.submit_content(Content {
            cid,
            comment_to_cid: Some(comment_to_cid),
            title: String::default(),
            content: comment,
            author,
            reviewers: HashMap::default(),
            approved: 0,
            rejected: 0,
            created_at: system_api::current_system_time(),
        })
        .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(author, Amount::from_tokens(10)).await?;
        Ok(())
    }

    async fn _approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason_cid: Option<String>,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let content = self
            .approve_content(
                reviewer,
                content_cid.clone(),
                reason.clone().unwrap_or_default(),
            )
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match content {
            Some(content) => {
                match content.comment_to_cid {
                    Some(comment_to_cid) => {
                        self.comment_content(
                            comment_to_cid,
                            content.cid.clone(),
                            content.content,
                            content.author,
                        )
                        .await?;
                    }
                    _ => {
                        self.publish_content(
                            content.cid,
                            content.title,
                            content.content,
                            content.author,
                        )
                        .await?
                    }
                }
                match reason_cid {
                    Some(cid) => {
                        self.recommend_content(content_cid.clone(), cid, reason.unwrap_or_default())
                            .await?
                    }
                    _ => {}
                }
                // TODO: notify author content is published
            }
            _ => {
                // TODO: notify author content is approved
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let content = self
            .reject_content(reviewer, content_cid, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match content {
            Some(_content) => {
                // TODO: notify author content is rejected
            }
            _ => {
                // TODO: notify author content is rejected
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _approve_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let asset = self
            .approve_asset(reviewer, cid, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match asset {
            Some(asset) => {
                self.create_collection(
                    asset.base_uri,
                    asset.uris,
                    asset.price,
                    asset.name,
                    asset.author,
                )
                .await?;
                // TODO: notify author is approved
            }
            _ => {
                // TODO: notify author
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _reject_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let asset = self
            .reject_asset(reviewer, cid, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        match asset {
            Some(_asset) => {
                // TODO: notify author is approved
            }
            _ => {
                // TODO: notify author
            }
        }
        self.reward_credits(reviewer, Amount::from_tokens(50))
            .await?;
        self.reward_tokens().await?;
        Ok(())
    }

    async fn _submit_asset(
        &mut self,
        author: Owner,
        cid: String,
        base_uri: String,
        uris: Vec<String>,
        price: Option<Amount>,
        name: String,
    ) -> Result<(), ContractError> {
        self.submit_asset(Asset {
            cid,
            author,
            base_uri,
            uris,
            price,
            name,
            reviewers: HashMap::default(),
            approved: 0,
            rejected: 0,
            created_at: system_api::current_system_time(),
        })
        .await?;
        Ok(())
    }

    async fn _submit_activity(
        &mut self,
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    ) -> Result<(), ContractError> {
        self.submit_activity(activity_id, activity_host, budget_amount)
            .await?;
        Ok(())
    }

    async fn _approve_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: Option<String>,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let activity = self
            .approve_activity(owner, activity_id, reason.unwrap_or_default())
            .await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(owner, Amount::from_tokens(50)).await?;
        self.reward_tokens().await?;
        if let Some(activity) = activity {
            self.lock_activity_funds(activity_id, activity.budget_amount)
                .await?;
        }
        Ok(())
    }

    async fn _reject_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: String,
        creation_chain: bool,
    ) -> Result<(), ContractError> {
        let _activity = self.reject_activity(owner, activity_id, reason).await?;
        if !creation_chain {
            return Ok(());
        }
        self.reward_credits(owner, Amount::from_tokens(50)).await?;
        self.reward_tokens().await?;
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
    #[error(transparent)]
    StateError(#[from] state::StateError),

    #[error("Invalid user")]
    InvalidUser,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
