use std::collections::HashMap;

use async_graphql::{Enum, SimpleObject};
use linera_sdk::base::{Amount, ContractAbi, Owner, ServiceAbi, Timestamp};
use serde::{Deserialize, Serialize};

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
    type Query = ();
    type QueryResponse = ();
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
    cids: Option<Vec<String>>,
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
    condition: ObjectCondition,
    sponsors: Vec<u64>,
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
    participantors: Vec<Owner>,
}
