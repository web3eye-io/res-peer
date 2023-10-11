use async_graphql::{Request, Response};
use linera_sdk::base::{Amount, ContractAbi, Owner, ServiceAbi};
use serde::{Deserialize, Serialize};

pub struct FoundationAbi;

impl ContractAbi for FoundationAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = ();
    type Message = ();
    type ApplicationCall = ApplicationCall;
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
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

#[derive(Debug, Deserialize, Serialize)]
pub enum RewardType {
    Review,
    Publish,
    Activity,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApplicationCall {
    Deposit {
        amount: Amount,
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount,
    },
    Reward {
        // Review: sender is the reward user
        // Author: sender is not the reward user
        // Activity: sender is not the reward user
        reward_user: Option<Owner>,
        reward_type: RewardType,
        // For activity we have amount, for other type amount is determined by foundation
        amount: Option<Amount>,
        activity_id: Option<u64>,
    },
    Lock {
        activity_id: u64,
        activity_host: Owner,
        amount: Amount,
    },
}
