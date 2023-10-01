use std::collections::HashMap;

use linera_sdk::{
    base::{ArithmeticError, Owner},
    views::{MapView, RegisterView, SetView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use review::InitialState;
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Review {
    pub reviewers: SetView<Owner>,
    pub reviewer_number: RegisterView<i16>,
    pub reviewer_applications: MapView<Owner, Vec<i16>>,
    pub reviewer_resumes: MapView<Owner, String>,
    pub reviewer_reviewers: MapView<Owner, HashMap<Owner, bool>>,
    pub content_applications: MapView<String, Vec<i16>>,
    pub content_reviewers: MapView<String, HashMap<Owner, bool>>,
    pub content_approvers: MapView<String, HashMap<Owner, String>>,
    pub content_rejectors: MapView<String, HashMap<Owner, String>>,
    pub asset_reviewers: MapView<u64, HashMap<Owner, bool>>,
    pub asset_approvers: MapView<String, HashMap<Owner, String>>,
    pub asset_rejectors: MapView<String, HashMap<Owner, String>>,
    pub asset_applications: MapView<u64, Vec<i16>>,
    pub content_approved_threshold: RegisterView<i16>,
    pub content_rejected_threshold: RegisterView<i16>,
    pub asset_approved_threshold: RegisterView<i16>,
    pub asset_rejected_threshold: RegisterView<i16>,
    pub reviewer_approved_threshold: RegisterView<i16>,
    pub reviewer_rejected_threshold: RegisterView<i16>,
}

#[allow(dead_code)]
impl Review {
    pub(crate) async fn initialize(&mut self, creator: Owner, state: InitialState) -> Result<(), StateError> {
        self.content_approved_threshold
            .set(state.content_approved_threshold);
        self.content_rejected_threshold
            .set(state.content_rejected_threshold);
        self.asset_approved_threshold
            .set(state.asset_approved_threshold);
        self.asset_rejected_threshold
            .set(state.asset_rejected_threshold);
        self.reviewer_rejected_threshold
            .set(state.reviewer_rejected_threshold);
        self.reviewer_rejected_threshold
            .set(state.reviewer_rejected_threshold);
        self.reviewers.insert(&creator)?;
        self.reviewer_number.set(1);
        Ok(())
    }

    pub(crate) async fn is_reviewer(&self, owner: Owner) -> Result<bool, StateError> {
        Ok(self.reviewers.contains(&owner).await?)
    }

    pub(crate) async fn apply_reviewer(
        &mut self,
        owner: Owner,
        resume: String,
    ) -> Result<(), StateError> {
        if self.reviewers.contains(&owner).await? {
            return Err(StateError::InvalidReviewer);
        }
        self.reviewer_applications.insert(&owner, vec![0, 0])?;
        self.reviewer_resumes.insert(&owner, resume)?;
        Ok(())
    }

    pub(crate) async fn update_reviewer_resume(
        &mut self,
        owner: Owner,
        resume: String,
    ) -> Result<(), StateError> {
        if !self.reviewers.contains(&owner).await? {
            match self.reviewer_applications.get(&owner).await? {
                Some(_) => {}
                _ => return Err(StateError::InvalidReviewer),
            }
        }
        self.reviewer_resumes.insert(&owner, resume)?;
        Ok(())
    }

    pub(crate) async fn validate_reviewer_review(
        &self,
        reviewer: Owner,
        candidate: Owner,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.reviewer_reviewers.get(&candidate).await? {
            Some(reviewers) => match reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub(crate) async fn approve_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
    ) -> Result<bool, StateError> {
        self.validate_reviewer_review(owner, candidate.clone())
            .await?;
        match self.reviewer_applications.get(&candidate).await?.as_deref() {
            Some([approved, rejected]) => {
                let approved = *approved + 1;
                self.reviewer_applications
                    .insert(&candidate, vec![approved, *rejected])?;
            }
            _ => {
                self.reviewer_applications.insert(&candidate, vec![1, 0])?;
            }
        }
        let mut need_notify = false;
        match self.reviewer_applications.get(&candidate).await?.as_deref() {
            Some([approved, _rejected]) => {
                let approved_threshold = *self.reviewer_approved_threshold.get();
                if *approved >= approved_threshold || *approved >= *self.reviewer_number.get() {
                    self.reviewers.insert(&candidate)?;
                    self.reviewer_number.set(self.reviewer_number.get() + 1);
                    need_notify = *approved == *self.content_approved_threshold.get()
                    || *approved == *self.reviewer_number.get();
                }
            }
            _ => todo!(),
        }
        Ok(need_notify)
    }

    pub(crate) async fn reject_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
    ) -> Result<(), StateError> {
        self.validate_reviewer_review(owner, candidate.clone())
            .await?;
        match self.reviewer_applications.get(&candidate).await?.as_deref() {
            Some([approved, rejected]) => {
                let rejected = *rejected + 1;
                self.reviewer_applications
                    .insert(&candidate, vec![*approved, rejected])?;
            }
            _ => {
                self.reviewer_applications.insert(&candidate, vec![0, 1])?;
            }
        }
        Ok(())
    }

    pub(crate) async fn validate_content_review(
        &self,
        reviewer: Owner,
        content_cid: String,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.content_reviewers.get(&content_cid).await? {
            Some(reviewers) => match reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub(crate) async fn approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
    ) -> Result<bool, StateError> {
        self.validate_content_review(reviewer, content_cid.clone())
            .await?;
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
                need_notify = approved == *self.content_approved_threshold.get()
                    || approved == *self.reviewer_number.get();
            }
            _ => {
                self.content_applications.insert(&content_cid, vec![1, 0])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
    ) -> Result<bool, StateError> {
        self.validate_content_review(reviewer, content_cid.clone())
            .await?;
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
                need_notify = rejected == *self.content_rejected_threshold.get()
                    || rejected == *self.reviewer_number.get();
            }
            _ => {
                self.content_applications.insert(&content_cid, vec![0, 1])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn validate_asset_review(
        &self,
        reviewer: Owner,
        collection_id: u64,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.asset_reviewers.get(&collection_id).await? {
            Some(reviewers) => match reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub(crate) async fn approve_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
    ) -> Result<bool, StateError> {
        self.validate_asset_review(reviewer, collection_id).await?;
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
                need_notify = approved == *self.content_approved_threshold.get()
                    || approved == *self.reviewer_number.get();
            }
            _ => {
                self.asset_applications.insert(&collection_id, vec![1, 0])?;
            }
        }
        Ok(need_notify)
    }

    pub(crate) async fn reject_asset(
        &mut self,
        reviewer: Owner,
        collection_id: u64,
    ) -> Result<bool, StateError> {
        self.validate_asset_review(reviewer, collection_id).await?;
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
                need_notify = rejected == *self.content_rejected_threshold.get()
                    || rejected == *self.reviewer_number.get();
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

    #[error("Invalid reviewer")]
    InvalidReviewer,

    #[error("Already reviewer")]
    AlreadyReviewed,
}
