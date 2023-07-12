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
    pub amount_alive_ms: RegisterView<u64>,
    pub balances: MapView<Owner, AgeAmounts>,
    pub spendables: MapView<Owner, Amount>,
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
        match self.spendables.get(&owner).await {
            Ok(Some(spendable)) => {
                self.spendables
                    .insert(&owner, spendable.saturating_add(amount))
                    .unwrap();
            }
            _ => {
                self.spendables.insert(&owner, amount).unwrap();
            }
        }

        log::info!(
            "Supply balance {} reward amount {}",
            self.balance.get(),
            amount
        );
        match self.balance.get().cmp(&amount) {
            Ordering::Less => return Err(StateError::InsufficientSupplyBalance),
            _ => {}
        }

        self.balance.set(self.balance.get().saturating_sub(amount));

        match self.balances.get(&owner).await {
            Ok(Some(mut amounts)) => {
                amounts.amounts.push(AgeAmount {
                    amount,
                    expired: Timestamp::from(
                        current_system_time()
                            .micros()
                            .saturating_add(*self.amount_alive_ms.get()),
                    ),
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
                        expired: Timestamp::from(
                            current_system_time()
                                .micros()
                                .saturating_add(*self.amount_alive_ms.get()),
                        ),
                    }],
                },
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
    }

    pub(crate) async fn liquidate(&mut self) {
        let owners = self.balances.indices().await.unwrap();
        for owner in owners {
            let mut amounts = self.balances.get(&owner).await.unwrap().unwrap();
            for (_, amount) in amounts.amounts.clone().into_iter().enumerate() {
                if current_system_time().saturating_diff_micros(amount.expired) == 0 {
                    self.balance
                        .set(self.balance.get().saturating_add(amount.amount));
                    self.spendables
                        .insert(
                            &owner,
                            self.spendables
                                .get(&owner)
                                .await
                                .unwrap()
                                .unwrap()
                                .saturating_sub(amount.amount),
                        )
                        .unwrap();
                    continue;
                }
            }
            amounts
                .amounts
                .retain(|amount| current_system_time().saturating_diff_micros(amount.expired) == 0);
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
