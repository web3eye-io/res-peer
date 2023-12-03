use std::{cmp::Ordering, collections::HashMap};

use activity::{ActivityError, ActivityItem, CreateParams, Winner};
use linera_sdk::{
    base::Owner,
    contract::system_api,
    views::{MapView, RegisterView, ViewStorageContext},
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
                title: params.title,
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
                announcements: HashMap::default(),
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
                winners: Vec::new(),
                finalized: false,
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
        let mut activity = self.activity(activity_id).await?;
        match activity.object_candidates.get(&object_id) {
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
        }
    }

    pub(crate) async fn announce(
        &mut self,
        activity_id: u64,
        cid: String,
        announce_prize: bool,
    ) -> Result<(), ActivityError> {
        let mut activity = self.activity(activity_id).await?;
        match activity.announcements.get(&cid) {
            Some(_) => return Err(ActivityError::ActivityAnnouncementAlreadyCreated),
            None => {}
        }
        activity.announcements.insert(cid.clone(), true);
        if announce_prize {
            activity.prize_announcement = cid;
        }
        Ok(())
    }

    pub(crate) async fn finalize(&mut self, activity_id: u64) -> Result<(), ActivityError> {
        let mut activity = self.activity(activity_id).await?;
        if activity.finalized {
            return Err(ActivityError::ActivityAlreadyFinalized);
        }
        let mut winners = Vec::<Winner>::new();
        let mut least_winner_power = 0 as u128;
        let mut least_winner_index = 0;
        activity
            .vote_powers
            .clone()
            .into_iter()
            .for_each(|(object_id, power)| {
                if winners.len() < activity.prize_configs.len() {
                    winners.push(Winner {
                        object_id,
                        place: 1,
                    });
                    if power < least_winner_power {
                        least_winner_power = power;
                        least_winner_index = winners.len() - 1;
                    }
                    return;
                }
                if power < least_winner_power {
                    return;
                }
                winners[least_winner_index] = Winner {
                    object_id,
                    place: 1,
                };
                least_winner_power = power;
                for i in 0..winners.len() {
                    let winner_power = activity
                        .vote_powers
                        .get(&winners[i].object_id)
                        .unwrap()
                        .clone();
                    if winner_power < least_winner_power {
                        least_winner_index = i;
                        least_winner_power = winner_power;
                    }
                }
            });
        winners.sort_by(|a, b| {
            let a_power = activity.vote_powers.get(&a.object_id).unwrap().clone();
            let b_power = activity.vote_powers.get(&b.object_id).unwrap().clone();
            if a_power > b_power {
                return Ordering::Greater;
            }
            Ordering::Less
        });
        for i in 0..winners.len() {
            winners[i].place = i as u16 + 1;
        }
        activity.winners = winners;
        activity.finalized = true;
        Ok(())
    }
}
