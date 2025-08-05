use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Run(RunCli),
}

#[derive(Parser, Debug)]
struct RunCli {
    /// Path to the project directory.
    pub directory: PathBuf,
}

impl CommandLineInterface {
    pub fn load() -> Self {
        Self::parse()
    }
    pub fn execute(self) {
        match self.command {
            SubCommand::Run(build) => build.execute(),
        }
    }
}

impl RunCli {
    pub fn execute(self) {
        crate::process::process_all_snapshots(&self.directory);
    }
}


