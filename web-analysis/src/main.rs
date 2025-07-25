extern crate super_html_ast as html_ast;

pub mod pass;
pub mod process;

use std::path::PathBuf;

fn main() {
    let root_directory = PathBuf::from(".web-crawler/spider-cloud");
    process::process_all_snapshots(&root_directory);
}
