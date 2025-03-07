use std::{fmt::Display, sync::Arc};
use actix_web::HttpResponse;
use bytes::Bytes;

/// This trait implements the base methods
/// for a structure to implement the hability
/// of storing key value pairs of string -> bytes.
///
/// The trait manages bytes to avoid encoding
/// issues, this way the client can serialize
/// and deserialize whatever they want.
///
/// The trait is completly thread safe,
/// meaning that there is only one shared
/// value for most of the things, being
/// this the `Arc<T>` behavour.
pub trait BaseCache where Self: Sized {
    /// This type is a generic error which must implement
    /// `Display` and `Into<HttpResponse>`, this to be used
    /// within the `actix_web` fallible routes.
    type Error: Into<HttpResponse> + Display;

    /// This method manages `self` as a shared reference,
    /// this to avoid concurrency problems between
    /// route threads.
    ///
    /// This method takes a key shared reference and
    /// obtains an item related to the key, if an item
    /// with that key does not exist, the function
    /// will return an instance of `Self::Error`.
    async fn get_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error>;

    /// This method manages `self` as a shared reference,
    /// this to avoid concurrency problems between
    /// route threads.
    ///
    /// This method takes a shared reference to a key
    /// and a shared reference to a `Byte` collection and
    /// inserts or overwrites the value associated to the key
    /// in the internal collection.
    async fn set_item(self: Arc<Self>, key: Arc<String>, value: Arc<Bytes>);

    /// This method manages `self` as a shared reference,
    /// this to avoid concurrency problems between
    /// route threads.
    ///
    /// This method takes a shared reference to a key
    /// and checks whether that key is present inside
    /// the internal collection, returns true if it is,
    /// otherwise false.
    async fn has_item(self: Arc<Self>, key: Arc<String>) -> bool;

    /// This method manages `self` as a shared reference,
    /// this to avoid concurrency problems between
    /// route threads.
    ///
    /// This method takes a shared reference to a key
    /// and removes the value associated to that key,
    /// the method returns the deleted value or `Self::Error`
    /// if the value does not exist.
    async fn remove_item(self: Arc<Self>, key: Arc<String>) -> Result<Arc<Bytes>, Self::Error>;

    /// This method manages `self` as a shared reference,
    /// this to avoid concurrency problems between
    /// route threads.
    ///
    /// This method takes all the keys from the internal
    /// collection and returns them as a vector of shared
    /// references to them.
    async fn keys(self: Arc<Self>) -> Vec<Arc<String>>;
}
