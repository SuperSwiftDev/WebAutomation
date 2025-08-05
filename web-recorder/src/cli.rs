use std::path::PathBuf;
use clap::{Parser, Subcommand};
use url::Url;

use crate::{data::Timestamp, manifest::ManifestSpec};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Open(OpenCli),
    Run(RunCli),
}

#[derive(Parser, Debug)]
struct OpenCli {
    /// TODO
    // #[arg(long)]
    pub url: Url,
}

#[derive(Parser, Debug)]
struct RunCli {
    /// TODO
    // #[arg(long)]
    pub manifest_path: PathBuf,
    /// Name of the site ro run.
    #[arg(short, long)]
    pub id: String,
    #[arg(short, long)]
    pub session: Option<String>,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub async fn execute(self) {
        match self.command {
            SubCommand::Open(build) => build.execute().await,
            SubCommand::Run(run) => run.execute().await,
        }
    }
}

impl OpenCli {
    pub async fn execute(self) {
        let settings = crate::system::Settings {
            url: self.url.clone(),
            output_directory: PathBuf::from(".web-recorder"),
            session_timestamp: Timestamp::now(),
        };
        crate::system::start(settings).await;
    }
}

impl RunCli {
    pub async fn execute(self) {
        let manifest_str = std::fs::read_to_string(&self.manifest_path).unwrap();
        let manifest = toml::from_str::<ManifestSpec>(&manifest_str).unwrap();
        let site = manifest.sites
            .iter()
            .find(|x| x.id.as_str() == self.id.as_str())
            .expect("the specified site as defined in the given manifest");
        let mut output_directory = PathBuf::from(".web-recorder");
        if let Some(name) = self.session.as_ref() {
            output_directory = output_directory.join(name);
        }
        let settings = crate::system::Settings {
            url: site.url.clone(),
            output_directory,
            session_timestamp: Timestamp::now(),
        };
        crate::system::start(settings).await
    }
}


