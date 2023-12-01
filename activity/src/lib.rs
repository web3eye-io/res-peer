use std::{collections::HashMap, convert::Infallible};

use async_graphql::{scalar, Enum, Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ArithmeticError, ContractAbi, Owner, ServiceAbi, Timestamp};
use linera_views::views::ViewError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct ActivityAbi;

impl ContractAbi for ActivityAbi {
    type Parameters = ();
    type InitializationArgument = ();
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for ActivityAbi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
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
    class: Option<Vec<String>>,
    min_words: u32,
    max_words: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct PrizeConfig {
    place: u16,
    medal: String,
    title: String,
    reward_amount: Option<Amount>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum JoinType {
    Online,
    InPerson,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Eq, PartialEq)]
pub struct ActivityItem {
    pub id: u64,
    pub slogan: Option<String>,
    pub banner: String,
    pub posters: Vec<String>,
    pub introduction: String,
    pub host: Owner,
    pub created_at: Timestamp,
    pub activity_type: ActivityType,
    pub votable: bool,
    pub vote_type: VoteType,
    pub object_type: Option<ObjectType>,
    pub object_candidates: HashMap<String, bool>,
    pub condition: ObjectCondition,
    pub sponsors: Vec<Owner>,
    pub prize_configs: Vec<PrizeConfig>,
    pub announcements: Vec<String>,
    pub prize_announcement: String,
    pub voter_reward_percent: u8,
    pub vote_counts: HashMap<String, u32>,
    pub voters: HashMap<String, Vec<Owner>>,
    pub budget_amount: Amount,
    pub join_type: JoinType,
    pub location: String,
    pub comments: Vec<String>,
    pub registers: Vec<Owner>,
    pub register_start_at: Timestamp,
    pub register_end_at: Timestamp,
    pub vote_start_at: Timestamp,
    pub vote_end_at: Timestamp,
    pub participantors: Vec<Owner>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateParams {
    pub slogan: Option<String>,
    pub banner: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Create {
        params: CreateParams,
    },
    Register {
        activity_id: u64,
        object_id: String,
    },
    Vote {
        activity_id: u64,
        object_id: String,
    },
    Announce {
        activity_id: u64,
        title: String,
        content: String,
        announce_prize: bool,
    },
    RequestSubscribe,
}

scalar!(Operation);

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Create {
        params: CreateParams,
    },
    Register {
        activity_id: u64,
        object_id: String,
    },
    Vote {
        activity_id: u64,
        object_id: String,
    },
    Announce {
        activity_id: u64,
        title: String,
        content: String,
        announce_prize: bool,
    },
    RequestSubscribe,
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ActivityError {
    #[error("Already registered")]
    AlreadyRegistered,

    #[error("Invalid activity")]
    InvalidActivity,

    #[error("Invalid query")]
    InvalidQuery(#[from] serde_json::Error),

    #[error("Cross-application sessions not supported")]
    SessionsNotSupported,

    #[error(transparent)]
    BcsError(#[from] bcs::Error),

    #[error(transparent)]
    ViewError(#[from] ViewError),

    #[error(transparent)]
    ArithmeticError(#[from] ArithmeticError),

    #[error(transparent)]
    Infallible(#[from] Infallible),
}
