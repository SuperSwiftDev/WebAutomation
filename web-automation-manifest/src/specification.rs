// use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct WhitelistSpec {
//     pub domains: Vec<String>,
// }

// impl WhitelistSpec {
//     pub fn normalize(self, _: &Path) -> Self {
//         Self {
//             domains: self.domains,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
// pub struct BlacklistSpec {
//     pub protocols: Vec<String>,
// }

// impl BlacklistSpec {
//     pub fn normalize(self, _: &Path) -> Self {
//         Self {
//             protocols: self.protocols
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSpec {
    pub id: String,
    pub seed_urls: Vec<String>,
    // pub output_dir: PathBuf,
}

// impl ProjectSpec {
//     pub fn normalize(self, base_path: &Path) -> Self {
//         Self {
//             id: self.id,
//             seed_urls: self.seed_urls,
//             // output_dir: base_path.join(self.output_dir),
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSpec {
    pub projects: Vec<ProjectSpec>,
}

// impl ManifestSpec {
//     pub fn normalize(self, base_path: &Path) -> Self {
//         Self {
//             projects: self.projects
//                 .into_iter()
//                 .map(|x| x.normalize(base_path))
//                 .collect()
//         }
//     }
// }
