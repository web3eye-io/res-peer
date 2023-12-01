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
    type Operation = ();
    type Message = ();
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
    id: u64,
    slogan: Option<String>,
    banner: String,
    posters: Vec<String>,
    introduction: String,
    host: Owner,
    created_at: Timestamp,
    activity_type: ActivityType,
    votable: bool,
    vote_type: VoteType,
    object_type: Option<ObjectType>,
    object_candidates: Vec<String>,
    condition: ObjectCondition,
    sponsors: Vec<Owner>,
    prize_configs: Vec<PrizeConfig>,
    announcements: Vec<String>,
    prize_announcement: String,
    voter_reward_percent: u8,
    vote_counts: HashMap<String, u32>,
    vouters: HashMap<String, Vec<Owner>>,
    budget_amount: Amount,
    join_type: JoinType,
    location: String,
    comments: Vec<String>,
    registers: Vec<Owner>,
    register_start_at: Timestamp,
    register_end_at: Timestamp,
    vote_start_at: Timestamp,
    vote_end_at: Timestamp,
    participantors: Vec<Owner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Create {
        slogan: Option<String>,
        banner: String,
        posters: Vec<String>,
        introduction: String,
        activity_type: ActivityType,
        votable: bool,
        vote_type: VoteType,
        object_type: ObjectType,
        condition: ObjectCondition,
        sponsors: Vec<Owner>,
        prize_configs: Vec<PrizeConfig>,
        voter_reward_percent: u8,
        budget_amount: Amount,
        join_type: JoinType,
        location: String,
        register_start_at: Timestamp,
        register_end_at: Timestamp,
        vote_start_at: Timestamp,
        vote_end_at: Timestamp,
    },
    Register {
        activity_id: u64,
        object_id: Option<String>,
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
}

scalar!(Operation);

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ActivityError {
    #[error("Already registered")]
    AlreadyRegistered,

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
