use std::cmp::Ordering;

use credit::{AgeAmount, AgeAmounts, InitialState};
use linera_sdk::{
    base::{Amount, Owner},
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
    pub amount_alive_ms: RegisterView<u64>,
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
                log::error!(
                    "Supply balance {} reward amount {}",
                    self.balance.get(),
                    amount
                );
                match self.balance.get().cmp(&amount) {
                    Ordering::Less => return Err(StateError::InsufficientSupplyBalance),
                    _ => {}
                }
                amounts.amounts.push(AgeAmount {
                    amount,
                    timestamp: current_system_time(),
                });
                match self.balances.insert(&owner, amounts) {
                    Ok(_) => {
                        self.balance.set(self.balance.get().saturating_sub(amount));
                        Ok(())
                    }
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
                Ok(_) => {
                    self.balance.set(self.balance.get().saturating_sub(amount));
                    Ok(())
                }
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
    }

    pub(crate) async fn liquidate(&mut self) {
        let owners = self.balances.indices().await.unwrap();
        for owner in owners {
            let mut amounts = self.balances.get(&owner).await.unwrap().unwrap();
            for (_, amount) in amounts.amounts.clone().into_iter().enumerate() {
                if current_system_time().saturating_diff_micros(amount.timestamp)
                    > *self.amount_alive_ms.get()
                {
                    self.balance
                        .set(self.balance.get().saturating_add(amount.amount));
                    continue;
                }
            }
            amounts.amounts.retain(|amount| {
                current_system_time().saturating_diff_micros(amount.timestamp)
                    > *self.amount_alive_ms.get()
            });
            self.balances.insert(&owner, amounts).unwrap();
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
