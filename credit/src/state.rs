use std::cmp::Ordering;

use credit::{AgeAmount, AgeAmounts, InitialState};
use linera_sdk::{
    base::{Amount, Owner, Timestamp},
    contract::system_api::current_system_time,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Credit {
    pub initial_supply: RegisterView<Amount>,
    pub balance: RegisterView<Amount>,
    pub amount_alive_ms: RegisterView<Timestamp>,
    pub balances: MapView<Owner, AgeAmounts>,
}

#[allow(dead_code)]
impl Credit {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.initial_supply.set(state.initial_supply);
        self.balance.set(state.initial_supply);
        self.amount_alive_ms.set(state.amount_alive_ms);
    }

    pub(crate) async fn initial_supply(&self) -> Amount {
        *self.initial_supply.get()
    }

    pub(crate) async fn balance(&self, owner: Option<Owner>) -> Amount {
        match owner {
            Some(owner) => self.balances.get(&owner).await.unwrap().unwrap().sum(),
            None => *self.balance.get(),
        }
    }

    pub(crate) async fn reward(&mut self, owner: Owner, amount: Amount) -> Result<(), StateError> {
        match self.balances.get(&owner).await {
            Ok(Some(mut amounts)) => {
                match self.balance.get().cmp(&amount) {
                    Ordering::Less => return Err(StateError::InsufficientSupplyBalance),
                    _ => {}
                }
                amounts.amounts.push(AgeAmount {
                    amount,
                    timestamp: current_system_time(),
                });
                match self.balances.insert(&owner, amounts) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StateError::ViewError(err)),
                }
            }
            _ => match self.balances.insert(
                &owner,
                AgeAmounts {
                    amounts: vec![AgeAmount {
                        amount,
                        timestamp: current_system_time(),
                    }],
                },
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Insufficient supply balance")]
    InsufficientSupplyBalance,

    #[error("Insufficient account balance")]
    _InsufficientAccountBalance,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
