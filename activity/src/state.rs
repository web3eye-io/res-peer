use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use activity::{ActivityError, ActivityItem, CreateParams, UpdateParams, Winner};
use linera_sdk::{
    base::{Amount, Owner},
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
    ) -> Result<u64, ActivityError> {
        let activity_id = self.activity_id.get() + 1;
        self.activity_id.set(activity_id);
        self.activities.insert(
            &activity_id,
            ActivityItem {
                id: activity_id,
                title: params.title,
                slogan: params.slogan,
                banner: params.banner,
                posters: params.posters,
                introduction: params.introduction,
                host: owner,
                host_resume: params.host_resume,
                created_at: system_api::current_system_time(),
                activity_type: params.activity_type,
                votable: params.votable,
                vote_type: params.vote_type,
                object_type: Some(params.object_type),
                condition: params.condition,
                sponsors: params.sponsors,
                prize_configs: params.prize_configs,
                voter_reward_percent: params.voter_reward_percent,
                object_candidates: HashSet::default(),
                announcements: HashSet::default(),
                prize_announcement: String::default(),
                vote_powers: HashMap::default(),
                voters: HashMap::default(),
                budget_amount: params.budget_amount,
                join_type: params.join_type,
                location: params.location,
                comments: HashSet::default(),
                registers: HashSet::default(),
                register_start_at: params.register_start_at,
                register_end_at: params.register_end_at,
                vote_start_at: params.vote_start_at,
                vote_end_at: params.vote_end_at,
                participantors: HashSet::default(),
                winners: Vec::new(),
                finalized: false,
            },
        )?;
        Ok(activity_id)
    }

    pub(crate) async fn update_activity(
        &mut self,
        params: UpdateParams,
    ) -> Result<(), ActivityError> {
        let mut activity = self.activities.get(&params.activity_id).await?.unwrap();
        if let Some(title) = params.title {
            activity.title = title
        }
        if let Some(slogan) = params.slogan {
            activity.slogan = Some(slogan)
        }
        if let Some(banner) = params.banner {
            activity.banner = banner
        }
        if let Some(host_resume) = params.host_resume {
            activity.host_resume = host_resume
        }
        if let Some(posters) = params.posters {
            activity.posters = posters
        }
        if let Some(introduction) = params.introduction {
            activity.introduction = introduction
        }
        if let Some(activity_type) = params.activity_type {
            activity.activity_type = activity_type
        }
        if let Some(votable) = params.votable {
            activity.votable = votable
        }
        if let Some(vote_type) = params.vote_type {
            activity.vote_type = vote_type
        }
        if let Some(object_type) = params.object_type {
            activity.object_type = Some(object_type)
        }
        if let Some(condition) = params.condition {
            activity.condition = condition
        }
        if let Some(sponsors) = params.sponsors {
            activity.sponsors = sponsors
        }
        if let Some(prize_configs) = params.prize_configs {
            activity.prize_configs = prize_configs
        }
        if let Some(voter_reward_percent) = params.voter_reward_percent {
            activity.voter_reward_percent = voter_reward_percent
        }
        if let Some(budget_amount) = params.budget_amount {
            activity.budget_amount = budget_amount
        }
        if let Some(join_type) = params.join_type {
            activity.join_type = join_type
        }
        if let Some(location) = params.location {
            activity.location = location
        }
        if let Some(register_start_at) = params.register_start_at {
            activity.register_start_at = register_start_at
        }
        if let Some(register_end_at) = params.register_end_at {
            activity.register_end_at = register_end_at
        }
        if let Some(vote_start_at) = params.vote_start_at {
            activity.vote_start_at = vote_start_at
        }
        if let Some(vote_end_at) = params.vote_end_at {
            activity.vote_end_at = vote_end_at
        }
        self.activities.insert(&activity.clone().id, activity)?;
        Ok(())
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
                    activity.object_candidates.insert(object_id);
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
                Some(_) => Ok(true),
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
        power: Amount,
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
                    Some(votes) => votes.saturating_add(power),
                    None => power,
                };
                activity.vote_powers.insert(object_id.clone(), vote_power);
                let mut voters = match activity.voters.get(&object_id) {
                    Some(voters) => voters.clone(),
                    None => HashSet::default(),
                };
                voters.insert(owner);
                activity.voters.insert(object_id.clone(), voters.clone());
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
        activity.announcements.insert(cid.clone());
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
        let mut least_winner_power = Amount::ZERO;
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
        self.activities.insert(&activity_id, activity)?;
        Ok(())
    }
}
