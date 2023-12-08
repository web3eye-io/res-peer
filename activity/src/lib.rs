use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
};

use async_graphql::{scalar, Enum, InputObject, Request, Response, SimpleObject};
use linera_sdk::base::{
    Amount, ApplicationId, ArithmeticError, ContractAbi, Owner, ServiceAbi, Timestamp,
};
use linera_views::views::ViewError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct ActivityAbi;

impl ContractAbi for ActivityAbi {
    type Parameters = ActivityParameters;
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for ActivityAbi {
    type Parameters = ActivityParameters;
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityParameters {
    pub review_app_id: ApplicationId<review::ReviewAbi>,
    pub foundation_app_id: ApplicationId<foundation::FoundationAbi>,
    pub feed_app_id: ApplicationId<feed::FeedAbi>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum ActivityType {
    MeetUp,
    Campaign,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum VoteType {
    Account,
    Power,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum ObjectType {
    Content,
    Comment,
    Author,
    Reviewer,
    ArtWork,
    ArtCollection,
    Creator,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct ObjectCondition {
    classes: Option<Vec<String>>,
    min_words: u32,
    max_words: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq, InputObject)]
pub struct PrizeConfig {
    pub place: u16,
    pub medal: String,
    pub title: String,
    pub reward_amount: Option<Amount>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum JoinType {
    Online,
    InPerson,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct Winner {
    pub place: u16,
    pub object_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct ActivityItem {
    pub id: u64,
    pub title: String,
    pub slogan: Option<String>,
    pub banner: String,
    pub posters: Vec<String>,
    pub introduction: String,
    pub host: Owner,
    pub host_resume: String,
    pub created_at: Timestamp,
    pub activity_type: ActivityType,
    pub votable: bool,
    pub vote_type: VoteType,
    pub object_type: Option<ObjectType>,
    pub object_candidates: HashSet<String>,
    pub condition: ObjectCondition,
    pub sponsors: Vec<Owner>,
    pub prize_configs: Vec<PrizeConfig>,
    pub announcements: HashSet<String>,
    pub prize_announcement: String,
    pub voter_reward_percent: u8,
    pub vote_powers: HashMap<String, Amount>,
    pub voters: HashMap<String, HashSet<Owner>>,
    pub budget_amount: Amount,
    pub join_type: JoinType,
    pub location: String,
    pub comments: HashSet<String>,
    pub registers: HashSet<Owner>,
    pub register_start_at: Timestamp,
    pub register_end_at: Timestamp,
    pub vote_start_at: Timestamp,
    pub vote_end_at: Timestamp,
    pub participantors: HashSet<Owner>,
    pub winners: Vec<Winner>,
    pub finalized: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateParams {
    pub title: String,
    pub slogan: Option<String>,
    pub banner: String,
    pub host_resume: String,
    pub posters: Vec<String>,
    pub introduction: String,
    pub activity_type: ActivityType,
    pub votable: bool,
    pub vote_type: VoteType,
    pub object_type: ObjectType,
    pub condition: ObjectCondition,
    pub sponsors: Vec<Owner>,
    pub prize_configs: Vec<PrizeConfig>,
    pub voter_reward_percent: u8,
    pub budget_amount: Amount,
    pub join_type: JoinType,
    pub location: String,
    pub register_start_at: Timestamp,
    pub register_end_at: Timestamp,
    pub vote_start_at: Timestamp,
    pub vote_end_at: Timestamp,
}

scalar!(CreateParams);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateParams {
    pub activity_id: u64,
    pub title: Option<String>,
    pub slogan: Option<String>,
    pub banner: Option<String>,
    pub host_resume: Option<String>,
    pub posters: Option<Vec<String>>,
    pub introduction: Option<String>,
    pub activity_type: Option<ActivityType>,
    pub votable: Option<bool>,
    pub vote_type: Option<VoteType>,
    pub object_type: Option<ObjectType>,
    pub condition: Option<ObjectCondition>,
    pub sponsors: Option<Vec<Owner>>,
    pub prize_configs: Option<Vec<PrizeConfig>>,
    pub voter_reward_percent: Option<u8>,
    pub budget_amount: Option<Amount>,
    pub join_type: Option<JoinType>,
    pub location: Option<String>,
    pub register_start_at: Option<Timestamp>,
    pub register_end_at: Option<Timestamp>,
    pub vote_start_at: Option<Timestamp>,
    pub vote_end_at: Option<Timestamp>,
}

scalar!(UpdateParams);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnnounceParams {
    pub activity_id: u64,
    pub cid: String,
    pub title: String,
    pub content: String,
    pub announce_prize: bool,
}

scalar!(AnnounceParams);

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Create { params: CreateParams },
    Update { params: UpdateParams },
    Register { activity_id: u64, object_id: String },
    Vote { activity_id: u64, object_id: String },
    Announce { params: AnnounceParams },
    RequestSubscribe,
    Finalize { activity_id: u64 },
}

scalar!(Operation);

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Create { params: CreateParams },
    Update { params: UpdateParams },
    Register { activity_id: u64, object_id: String },
    Vote { activity_id: u64, object_id: String },
    Announce { params: AnnounceParams },
    RequestSubscribe,
    Finalize { activity_id: u64 },
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ActivityError {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid activity")]
    InvalidActivity,

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error("Activity not votable")]
    ActivityNotVotable,

    #[error("Activity not approved")]
    ActivityNotApproved,

    #[error("Activity object not found")]
    ActivityObjectNotFound,

    #[error("Activity object already voted")]
    ActivityObjectAlreadyVoted,

    #[error("Activity announcement already created")]
    ActivityAnnouncementAlreadyCreated,

    #[error("Only host can finalize activity")]
    NotActivityHost,

    #[error("Activity already finalized")]
    ActivityAlreadyFinalized,

    #[error("Account balance required")]
    AccountBalanceRequired,

    #[error("Invalid content author")]
    InvalidContentAuthor,

    #[error("Invalid prize config")]
    InvalidPrizeConfig,

    #[error("Invalid query")]
    InvalidQuery(#[from] serde_json::Error),

    #[error(transparent)]
    BcsError(#[from] bcs::Error),

    #[error(transparent)]
    ViewError(#[from] ViewError),

    #[error(transparent)]
    ArithmeticError(#[from] ArithmeticError),

    #[error(transparent)]
    Infallible(#[from] Infallible),
}
