use linera_views::views::{GraphQLView, RootView};
use linera_sdk::{
    base::{Owner, Amount, Timestamp},
    views::{MapView, ViewStorageContext, RegisterView},
};
use credit::AgeAmounts;
use credit::InitialState;

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
}
