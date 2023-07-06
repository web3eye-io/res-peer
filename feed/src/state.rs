use feed::{Content, InitialState};
use linera_sdk::{views::{ViewStorageContext, MapView, RegisterView}, base::{Owner, Timestamp}};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Feed {
    pub contents: MapView<String, Content>,
    pub react_interval_ms: RegisterView<u64>,
    pub react_accounts: MapView<Owner, Timestamp>
}

#[allow(dead_code)]
impl Feed {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.react_interval_ms.set(state.react_interval_ms);
    }

    pub(crate) async fn react_interval_ms(&self) -> u64 {
        *self.react_interval_ms.get()
    }

    pub(crate) async fn content(&self, cid: String) -> Content {
        self.contents.get(&cid).await.unwrap().unwrap()
    }

    pub(crate) async fn account_react_timestamp(&self, owner: Owner) -> Timestamp {
        self.react_accounts.get(&owner).await.unwrap().unwrap()
    }
}