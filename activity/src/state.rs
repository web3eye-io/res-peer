use activity::ActivityItem;
use linera_sdk::views::{SetView, ViewStorageContext};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Activity {
    pub activities: SetView<ActivityItem>,
}
