use std::{hash::RandomState, sync::Arc};

use super::BaseDriver;
use dashmap::DashMap;

/// A thread-safe key-value store using `DashMap`.
///
/// This implementation provides highly optimized concurrent access
/// to stored data, making it ideal for multi-threaded environments.
///
/// # Example Usage
/// ```
/// let driver = DashMapDriver::<String>::new();
/// driver.set_item("username".to_string(), "Alice".to_string());
/// assert_eq!(driver.get_item("username"), Some("Alice".to_string()));
/// driver.remove_item("username");
/// ```
pub struct DashMapDriver<T: Clone> {
    storage: Arc<DashMap<String, T>>,
}

impl<T: Clone> BaseDriver<T> for DashMapDriver<T> {
    /// Creates a new `DashMapDriver` instance.
    fn new() -> Self {
        let hasher = RandomState::new();

        Self {
            storage: Arc::new(DashMap::<String, T>::with_capacity_and_hasher(100, hasher)),
        }
    }

    /// Retrieves a value associated with a key, if it exists.
    fn get_item(&self, key: String) -> Option<T> {
        self.storage.get(&key).map(|v| v.clone())
    }

    /// Stores a key-value pair in the storage.
    fn set_item(&self, key: String, value: T) {
        self.storage.insert(key, value);
    }

    /// Removes an item from the storage, returning the value if found.
    fn remove_item(&self, key: String) -> Option<T> {
        self.storage.remove(&key).map(|(_, v)| v)
    }

    /// Returns a list of all keys stored.
    fn keys(&self) -> Vec<String> {
        self.storage
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }
}
