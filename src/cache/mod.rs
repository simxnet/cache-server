pub mod dashmap;

/// A generic key-value storage driver trait.
///
/// This trait defines an interface for a key-value storage system where the
/// values can be of any type `T`. Implementations should handle storing,
/// retrieving, checking, and removing items, as well as listing all stored keys.
///
/// # Type Parameters
/// - `T`: The type of the values being stored.
///
/// # Example Implementation Using `dashmap`
///
/// ```rust
/// use dashmap::DashMap;
/// use super::BaseDriver;
///
/// /// A thread-safe key-value store using DashMap.
/// struct DashMapDriver<T: Clone + Send + Sync + 'static> {
///     storage: DashMap<String, T>,
/// }
///
/// impl<T: Clone + Send + Sync + 'static> BaseDriver<T> for DashMapDriver<T> {
///     fn new() -> Self {
///         Self {
///             storage: DashMap::new(),
///         }
///     }
///
///     fn get_item(&self, key: String) -> Option<T> {
///         self.storage.get(&key).map(|v| v.clone())
///     }
///
///     fn set_item(&self, key: String, value: T) {
///         self.storage.insert(key, value);
///     }
///
///     fn remove_item(&self, key: String) -> Option<T> {
///         self.storage.remove(&key).map(|(_, v)| v)
///     }
///
///     fn keys(&self) -> Vec<String> {
///         self.storage.iter().map(|entry| entry.key().clone()).collect()
///     }
/// }
/// ```
pub trait BaseDriver<T: Clone> {
    /// Creates a new instance of the driver.
    fn new() -> Self;

    /// Retrieves the value associated with the given key.
    ///
    /// # Returns
    /// - `Some(T)`: The value if the key exists.
    /// - `None`: If the key is not found.
    fn get_item(&self, key: String) -> Option<T>;

    /// Stores a key-value pair in the storage.
    ///
    /// # Arguments
    /// - `key`: The key to store.
    /// - `value`: The value to associate with the key.
    fn set_item(&self, key: String, value: T);

    /// Removes a key-value pair from the storage.
    ///
    /// # Returns
    /// - `Some(T)`: The removed value if the key existed.
    /// - `None`: If the key was not found.
    fn remove_item(&self, key: String) -> Option<T>;

    /// Returns a list of all stored keys.
    fn keys(&self) -> Vec<String>;
}
