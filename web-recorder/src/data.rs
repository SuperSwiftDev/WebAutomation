use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Timestamp(pub String);

impl Timestamp {
    pub fn now() -> Self {
        let now = chrono::Local::now();
        let formatted = now.format("%Y%m%d-%H:%M").to_string();
        Self(formatted)
    }
}
