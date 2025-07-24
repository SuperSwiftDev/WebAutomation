use std::path::PathBuf;
use std::str::FromStr;

use indexmap::IndexSet;
use url::Url;
use web_crawler_core::{settings::{FileSystemPaths, UrlVisitorSettings}, CrawlerSettings, WebCrawler};


#[tokio::main]
async fn main() {
    let seed_urls = vec![
        Url::from_str("https://rsseau.fr/en/").unwrap(),
    ];
    let snapshot_directory = PathBuf::from(".web-crawler");
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
    let _ = crawler_settings;
    let _ = snapshot_manifest;
}
