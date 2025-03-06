pub mod dashmap;

pub trait BaseCache {
    fn new() -> Self;
    fn get_item(&self, key: String) -> Option<String>;
    fn set_item(&self, key: String, value: String);
    fn has_item(&self, key: String) -> bool;
    fn remove_item(&self, key: String) -> Option<String>;
    fn keys(&self) -> Vec<String>;
}
