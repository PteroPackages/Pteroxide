use serde::{Deserialize, Serialize};

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
