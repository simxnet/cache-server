use std::{collections::HashMap, sync::Arc};
use super::base_cache::BaseCache;
use actix_error_proc::ActixError;
use bytes::Bytes;
use thiserror::Error;
use tokio::sync::Mutex;

/// This error is used to serialize errors
/// in the implementation of `BaseCache` for
/// `CacheClient`, the enumerator implements
/// `Into<HttpResponse>` as per the `ActixError`
/// derive to be used within any `proof_route`.
#[derive(ActixError, Error, Debug)]
pub enum CacheClientError {
    /// This error should be returned
    /// when a required key is not found
    /// in the internal `CacheClient` collection.
    #[error("An entry with the key \"{0}\" was not found.")]
    #[http_status(NotFound)]
    EntryNotFound(Arc<String>)
}

/// `CacheClient` is an immutable implementation
/// of `BaseCache`, being a wrapper to a thread
/// safe collection.
pub struct CacheClient {
    /// `entries` is the internal collection
    /// for `CacheClient`, it stores an internally
    /// mutable `HashMap` of shared references
    /// both key and value.
    entries: Arc<Mutex<HashMap<Arc<String>, Arc<Bytes>>>>,
}

impl CacheClient {
    /// This function creates a new instance
    /// of `CacheClient`, also intializing the
    /// internal collection.
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl BaseCache for CacheClient {
    type Error = CacheClientError;

    async fn get_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error> {
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

    async fn set_item(self: Arc<Self>, key: Arc<String>, value: Arc<Bytes>) {
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

    async fn remove_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error> {
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
