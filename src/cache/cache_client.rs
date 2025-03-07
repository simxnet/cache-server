use std::{collections::HashMap, sync::Arc};
use super::base_cache::BaseCache;
use actix_error_proc::ActixError;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(ActixError, Error, Debug)]
pub enum CacheClientError {
    #[error("An entry with the key \"{0}\" was not found.")]
    #[http_status(NotFound)]
    EntryNotFound(Arc<String>)
}

#[derive(Clone)]
pub struct CacheClient {
    entries: Arc<Mutex<HashMap<Arc<String>, Arc<String>>>>,
}

impl CacheClient {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl BaseCache for CacheClient {
    type Error = CacheClientError;

    async fn get_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<String>, Self::Error> {
        let entries = self
            .entries
            .lock()
            .await;

        entries
            .get(&key)
            .map(|v| v.clone())
            .ok_or(CacheClientError::EntryNotFound(key.clone()))
    }

    async fn has_item(self: Arc<Self>, key: Arc<String>) -> bool {
        let entries = self
            .entries
            .lock()
            .await;

        entries
            .contains_key(&key)
    }

    async fn set_item(self: Arc<Self>, key: Arc<String>, value: Arc<String>) {
        let mut entries = self
            .entries
            .lock()
            .await;

        entries
            .insert(
                key.clone(),
                value.clone()
            );
    }

    async fn remove_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<String>, Self::Error> {
        let mut entries = self
            .entries
            .lock()
            .await;

        entries
            .remove(&key)
            .map(|v| v.clone())
            .ok_or(CacheClientError::EntryNotFound(key.clone()))
    }

    async fn keys(self: Arc<Self>) -> Vec<Arc<String>> {
        let entries = self
            .entries
            .lock()
            .await;

        entries
            .iter()
            .map(|(key, _)| key.clone())
            .collect()
    }
}
