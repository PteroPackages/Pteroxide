use serde::{Deserialize, Serialize};

/// Represents an allocation object (a combination of an IP address and port).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Allocation {
    pub id: i32,
    pub ip: String,
    pub alias: Option<String>,
    pub port: i32,
    pub notes: Option<String>,
    pub assigned: bool,
}
