use serde_json::Value;

pub mod dashmap;

pub trait BaseCache {
    fn new() -> Self;
    fn get_item(&self, key: String) -> Option<Value>;
    fn set_item(&self, key: String, value: Value);
    fn has_item(&self, key: String) -> bool;
    fn remove_item(&self, key: String) -> Option<Value>;
    fn keys(&self) -> Vec<String>;
}
