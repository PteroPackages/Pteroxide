//! HTTP bindings for the Pterodactyl API using [`pteroxide-models`].
//!
//! [`pteroxide-models`]: pteroxide_models

pub mod application;
pub mod builder;
pub mod error;
pub mod routing;
pub mod value;

pub use self::{application::Application, builder::Builder, error::Error, value::Value};
