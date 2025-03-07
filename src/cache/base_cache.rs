use std::sync::Arc;
use bytes::Bytes;

pub trait BaseCache {
    type Error;

    async fn get_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error>;

    async fn set_item(self: Arc<Self>, key: Arc<String>, value: Arc<Bytes>);

    async fn has_item(self: Arc<Self>, key: Arc<String>) -> bool;

    async fn remove_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error>;

    async fn keys(self: Arc<Self>) -> Vec<Arc<String>>;
}
