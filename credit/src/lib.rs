use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::base::{Amount, ContractAbi, ServiceAbi, Timestamp, Owner};
use serde::{Deserialize, Serialize};

pub struct CreditAbi;

impl ContractAbi for CreditAbi {
    type Parameters = ();
    type InitializationArgument = InitialState;
    type Operation = ();
    type Message = ();
    type ApplicationCall = ();
    type SessionCall = ();
    type SessionState = ();
    type Response = ();
}

impl ServiceAbi for CreditAbi {
    type Parameters = ();
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct AgeAmount {
    pub amount: Amount,
    pub timestamp: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct AgeAmounts {
    pub amounts: Vec<AgeAmount>,
}

impl AgeAmounts {
    pub fn sum(&self) -> Amount {
        let mut _sum = Amount::zero();
        self.amounts
            .iter()
            .for_each(|a| _sum = _sum.try_add(a.amount).unwrap());
        _sum
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InitialState {
    pub initial_supply: Amount,
    pub amount_alive_ms: Timestamp,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Liquidate,
    Reward {
        owner: Owner,
        amount: Amount
    },
    Transfer {
        from: Owner,
        to: Owner,
        amount: Amount
    }
}
