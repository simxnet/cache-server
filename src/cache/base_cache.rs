use std::sync::Arc;

pub trait BaseCache {
    type Error;

    async fn get_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<String>, Self::Error>;

    async fn set_item(self: Arc<Self>, key: Arc<String>, value: Arc<String>);

    async fn has_item(self: Arc<Self>, key: Arc<String>) -> bool;

    async fn remove_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<String>, Self::Error>;

    async fn keys(self: Arc<Self>) -> Vec<Arc<String>>;
}
