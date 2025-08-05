#![allow(unused)]
pub mod system;
pub mod cli;
pub mod manifest;
pub mod path_utils;
pub mod data;

#[tokio::main]
async fn main() {
    let command_line_interface = cli::CommandLineInterface::load();
    command_line_interface.execute().await
}
