use std::collections::HashMap;

use activity::{ActivityError, ActivityItem, CreateParams};
use linera_sdk::{
    base::Owner,
    contract::system_api,
    views::{generic_array::typenum::False, MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Activity {
    pub activities: MapView<u64, ActivityItem>,
    pub activity_id: RegisterView<u64>,
}

#[allow(dead_code)]
impl Activity {
    pub(crate) async fn create_activity(
        &mut self,
        owner: Owner,
        params: CreateParams,
    ) -> Result<(), ActivityError> {
        let activity_id = self.activity_id.get() + 1;
        self.activity_id.set(activity_id);
        Ok(self.activities.insert(
            &activity_id,
            ActivityItem {
                id: activity_id,
                slogan: params.slogan,
                banner: params.banner,
                posters: params.posters,
                introduction: params.introduction,
                host: owner,
                created_at: system_api::current_system_time(),
                activity_type: params.activity_type,
                votable: params.votable,
                vote_type: params.vote_type,
                object_type: Some(params.object_type),
                condition: params.condition,
                sponsors: params.sponsors,
                prize_configs: params.prize_configs,
                voter_reward_percent: params.voter_reward_percent,
                object_candidates: HashMap::default(),
                announcements: Vec::new(),
                prize_announcement: String::default(),
                vote_powers: HashMap::default(),
                voters: HashMap::default(),
                budget_amount: params.budget_amount,
                join_type: params.join_type,
                location: params.location,
                comments: Vec::new(),
                registers: Vec::new(),
                register_start_at: params.register_start_at,
                register_end_at: params.register_end_at,
                vote_start_at: params.vote_start_at,
                vote_end_at: params.vote_end_at,
                participantors: Vec::new(),
            },
        )?)
    }

    pub(crate) async fn register(
        &mut self,
        activity_id: u64,
        object_id: String,
    ) -> Result<(), ActivityError> {
        match self.activities.get(&activity_id).await {
            Ok(Some(mut activity)) => match activity.object_candidates.get(&object_id) {
                Some(_) => Err(ActivityError::AlreadyRegistered),
                _ => {
                    activity.object_candidates.insert(object_id, true);
                    Ok(self.activities.insert(&activity_id, activity)?)
                }
            },
            Ok(None) => Err(ActivityError::InvalidActivity),
            Err(err) => Err(ActivityError::ViewError(err)),
        }
    }

    pub(crate) async fn activity(&self, activity_id: u64) -> Result<ActivityItem, ActivityError> {
        match self.activities.get(&activity_id).await {
            Ok(Some(activity)) => Ok(activity),
            Ok(None) => Err(ActivityError::InvalidActivity),
            Err(err) => Err(ActivityError::ViewError(err)),
        }
    }

    pub(crate) async fn votable(&self, activity_id: u64) -> Result<bool, ActivityError> {
        Ok(self.activity(activity_id).await?.votable)
    }

    pub(crate) async fn voted(
        &self,
        owner: Owner,
        activity_id: u64,
        object_id: String,
    ) -> Result<bool, ActivityError> {
        let activity = self.activity(activity_id).await?;
        match activity.voters.get(&object_id) {
            Some(voters) => match voters.get(&owner) {
                Some(_) => Ok(false),
                None => Ok(false),
            },
            None => Ok(false),
        }
    }

    pub(crate) async fn vote(
        &mut self,
        owner: Owner,
        activity_id: u64,
        object_id: String,
        // If by power, it'll be owner balance; if by account, it'll be 1
        power: u128,
    ) -> Result<(), ActivityError> {
        match self.voted(owner, activity_id, object_id.clone()).await {
            Ok(true) => return Err(ActivityError::ActivityObjectAlreadyVoted),
            Ok(false) => {}
            Err(err) => return Err(err),
        }
        match self.activities.get(&activity_id).await {
            Ok(Some(mut activity)) => match activity.object_candidates.get(&object_id) {
                Some(_) => {
                    let vote_power = match activity.vote_powers.get(&object_id) {
                        Some(votes) => votes + power,
                        None => power,
                    };
                    activity.vote_powers.insert(object_id.clone(), vote_power);
                    let mut voters = match activity.voters.get(&object_id) {
                        Some(voters) => voters.clone(),
                        None => HashMap::default(),
                    };
                    voters.insert(owner, true);
                    activity.voters.insert(object_id, voters);
                    self.activities.insert(&activity_id, activity)?;
                    Ok(())
                }
                _ => return Err(ActivityError::ActivityObjectNotFound),
            },
            Ok(None) => Err(ActivityError::InvalidActivity),
            Err(err) => Err(ActivityError::ViewError(err)),
        }
    }
}
