use std::path::{Path, PathBuf};
use std::str::FromStr;

use indexmap::IndexSet;
use url::Url;
use web_crawler_core::{settings::{FileSystemPaths, UrlVisitorSettings}, CrawlerSettings, WebCrawler};

pub mod cli;

#[tokio::main]
async fn main() {
    let command_line_interface = cli::CommandLineInterface::load();
    command_line_interface.execute().await
}

// async fn dev() {
//     let project_id = "rsseau";
//     let manifest_path = PathBuf::from("web-automation.toml");
//     evaluate(manifest_path, project_id).await
// }

pub async fn evaluate(file_path: impl AsRef<Path>, project_id: &str) {
    let file_path = file_path.as_ref();
    let manifest = web_automation_manifest::ManifestContext::load(&file_path).unwrap();
    let project = manifest.get_project(project_id).unwrap();
    let seed_urls = project.seed_urls
        .iter()
        .map(|x| Url::from_str(x).unwrap())
        .collect::<Vec<_>>();
    run(seed_urls, project_id).await;
}

pub async fn run(seed_urls: impl IntoIterator<Item=Url>, id: &str) {
    let seed_urls = seed_urls.into_iter().collect::<Vec<_>>();
    let snapshot_directory = PathBuf::from(".web-crawler").join(id);
    let manifest_path = snapshot_directory.join("manifest.toml");
    let url_visitor_settings = UrlVisitorSettings::from_seed_urls_with_defaults(&seed_urls);
    let crawler_settings = CrawlerSettings {
        seed_urls: IndexSet::from_iter(seed_urls),
        url_visitor_settings,
        file_system_paths: FileSystemPaths {
            snapshot_directory,
            manifest_path,
        },
    };
    let mut web_crawler = WebCrawler::new(crawler_settings);
    web_crawler.execute().await;
    let (crawler_settings, snapshot_manifest) = web_crawler.finalize();
    // - TODO -
    let _ = crawler_settings;
    let _ = snapshot_manifest;
}
