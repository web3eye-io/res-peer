use linera_sdk::{
    base::{ArithmeticError, Owner},
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use review::InitialState;
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Review {
    pub reviewers: MapView<Owner, bool>,
    pub content_applications: MapView<String, Vec<i16>>,
    pub asset_applications: MapView<u64, Vec<i16>>,
    pub content_approved_threshold: RegisterView<i16>,
    pub content_rejected_threshold: RegisterView<i16>,
    pub asset_approved_threshold: RegisterView<i16>,
    pub asset_rejected_threshold: RegisterView<i16>,
}

#[allow(dead_code)]
impl Review {
    pub(crate) async fn initialize(&mut self, state: InitialState) -> Result<(), StateError> {
        self.content_approved_threshold
            .set(state.content_approved_threshold);
        self.content_rejected_threshold
            .set(state.content_rejected_threshold);
        self.asset_approved_threshold
            .set(state.asset_approved_threshold);
        self.asset_rejected_threshold
            .set(state.asset_rejected_threshold);
        Ok(())
    }

    pub(crate) async fn is_reviewer(&self, owner: Owner) -> Result<bool, StateError> {
        match self.reviewers.get(&owner).await? {
            Some(approved) => Ok(approved),
            None => Ok(false),
        }
    }

    pub(crate) async fn approve_content(
        &mut self,
        content_cid: String,
    ) -> Result<bool, StateError> {
        let mut need_notify = false;
        match self
            .content_applications
            .get(&content_cid)
            .await?
            .as_deref()
        {
            Some([approved, rejected]) => {
                let approved = *approved + 1;
                self.content_applications
                    .insert(&content_cid, vec![approved, *rejected])?;
                need_notify = approved >= *self.content_approved_threshold.get();
            }
            _ => {
                self.content_applications.insert(&content_cid, vec![1, 0])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn reject_content(&mut self, content_cid: String) -> Result<bool, StateError> {
        let mut need_notify = false;
        match self
            .content_applications
            .get(&content_cid)
            .await?
            .as_deref()
        {
            Some([approved, rejected]) => {
                let rejected = *rejected + 1;
                self.content_applications
                    .insert(&content_cid, vec![*approved, rejected])?;
                need_notify = rejected >= *self.content_rejected_threshold.get();
            }
            _ => {
                self.content_applications.insert(&content_cid, vec![0, 1])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn approve_asset(&mut self, collection_id: u64) -> Result<bool, StateError> {
        let mut need_notify = false;
        match self
            .asset_applications
            .get(&collection_id)
            .await?
            .as_deref()
        {
            Some([approved, rejected]) => {
                let approved = *approved + 1;
                self.asset_applications
                    .insert(&collection_id, vec![approved, *rejected])?;
                need_notify = approved >= *self.content_approved_threshold.get();
            }
            _ => {
                self.asset_applications.insert(&collection_id, vec![1, 0])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn reject_asset(&mut self, collection_id: u64) -> Result<bool, StateError> {
        let mut need_notify = false;
        match self
            .asset_applications
            .get(&collection_id)
            .await?
            .as_deref()
        {
            Some([approved, rejected]) => {
                let rejected = *rejected + 1;
                self.asset_applications
                    .insert(&collection_id, vec![*approved, rejected])?;
                need_notify = rejected >= *self.content_rejected_threshold.get();
            }
            _ => {
                self.asset_applications.insert(&collection_id, vec![0, 1])?;
            }
        }
        Ok(need_notify)
    }
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Arithmetic error")]
    ArithmeticError(#[from] ArithmeticError),
}
