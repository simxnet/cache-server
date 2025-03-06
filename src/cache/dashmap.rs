use dashmap::DashMap;
use serde_json::Value;

use super::BaseCache;

#[derive(Clone)]
pub struct CacheClient {
    entries: DashMap<String, Value>,
}

impl BaseCache for CacheClient {
    fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    fn get_item(&self, key: String) -> Option<Value> {
        self.entries.get(&key).map(|v| v.clone())
    }

    fn has_item(&self, key: String) -> bool {
        self.entries.contains_key(&key)
    }

    fn set_item(&self, key: String, value: Value) {
        self.entries.insert(key, value);
    }

    fn remove_item(&self, key: String) -> Option<Value> {
        self.entries.remove(&key).map(|(_, v)| v)
    }

    fn keys(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
}
