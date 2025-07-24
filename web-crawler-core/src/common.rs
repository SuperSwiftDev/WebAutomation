use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use url::Url;

// ————————————————————————————————————————————————————————————————————————————
// BASICS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct CanonicalUrlString(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct OriginalUrlString(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct CanonicalUrl(pub Url);

impl CanonicalUrl {
    pub fn from_url(mut url: Url) -> Self {
        url.set_fragment(None);
        Self(url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct OriginalUrl(pub Url);

impl From<Url> for OriginalUrl {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct SnapshotDate(pub String);

impl SnapshotDate {
    pub fn now() -> Self {
        let date = chrono::Utc::now();
        Self(date.to_rfc3339())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeFilePath(pub PathBuf);

impl RelativeFilePath {
    /// Get a `file://./` prefixed string for JSON output.
    pub fn to_file_url(&self) -> String {
        format!("file://./{}", self.0.to_str().unwrap())
    }
}

impl Serialize for RelativeFilePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_file_url())
    }
}

impl<'de> Deserialize<'de> for RelativeFilePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let stripped = s
            .strip_prefix("file://./")
            .ok_or_else(|| serde::de::Error::custom("Expected 'file://./' prefix"))?;
        Ok(RelativeFilePath(PathBuf::from(stripped)))
    }
}

