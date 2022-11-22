//! HTTP bindings for the Pterodactyl API using [`pteroxide-models`].
//! 
//! [`pteroxide-models`]: pteroxide_models

pub mod actions;
pub mod builder;
pub mod error;
pub mod http;
pub mod routing;

pub use self::{builder::Builder, error::Error, http::Http};
