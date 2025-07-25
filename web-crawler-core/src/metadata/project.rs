use std::{collections::LinkedList, path::Path};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::metadata::{common::SnapshotDirectory, snapshot::{SnapshotLog, TaskLog}};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectLog {
    pub snapshot_logs: LinkedList<SnapshotLog>,
    pub task_logs: LinkedList<TaskLog>,
}

impl ProjectLog {
    pub fn load(project_directory: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let project_directory = project_directory.as_ref();
        std::fs::create_dir_all(&project_directory).unwrap();
        let snapshot_log_glob = format!(
            "**/{}",
            SnapshotLog::TOML_LOG_FILE_NAME
        );
        let task_log_glob = format!(
            "**/{}",
            TaskLog::TOML_LOG_FILE_NAME
        );
        let snapshot_logs = wax::Glob::new(&snapshot_log_glob)
            .unwrap()
            .walk(project_directory)
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| x.file_type().is_file())
            .map(|x| x.into_path())
            .filter_map(|file_path| {
                SnapshotLog::open(&file_path).ok()
            })
            .collect::<LinkedList<_>>();
        let task_logs = wax::Glob::new(&task_log_glob)
            .unwrap()
            .walk(project_directory)
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| x.file_type().is_file())
            .map(|x| x.into_path())
            .filter_map(|file_path| {
                TaskLog::open(&file_path).ok()
            })
            .collect::<LinkedList<_>>();
        Ok(ProjectLog {
            snapshot_logs,
            task_logs,
        })
    }
    pub fn persist_snapshot_log(
        &mut self,
        snapshot_directory: &SnapshotDirectory,
        log: SnapshotLog
    ) -> Result<(), Box<dyn std::error::Error>> {
        log.write(snapshot_directory)?;
        self.snapshot_logs.push_back(log);
        Ok(())
    }
    pub fn persist_task_log(
        &mut self,
        snapshot_directory: &SnapshotDirectory,
        log: TaskLog
    ) -> Result<(), Box<dyn std::error::Error>> {
        log.write(snapshot_directory)?;
        self.task_logs.push_back(log);
        Ok(())
    }
    pub fn should_visit(
        &self,
        given_url: &Url,
        snapshot_directory: &SnapshotDirectory,
    ) -> bool {
        for entry in self.task_logs.iter() {
            if entry.contains_url(given_url, snapshot_directory) {
                return false
            }
        }
        true
    }
}

