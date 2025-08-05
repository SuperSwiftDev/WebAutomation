use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSpec {
    pub id: String,
    pub url: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSpec {
    pub sites: Vec<SiteSpec>,
}

