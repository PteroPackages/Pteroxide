use serde::{Deserialize, Serialize};

/// Represents a nest object containing eggs (services) information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Nest {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[cfg(feature = "time")]
crate::impl_time!(Nest);
