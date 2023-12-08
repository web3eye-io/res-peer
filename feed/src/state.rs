use feed::{Content, InitialState};
use linera_sdk::{
    base::{Owner, Timestamp},
    contract::system_api::current_system_time,
    views::{MapView, RegisterView, ViewStorageContext},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct Feed {
    pub contents: MapView<String, Content>,
    pub content_recommends: MapView<String, Vec<String>>,
    pub content_comments: MapView<String, Vec<String>>,
    pub publishes: MapView<Owner, Vec<String>>,
    pub react_interval_ms: RegisterView<u64>,
    pub react_accounts: MapView<Owner, Timestamp>,
    pub collection_recommends: MapView<u64, Vec<String>>,
    pub collection_comments: MapView<u64, Vec<String>>,
}

#[allow(dead_code)]
impl Feed {
    pub(crate) async fn initialize_feed(&mut self, state: InitialState) {
        self.react_interval_ms.set(state.react_interval_ms);
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

    pub(crate) async fn recommend_content(
        &mut self,
        cid: String,
        reason_cid: String,
    ) -> Result<(), StateError> {
        match self.content_recommends.get(&cid).await? {
            Some(mut recommends) => {
                recommends.push(reason_cid);
                self.content_recommends.insert(&cid, recommends)?;
            }
            _ => {
                self.content_recommends.insert(&cid, vec![reason_cid])?;
            }
        }
        Ok(())
    }

    pub(crate) async fn comment_content(
        &mut self,
        cid: String,
        comment_cid: String,
    ) -> Result<(), StateError> {
        match self.content_comments.get(&cid).await? {
            Some(mut comments) => {
                comments.push(comment_cid);
                self.content_comments.insert(&cid, comments)?;
            }
            _ => {
                self.content_comments.insert(&cid, vec![comment_cid])?;
            }
        }
        Ok(())
    }

    pub(crate) async fn content_author(&self, cid: String) -> Result<Owner, StateError> {
        match self.contents.get(&cid).await {
            Ok(Some(content)) => Ok(content.author),
            Ok(None) => Err(StateError::InvalidContent),
            Err(err) => Err(StateError::ViewError(err)),
        }
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

    #[error("Invalid content")]
    InvalidContent,

    #[error("View error")]
    ViewError(#[from] linera_views::views::ViewError),
}
