use std::path::{Path, PathBuf};

pub mod specification;

#[derive(Debug, Clone)]
pub struct ManifestContext {
    spec: specification::ManifestSpec,
    #[allow(unused)]
    file_path: PathBuf,
}

impl ManifestContext {
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let file = std::fs::read_to_string(file_path)?;
        let spec = toml::from_str::<specification::ManifestSpec>(&file)?;
        // let base_path = file_path.parent().unwrap();
        // let spec = spec.normalize(&base_path);
        Ok(Self {
            spec,
            file_path: file_path.to_path_buf(),
        })
    }
    pub fn get_project(&self, id: impl AsRef<str>) -> Option<&specification::ProjectSpec> {
        self.spec.projects.iter().find(|project| {
            project.id == id.as_ref()
        })
    }
}

