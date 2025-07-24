use std::path::Path;

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::common::{CanonicalUrl, OriginalUrl, RelativeFilePath, SnapshotDate};

// ————————————————————————————————————————————————————————————————————————————
// HELPERS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VisitedPage {
    Success { url: OriginalUrl, http_status: Option<i64> },
    Failure { url: OriginalUrl, http_status: Option<i64> },
    Redirected { from: OriginalUrl, to: OriginalUrl, http_status: Option<i64> },
}

// impl VisitedPage {
//     pub fn url(&self) -> &OriginalUrlString {
//         match self {
//             Self::Success { url, .. } => url,
//             Self::Failure { url, .. } => url,
//         }
//     }
// }

// ————————————————————————————————————————————————————————————————————————————
// PAGE ENTRY
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebpageSnapshotMetadata {
    pub http_status: Option<i64>,
    pub original_url: OriginalUrl,
    pub canonical_url: CanonicalUrl,
    /// Relative to the output directory.
    pub snapshot_path: RelativeFilePath,
    pub snapshot_date: SnapshotDate,
    pub outgoing_links: IndexSet<OriginalUrl>,
    pub incoming_links: IndexSet<OriginalUrl>,
}

// ————————————————————————————————————————————————————————————————————————————
// DATABASE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotManifest {
    pub snapshots: Vec<WebpageSnapshotMetadata>,
    pub visited_pages: IndexSet<VisitedPage>,
    // pub entries: BTreeMap<CanonicalUrlString, PageEntry>,
    // pub failed_urls: BTreeMap<OriginalUrlString, FailedUrlError>,
    // pub skipped_locals: BTreeSet<OriginalUrlString>,
    // pub redirects: BTreeMap<OriginalUrlString, OriginalUrlString>,
}

impl SnapshotManifest {
    pub fn load_or_default(file_path: impl AsRef<Path>) -> Self {
        Self::open(file_path).unwrap_or_default()
    }
    pub fn open(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let contents = std::fs::read_to_string(file_path)?;
        let manifest = toml::from_str::<Self>(&contents)?;
        Ok(manifest)
    }
    pub fn write(&self, file_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let toml_str = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        std::fs::write(file_path, toml_str)?;
        Ok(())
    }
    pub fn should_visit(&self, given_url: &Url) -> bool {
        for entry in self.visited_pages.iter() {
            match entry {
                VisitedPage::Success { url, .. } => {
                    if given_url == &url.0 {
                        return false
                    }
                }
                VisitedPage::Failure { url, .. } => {
                    if given_url == &url.0 {
                        return false
                    }
                }
                VisitedPage::Redirected { from, to, .. } => {
                    if given_url == &from.0 {
                        return false
                    }
                    if given_url == &to.0 {
                        return false
                    }
                }
            }
        }
        true
    }
}

