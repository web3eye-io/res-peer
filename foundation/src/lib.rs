use std::collections::HashSet;

use async_graphql::{Request, Response};
use linera_sdk::base::{Amount, ContractAbi, Owner, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct FoundationAbi;

impl ContractAbi for FoundationAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = Operation;
    type Message = Message;
    type ApplicationCall = ApplicationCall;
    type SessionCall = ();
    type SessionState = ();
    type Response = Amount;
}

impl ServiceAbi for FoundationAbi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub review_reward_percent: u8,
    pub review_reward_factor: u8,
    pub author_reward_percent: u8,
    pub author_reward_factor: u8,
    pub activity_reward_percent: u8,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum RewardType {
    Review,
    Publish,
    Activity,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    UserDeposit { amount: Amount },
    RequestSubscribe,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationCall {
    Deposit {
        from: Owner,
        amount: Amount,
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount,
    },
    Reward {
        // Review: sender is the reward user (the reviewer)
        // Author: sender is not the reward user but the reviewer
        // Activity: sender must be the activity host
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    },
    ActivityRewards {
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    },
    Lock {
        activity_id: u64,
        amount: Amount,
    },
    Balance {
        owner: Owner,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    UserDeposit {
        amount: Amount,
    },
    RequestSubscribe,
    InitialState {
        state: InitialState,
    },
    Deposit {
        from: Owner,
        amount: Amount,
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount,
    },
    Reward {
        // Review: sender is the reward user (the reviewer)
        // Author: sender is not the reward user but the reviewer
        // Activity: sender must be the activity host
        // TODO: for activity host reward we should review it
        reward_user: Option<Owner>,
        reward_type: RewardType,
        activity_id: Option<u64>,
    },
    ActivityRewards {
        activity_id: u64,
        winner_user: Owner,
        voter_users: HashSet<Owner>,
        reward_amount: Amount,
        voter_reward_percent: u8,
    },
    Lock {
        activity_id: u64,
        amount: Amount,
    },
}
