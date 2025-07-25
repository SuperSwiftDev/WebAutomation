use std::path::Path;

// use colored::Colorize;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::metadata::common::{CanonicalUrl, OriginalUrl, RelativeFilePath, SnapshotDate, SnapshotDirectory, Status};

// ————————————————————————————————————————————————————————————————————————————
// BASICS
// ————————————————————————————————————————————————————————————————————————————

// ————————————————————————————————————————————————————————————————————————————
// WEBPAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotLog {
    pub http_status: Option<i64>,
    pub original_url: OriginalUrl,
    pub canonical_url: CanonicalUrl,
    /// Relative to the output directory.
    pub snapshot_path: RelativeFilePath,
    pub snapshot_date: SnapshotDate,
    pub outgoing_links: IndexSet<OriginalUrl>,
    pub incoming_links: IndexSet<OriginalUrl>,
}

impl SnapshotLog {
    pub const TOML_LOG_FILE_NAME: &'static str = ".snapshot.log.toml";
    pub fn load(snapshot_directory: &SnapshotDirectory) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = snapshot_directory.join(Self::TOML_LOG_FILE_NAME);
        let contents = std::fs::read_to_string(file_path)?;
        let data = toml::from_str::<Self>(&contents)?;
        Ok(data)
    }
    pub fn open(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let contents = std::fs::read_to_string(file_path)?;
        let data = toml::from_str::<Self>(&contents)?;
        Ok(data)
    }
    pub fn write(&self, snapshot_directory: &SnapshotDirectory) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = snapshot_directory.join(Self::TOML_LOG_FILE_NAME);
        let contents = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(&snapshot_directory.0).unwrap();
        std::fs::write(file_path, &contents)?;
        Ok(())
    }
}

// ————————————————————————————————————————————————————————————————————————————
// WEBPAGE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskLog {
    pub entries: Vec<Status>,
}

impl TaskLog {
    pub const TOML_LOG_FILE_NAME: &'static str = ".task.log.toml";
    pub fn load(snapshot_directory: &SnapshotDirectory) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = snapshot_directory.join(Self::TOML_LOG_FILE_NAME);
        let contents = std::fs::read_to_string(file_path)?;
        let data = toml::from_str::<Self>(&contents)?;
        Ok(data)
    }
    pub fn open(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let contents = std::fs::read_to_string(file_path)?;
        let data = toml::from_str::<Self>(&contents)?;
        Ok(data)
    }
    pub fn write(&self, snapshot_directory: &SnapshotDirectory) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = snapshot_directory.join(Self::TOML_LOG_FILE_NAME);
        let contents = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(&snapshot_directory.0).unwrap();
        std::fs::write(file_path, &contents)?;
        Ok(())
    }
    pub fn contains_url(
        &self,
        given_url: &Url,
        snapshot_directory: &SnapshotDirectory,
    ) -> bool {
        // let snapshot_directory = snapshot_file_path.parent().unwrap();
        let log_file = snapshot_directory.join(Self::TOML_LOG_FILE_NAME);
        let existing = std::fs::read_to_string(&log_file)
            .map_err(|x| Box::new(x) as Box<dyn std::error::Error>)
            .and_then(|contents| {
                let data = toml::from_str::<Self>(&contents)?;
                Ok(data)
            })
            .map(|x| x.entries)
            // .inspect_err(|error| {
            //     eprintln!("{}", format!(
            //         "\t ☞ [notice] tried to read cached file (not necessarily an issue): {error}",
            //     ).blue());
            // })
            .unwrap_or_default();
        let all_entries = self.entries
            .clone()
            .into_iter()
            .chain(existing)
            .collect::<IndexSet<_>>();
        for entry in all_entries {
            match entry {
                Status::Failure { url, .. } => {
                    if given_url == &url.0 {
                        return true
                    }
                }
                Status::Redirected { from, to, .. } => {
                    if given_url == &from.0 {
                        return true
                    }
                    if given_url == &to.0 {
                        return true
                    }
                }
                Status::Success { url, .. } => {
                    if given_url == &url.0 {
                        return true
                    }
                }
            }
        }
        false
    }
    pub fn contains_failures(&self) -> bool {
        self.entries.iter().any(|status| {
            match status {
                Status::Failure { .. } => true,
                Status::Success { .. } => false,
                Status::Redirected { .. } => false,
            }
        })
    }
}
