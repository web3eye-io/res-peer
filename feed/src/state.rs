use feed::{Channel, Content, InitialState};
use linera_sdk::{
    base::{ChainId, Owner, Timestamp},
    contract::system_api::current_system_time,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Feed {
    pub contents: MapView<String, Content>,
    pub publishes: MapView<Owner, Vec<String>>,
    pub react_interval_ms: RegisterView<u64>,
    pub react_accounts: MapView<Owner, Timestamp>,
    pub avatars: MapView<Owner, String>,
    pub channels: RegisterView<Vec<Channel>>,
    pub channel_id: RegisterView<u64>,
}

#[allow(dead_code)]
impl Feed {
    pub(crate) async fn initialize(&mut self, state: InitialState) {
        self.react_interval_ms.set(state.react_interval_ms);
        self.channel_id.set(1000);
    }

    pub(crate) async fn create_content(
        &mut self,
        content: Content,
        owner: Owner,
    ) -> Result<(), StateError> {
        match self.contents.get(&content.clone().cid).await {
            Ok(Some(_)) => return Err(StateError::AlreadyExists),
            _ => {}
        }
        self.contents
            .insert(&content.clone().cid, content.clone())
            .unwrap();
        match self.publishes.get(&owner).await {
            Ok(Some(mut cids)) => {
                cids.push(content.cid);
                match self.publishes.insert(&owner, cids) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(StateError::ViewError(err)),
                }
            }
            _ => match self.publishes.insert(&owner, vec![content.cid]) {
                Ok(_) => Ok(()),
                Err(err) => Err(StateError::ViewError(err)),
            },
        }
    }

    pub(crate) async fn like_content(
        &mut self,
        ccid: String,
        owner: Owner,
        like: bool,
    ) -> Result<(), StateError> {
        match self.react_accounts.get(&owner).await {
            Ok(Some(reacted_at)) => {
                if current_system_time().saturating_diff_micros(reacted_at)
                    < *self.react_interval_ms.get()
                {
                    return Err(StateError::TooFrequently);
                }
            }
            _ => {
                self.react_accounts.insert(&owner, current_system_time())?;
            }
        }
        match self.contents.get(&ccid).await {
            Ok(Some(mut content)) => match content.accounts.get(&owner) {
                Some(&_like) => {
                    if (_like && like) || (!_like && !like) {
                        return Err(StateError::TooManyLike);
                    }
                    content.accounts.insert(owner, like);
                    if _like {
                        content.likes -= 1;
                        content.dislikes += 1;
                    } else {
                        content.likes += 1;
                        content.dislikes -= 1;
                    }
                    self.contents.insert(&content.clone().cid, content)?;
                    Ok(())
                }
                _ => {
                    if like {
                        content.likes += 1;
                    } else {
                        content.dislikes += 1;
                    }
                    content.accounts.insert(owner, like);
                    self.contents.insert(&content.clone().cid, content)?;
                    Ok(())
                }
            },
            _ => return Err(StateError::NotExist),
        }
    }

    pub(crate) async fn create_channel(
        &mut self,
        name: String,
        owner: Owner,
        chain_id: ChainId,
    ) -> Result<(), StateError> {
        let mut channels = self.channels.get().clone();
        channels.push(Channel {
            channel_id: *self.channel_id.get(),
            name,
            owner,
            chain_id,
        });
        self.channels.set(channels);
        self.channel_id.set(self.channel_id.get() + 1);
        Ok(())
    }

    pub(crate) async fn delete_channel(&mut self, channel_id: u64) -> Result<(), StateError> {
        let mut channels = self.channels.get().clone();
        match channels.iter().position(|ch| ch.channel_id == channel_id) {
            Some(index) => {
                channels.remove(index);
                self.channels.set(channels);
            }
            None => return Err(StateError::ChannelNotExist),
        }
        Ok(())
    }
}

/// Attempts to debit from an account with insufficient funds.
#[derive(Debug, Error)]
#[error("Insufficient balance for transfer")]
pub enum StateError {
    #[error("Content already exists")]
    AlreadyExists,

    #[error("Content not exist")]
    NotExist,

    #[error("Only 1 reaction is allowed within 1 minute")]
    TooFrequently,

    #[error("Only 1 like is allowed for each content")]
    TooManyLike,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),

    #[error("Channel not exist")]
    ChannelNotExist,
}
