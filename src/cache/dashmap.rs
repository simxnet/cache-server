use std::sync::Arc;

use dashmap::DashMap;

use super::BaseCache;

/// DashMap driver
/// Disclaimer: JSON content should be managed from the app that uses the cache-server. This
/// can be done using serde_json to convert JSON payload to String
/// and viceversa.
#[derive(Clone)]
pub struct CacheClient {
    entries: Arc<DashMap<String, String>>,
}

impl BaseCache for CacheClient {
    fn new() -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
        }
    }

    fn get_item(&self, key: String) -> Option<String> {
        self.entries.get(&key).map(|v| v.clone())
    }

    fn has_item(&self, key: String) -> bool {
        self.entries.contains_key(&key)
    }

    fn set_item(&self, key: String, value: String) {
        self.entries.insert(key, value);
    }

    fn remove_item(&self, key: String) -> Option<String> {
        self.entries.remove(&key).map(|(_, v)| v)
    }

    fn keys(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
}
