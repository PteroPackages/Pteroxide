use serde::{Deserialize, Serialize};

/// Represents a location object. Locations are used to house (or group) [`Node`]s for easier
/// management and server deployment.
///
/// [`Node`]: super::node::Node
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Location {
    pub id: i32,
    pub short: String,
    pub long: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[cfg(feature = "time")]
crate::impl_time!(Location);
