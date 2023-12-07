use std::collections::HashMap;

use linera_sdk::{
    base::{Amount, ArithmeticError, ChainId, Owner},
    contract::system_api,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use review::{Activity, Asset, Content, InitialState, Review as _Review, Reviewer};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Review {
    pub reviewers: MapView<Owner, Reviewer>,
    pub reviewer_number: RegisterView<u16>,
    pub reviewer_applications: MapView<Owner, Reviewer>,
    pub content_applications: MapView<String, Content>,
    pub asset_applications: MapView<String, Asset>,
    pub content_approved_threshold: RegisterView<u16>,
    pub content_rejected_threshold: RegisterView<u16>,
    pub asset_approved_threshold: RegisterView<u16>,
    pub asset_rejected_threshold: RegisterView<u16>,
    pub reviewer_approved_threshold: RegisterView<u16>,
    pub reviewer_rejected_threshold: RegisterView<u16>,
    pub activity_applications: MapView<u64, Activity>,
    pub activity_approved_threshold: RegisterView<u16>,
    pub activity_rejected_threshold: RegisterView<u16>,
}

#[allow(dead_code)]
impl Review {
    pub(crate) async fn initialize_review(
        &mut self,
        state: InitialState,
    ) -> Result<(), StateError> {
        self.content_approved_threshold
            .set(state.content_approved_threshold);
        self.content_rejected_threshold
            .set(state.content_rejected_threshold);
        self.asset_approved_threshold
            .set(state.asset_approved_threshold);
        self.asset_rejected_threshold
            .set(state.asset_rejected_threshold);
        self.reviewer_approved_threshold
            .set(state.reviewer_approved_threshold);
        self.reviewer_rejected_threshold
            .set(state.reviewer_rejected_threshold);
        self.activity_approved_threshold
            .set(state.activity_approved_threshold);
        self.activity_rejected_threshold
            .set(state.activity_rejected_threshold);
        Ok(())
    }

    pub(crate) async fn initial_state(&self) -> Result<InitialState, StateError> {
        Ok(InitialState {
            content_approved_threshold: *self.content_approved_threshold.get(),
            content_rejected_threshold: *self.content_rejected_threshold.get(),
            asset_approved_threshold: *self.asset_approved_threshold.get(),
            asset_rejected_threshold: *self.asset_rejected_threshold.get(),
            reviewer_approved_threshold: *self.reviewer_approved_threshold.get(),
            reviewer_rejected_threshold: *self.reviewer_rejected_threshold.get(),
            activity_approved_threshold: *self.activity_approved_threshold.get(),
            activity_rejected_threshold: *self.activity_rejected_threshold.get(),
        })
    }

    pub(crate) async fn genesis_reviewer(
        &mut self,
        chain_id: ChainId,
        creator: Owner,
    ) -> Result<(), StateError> {
        self.reviewers.insert(
            &creator,
            Reviewer {
                chain_id,
                reviewer: creator,
                resume: None,
                reviewers: HashMap::default(),
                approved: 1,
                rejected: 0,
                created_at: system_api::current_system_time(),
            },
        )?;
        self.reviewer_number.set(1);
        Ok(())
    }

    pub(crate) async fn add_exist_reviewer(
        &mut self,
        reviewer: Reviewer,
    ) -> Result<(), StateError> {
        self.reviewers
            .insert(&reviewer.clone().reviewer, reviewer)?;
        Ok(())
    }

    pub(crate) async fn is_reviewer(&self, owner: Owner) -> Result<bool, StateError> {
        match self.reviewers.get(&owner).await? {
            Some(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub(crate) async fn apply_reviewer(
        &mut self,
        chain_id: ChainId,
        owner: Owner,
        resume: String,
    ) -> Result<(), StateError> {
        if self.is_reviewer(owner).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.reviewer_applications.get(&owner).await? {
            Some(_) => return Err(StateError::InvalidReviewer),
            _ => {}
        }
        self.reviewer_applications.insert(
            &owner,
            Reviewer {
                chain_id,
                reviewer: owner,
                resume: Some(resume),
                reviewers: HashMap::default(),
                approved: 0,
                rejected: 0,
                created_at: system_api::current_system_time(),
            },
        )?;
        Ok(())
    }

    pub(crate) async fn update_reviewer_resume(
        &mut self,
        owner: Owner,
        resume: String,
    ) -> Result<(), StateError> {
        match self.reviewers.get(&owner).await? {
            Some(mut reviewer) => {
                reviewer.resume = Some(resume);
                self.reviewers.insert(&owner, reviewer)?;
                return Ok(());
            }
            _ => {}
        }
        match self.reviewer_applications.get(&owner).await? {
            Some(mut reviewer) => {
                reviewer.resume = Some(resume);
                self.reviewer_applications.insert(&owner, reviewer)?;
                return Ok(());
            }
            _ => Err(StateError::InvalidReviewer),
        }
    }

    pub(crate) async fn validate_reviewer_review(
        &self,
        reviewer: Owner,
        candidate: Owner,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.reviewer_applications.get(&candidate).await? {
            Some(_reviewer) => match _reviewer.reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Err(StateError::InvalidReviewer),
        }
    }

    pub(crate) async fn approve_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
        reason: String,
    ) -> Result<Option<Reviewer>, StateError> {
        if owner == candidate {
            return Err(StateError::InvalidReviewer);
        }
        self.validate_reviewer_review(owner, candidate.clone())
            .await?;
        match self.reviewer_applications.get(&candidate).await? {
            Some(mut reviewer) => {
                reviewer.approved += 1;
                reviewer.reviewers.insert(
                    owner,
                    _Review {
                        reviewer: owner,
                        approved: true,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.reviewer_applications.insert(&candidate, reviewer)?;
            }
            _ => return Err(StateError::InvalidReviewer),
        }
        match self.reviewer_applications.get(&candidate).await? {
            Some(reviewer) => {
                let approved_threshold = *self.reviewer_approved_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if reviewer.approved >= approved_threshold || reviewer.approved >= reviewer_number {
                    self.reviewers.insert(&candidate, reviewer.clone())?;
                    self.reviewer_applications.remove(&candidate)?;
                    self.reviewer_number.set(reviewer_number + 1);
                    return Ok(Some(reviewer));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn reject_reviewer(
        &mut self,
        owner: Owner,
        candidate: Owner,
        reason: String,
    ) -> Result<Option<Reviewer>, StateError> {
        if owner == candidate {
            return Err(StateError::InvalidReviewer);
        }
        self.validate_reviewer_review(owner, candidate.clone())
            .await?;
        match self.reviewer_applications.get(&candidate).await? {
            Some(mut reviewer) => {
                reviewer.rejected += 1;
                reviewer.reviewers.insert(
                    owner,
                    _Review {
                        reviewer: owner,
                        approved: true,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.reviewer_applications.insert(&candidate, reviewer)?;
            }
            _ => return Err(StateError::InvalidReviewer),
        }
        match self.reviewer_applications.get(&candidate).await? {
            Some(reviewer) => {
                let rejected_threshold = *self.reviewer_rejected_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if reviewer.rejected >= rejected_threshold || reviewer.rejected >= reviewer_number {
                    return Ok(Some(reviewer));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn validate_content_review(
        &self,
        reviewer: Owner,
        content_cid: String,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.content_applications.get(&content_cid).await? {
            Some(content) => match content.reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Err(StateError::InvalidContent),
        }
    }

    pub(crate) async fn submit_content(&mut self, content: Content) -> Result<(), StateError> {
        self.content_applications
            .insert(&content.clone().cid, content)?;
        Ok(())
    }

    pub(crate) async fn approve_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason: String,
    ) -> Result<Option<Content>, StateError> {
        self.validate_content_review(reviewer, content_cid.clone())
            .await?;
        match self.content_applications.get(&content_cid).await? {
            Some(mut content) => {
                if reviewer == content.author {
                    return Err(StateError::InvalidReviewer);
                }
                content.approved += 1;
                content.reviewers.insert(
                    reviewer,
                    _Review {
                        reviewer,
                        approved: true,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.content_applications.insert(&content_cid, content)?;
            }
            _ => return Err(StateError::InvalidContent),
        }
        match self.content_applications.get(&content_cid).await? {
            Some(content) => {
                let approved_threshold = *self.reviewer_approved_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if content.approved >= approved_threshold || content.approved >= reviewer_number {
                    return Ok(Some(content));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn reject_content(
        &mut self,
        reviewer: Owner,
        content_cid: String,
        reason: String,
    ) -> Result<Option<Content>, StateError> {
        self.validate_content_review(reviewer, content_cid.clone())
            .await?;
        match self.content_applications.get(&content_cid).await? {
            Some(mut content) => {
                if reviewer == content.author {
                    return Err(StateError::InvalidReviewer);
                }
                content.rejected += 1;
                content.reviewers.insert(
                    reviewer,
                    _Review {
                        reviewer,
                        approved: false,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.content_applications.insert(&content_cid, content)?;
            }
            _ => return Err(StateError::InvalidReviewer),
        }
        match self.content_applications.get(&content_cid).await? {
            Some(content) => {
                let rejected_threshold = *self.reviewer_rejected_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if content.rejected >= rejected_threshold || content.rejected >= reviewer_number {
                    return Ok(Some(content));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn validate_asset_review(
        &self,
        reviewer: Owner,
        cid: String,
    ) -> Result<(), StateError> {
        if !self.is_reviewer(reviewer).await? {
            return Err(StateError::InvalidReviewer);
        }
        match self.asset_applications.get(&cid).await? {
            Some(asset) => match asset.reviewers.get(&reviewer) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub(crate) async fn approve_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: String,
    ) -> Result<Option<Asset>, StateError> {
        self.validate_asset_review(reviewer, cid.clone()).await?;
        match self.asset_applications.get(&cid).await? {
            Some(mut asset) => {
                if reviewer == asset.author {
                    return Err(StateError::InvalidReviewer);
                }
                asset.approved += 1;
                asset.reviewers.insert(
                    reviewer,
                    _Review {
                        reviewer,
                        approved: true,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.asset_applications.insert(&cid, asset)?;
            }
            _ => return Err(StateError::InvalidReviewer),
        }
        match self.asset_applications.get(&cid).await? {
            Some(asset) => {
                let approved_threshold = *self.reviewer_approved_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if asset.approved >= approved_threshold || asset.approved >= reviewer_number {
                    return Ok(Some(asset));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn reject_asset(
        &mut self,
        reviewer: Owner,
        cid: String,
        reason: String,
    ) -> Result<Option<Asset>, StateError> {
        self.validate_asset_review(reviewer, cid.clone()).await?;
        match self.asset_applications.get(&cid).await? {
            Some(mut asset) => {
                if reviewer == asset.author {
                    return Err(StateError::InvalidReviewer);
                }
                asset.rejected += 1;
                asset.reviewers.insert(
                    reviewer,
                    _Review {
                        reviewer,
                        approved: false,
                        reason,
                        created_at: system_api::current_system_time(),
                    },
                );
                self.asset_applications.insert(&cid, asset)?;
            }
            _ => return Err(StateError::InvalidReviewer),
        }
        match self.asset_applications.get(&cid).await? {
            Some(asset) => {
                let rejected_threshold = *self.reviewer_rejected_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if asset.rejected >= rejected_threshold || asset.rejected >= reviewer_number {
                    return Ok(Some(asset));
                }
            }
            _ => todo!(),
        }
        Ok(None)
    }

    pub(crate) async fn submit_asset(&mut self, asset: Asset) -> Result<(), StateError> {
        match self.asset_applications.get(&asset.clone().cid).await? {
            Some(_) => return Err(StateError::AlreadyExists),
            _ => {
                self.asset_applications.insert(&asset.clone().cid, asset)?;
            }
        }
        Ok(())
    }

    pub(crate) async fn submit_activity(
        &mut self,
        activity_id: u64,
        activity_host: Owner,
        budget_amount: Amount,
    ) -> Result<(), StateError> {
        match self.activity_applications.get(&activity_id).await {
            Ok(Some(_)) => Err(StateError::AlreadyExists),
            _ => Ok(self.activity_applications.insert(
                &activity_id,
                Activity {
                    activity_id,
                    activity_host,
                    budget_amount,
                    approved: 0,
                    rejected: 0,
                    created_at: system_api::current_system_time(),
                    reviewers: HashMap::default(),
                },
            )?),
        }
    }

    pub(crate) async fn validate_activity_review(
        &self,
        owner: Owner,
        activity_id: u64,
    ) -> Result<(), StateError> {
        match self.activity_applications.get(&activity_id).await {
            Ok(Some(activity)) => match activity.reviewers.get(&owner) {
                Some(_) => Err(StateError::AlreadyReviewed),
                _ => Ok(()),
            },
            Ok(None) => Err(StateError::InvalidActivity),
            Err(err) => Err(StateError::ViewError(err)),
        }
    }

    pub(crate) async fn approve_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: String,
    ) -> Result<Option<Activity>, StateError> {
        self.validate_activity_review(owner, activity_id).await?;

        let mut activity = self.activity_applications.get(&activity_id).await?.unwrap();
        activity.reviewers.insert(
            owner.clone(),
            _Review {
                reviewer: owner,
                approved: true,
                reason,
                created_at: system_api::current_system_time(),
            },
        );
        activity.approved += 1;
        self.activity_applications
            .insert(&activity_id, activity.clone())?;

        let approved_threshold = *self.activity_approved_threshold.get();
        let reviewer_number = *self.reviewer_number.get();

        if activity.approved >= approved_threshold || activity.approved >= reviewer_number {
            return Ok(Some(activity));
        }
        Ok(None)
    }

    pub(crate) async fn reject_activity(
        &mut self,
        owner: Owner,
        activity_id: u64,
        reason: String,
    ) -> Result<Option<Activity>, StateError> {
        self.validate_activity_review(owner, activity_id).await?;

        let mut activity = self.activity_applications.get(&activity_id).await?.unwrap();
        activity.reviewers.insert(
            owner.clone(),
            _Review {
                reviewer: owner,
                approved: true,
                reason,
                created_at: system_api::current_system_time(),
            },
        );
        activity.rejected += 1;
        self.activity_applications
            .insert(&activity_id, activity.clone())?;

        let rejected_threshold = *self.activity_rejected_threshold.get();
        let reviewer_number = *self.reviewer_number.get();

        if activity.rejected >= rejected_threshold || activity.rejected >= reviewer_number {
            return Ok(Some(activity));
        }
        Ok(None)
    }

    pub(crate) async fn activity_approved(&self, activity_id: u64) -> Result<bool, StateError> {
        match self.activity_applications.get(&activity_id).await {
            Ok(Some(activity)) => {
                let approved_threshold = *self.activity_approved_threshold.get();
                let reviewer_number = *self.reviewer_number.get();
                if activity.approved >= approved_threshold || activity.approved >= reviewer_number {
                    return Ok(true);
                }
                Ok(false)
            }
            Ok(None) => Err(StateError::InvalidActivity),
            Err(err) => Err(StateError::ViewError(err)),
        }
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

    #[error("Already reviewed")]
    AlreadyReviewed,

    #[error("Invalid content")]
    InvalidContent,

    #[error("Already exists")]
    AlreadyExists,

    #[error("Invalid activity")]
    InvalidActivity,
}
