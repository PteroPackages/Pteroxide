use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Allocation {
    pub id: i32,
    pub ip: String,
    pub alias: Option<String>,
    pub port: i64,
    pub notes: Option<String>,
    pub assigned: bool,
}
