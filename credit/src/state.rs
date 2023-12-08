use std::cmp::Ordering;

use credit::{AgeAmount, AgeAmounts, InitialState};
use linera_sdk::{
    base::{Amount, ApplicationId, Owner, Timestamp},
    contract::system_api::current_system_time,
    views::{MapView, RegisterView, SetView, ViewStorageContext},
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
    pub reward_callers: SetView<ApplicationId>,
    pub transfer_callers: SetView<ApplicationId>,
}

#[allow(dead_code)]
impl Credit {
    pub(crate) async fn initialize_credit(&mut self, mut state: InitialState) {
        if state.initial_supply.eq(&Amount::ZERO) {
            state.initial_supply = Amount::from_tokens(100000000);
        }
        self.initial_supply.set(state.initial_supply);
        self.balance.set(state.initial_supply);
        self.amount_alive_ms.set(state.amount_alive_ms);
    }

    pub(crate) async fn initial_state(&self) -> Result<InitialState, StateError> {
        Ok(InitialState {
            initial_supply: *self.initial_supply.get(),
            amount_alive_ms: *self.amount_alive_ms.get(),
        })
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

        match self.balance.get().cmp(&amount) {
            Ordering::Less => {
                log::error!(
                    "Here we should correct: supply balance {} reward amount {}",
                    self.balance.get(),
                    amount
                );
                // return Err(StateError::InsufficientSupplyBalance)
            }
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
            let mut amounts = match self.balances.get(&owner).await {
                Ok(Some(amounts)) => amounts,
                _ => continue,
            };
            let mut spendable = match self.spendables.get(&owner).await {
                Ok(Some(spendable)) => spendable,
                _ => continue,
            };
            amounts.amounts.retain(|amount| {
                let expired = current_system_time().saturating_diff_micros(amount.expired) > 0;
                if expired {
                    self.balance
                        .set(self.balance.get().saturating_add(amount.amount));
                    spendable = spendable.saturating_sub(amount.amount);
                }
                !expired
            });
            self.spendables.insert(&owner, spendable).unwrap();
            self.balances.insert(&owner, amounts).unwrap();
        }
    }

    pub(crate) async fn set_reward_callers(&mut self, application_ids: Vec<ApplicationId>) {
        application_ids
            .iter()
            .for_each(|application_id| self.reward_callers.insert(application_id).unwrap())
    }

    pub(crate) async fn set_transfer_callers(&mut self, application_ids: Vec<ApplicationId>) {
        application_ids
            .iter()
            .for_each(|application_id| self.transfer_callers.insert(application_id).unwrap())
    }

    pub(crate) async fn transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), StateError> {
        match self.spendables.get(&from).await {
            Ok(Some(spendable)) => match spendable.cmp(&amount) {
                Ordering::Less => Err(StateError::InsufficientAccountBalance),
                _ => {
                    self.spendables
                        .insert(&from, spendable.saturating_sub(amount))?;
                    let mut amounts = self.balances.get(&from).await.unwrap().unwrap();
                    let mut total: Amount = Amount::ZERO;
                    let mut remain: Option<AgeAmount> = None;
                    amounts.amounts.retain(|_amount| {
                        if total.ge(&amount) {
                            return true;
                        }
                        total = total.saturating_add(_amount.amount);
                        if total.ge(&amount) {
                            match total.try_sub(amount) {
                                Ok(result) => {
                                    remain = Some(AgeAmount {
                                        amount: result,
                                        expired: Timestamp::from(
                                            current_system_time()
                                                .micros()
                                                .saturating_add(*self.amount_alive_ms.get()),
                                        ),
                                    })
                                }
                                _ => {}
                            }
                            return false;
                        }
                        return false;
                    });
                    match remain {
                        Some(result) => amounts.amounts.push(result),
                        _ => {}
                    }
                    self.balances.insert(&from, amounts).unwrap();
                    match self.balances.get(&to).await {
                        Ok(Some(mut amounts)) => {
                            amounts.amounts.push(AgeAmount {
                                amount,
                                expired: Timestamp::from(
                                    current_system_time()
                                        .micros()
                                        .saturating_add(*self.amount_alive_ms.get()),
                                ),
                            });
                            self.balances.insert(&to, amounts).unwrap();
                        }
                        _ => self
                            .balances
                            .insert(
                                &to,
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
                            )
                            .unwrap(),
                    }
                    match self.spendables.get(&to).await {
                        Ok(Some(spendable)) => self
                            .spendables
                            .insert(&to, spendable.saturating_add(amount))?,
                        _ => self.spendables.insert(&to, amount)?,
                    }
                    Ok(())
                }
            },
            _ => return Err(StateError::InsufficientAccountBalance),
        }
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Insufficient account balance")]
    InsufficientAccountBalance,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
