use std::pin::Pin;
use std::time::Duration;
use std::{collections::VecDeque, str::FromStr};
use indexmap::IndexSet;
use url::Url;
use colored::Colorize;

use crate::common::{CanonicalUrl, OriginalUrl, SnapshotDate};
use crate::db::{SnapshotManifest, VisitedPage, WebpageSnapshotMetadata};
use crate::settings::{FailedFilterReason, SkipUrlReason};

pub use crate::settings::CrawlerSettings;

pub struct WebCrawler {
    crawler_settings: CrawlerSettings,
    snapshot_manifest: SnapshotManifest,
    queue: VecDeque<Url>,
}

fn get_actual_url<'a>(
    tab: &'a web_client_bot::WebClientTab,
    counter: usize,
    limit: usize,
) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
    Box::pin(async move {
        match tab.actual_url().await {
            Ok(url) => Ok(url),
            Err(err) => {
                if counter < limit {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    let result = get_actual_url(tab, counter + 1, limit).await;
                    result
                } else {
                    Err(err)
                }
            }
        }
    })
}

// ————————————————————————————————————————————————————————————————————————————
// PUBLIC API
// ————————————————————————————————————————————————————————————————————————————

impl WebCrawler {
    pub fn new(crawler_settings: CrawlerSettings) -> Self {
        let manifest_path = crawler_settings.file_system_paths.manifest_path.as_path();
        let snapshot_manifest = SnapshotManifest::load_or_default(manifest_path);
        let queue = VecDeque::from_iter(crawler_settings.seed_urls.clone());
        Self {
            queue,
            snapshot_manifest,
            crawler_settings,
        }
    }
    pub fn write_snapshot_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.crawler_settings.file_system_paths.manifest_path.as_path();
        let _ = self.snapshot_manifest.write(file_path)?;
        Ok(())
    }
    pub async fn execute(&mut self) {
        let mut client = web_client_bot::WebClient::start().await;
        while let Some(task) = self.queue.pop_front() {
            self.process_url(task, &mut client).await;
        }
        client.close().await;
        self.write_snapshot_manifest().unwrap();
    }
    pub fn finalize(self) -> (CrawlerSettings, SnapshotManifest) {
        (self.crawler_settings, self.snapshot_manifest)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// CRAWL URL
// ————————————————————————————————————————————————————————————————————————————

impl WebCrawler {
    fn should_visit(&self, url: &Url) -> Result<(), SkipUrlReason> {
        let already_visited = !self.snapshot_manifest.should_visit(url);
        let filtered_out = self.crawler_settings.url_visitor_settings.should_visit(url);
        if already_visited {
            return Err(SkipUrlReason::AlreadyVisited)
        }
        if let Err(rejected_url_reason) = filtered_out {
            return Err(SkipUrlReason::FailedFilter(rejected_url_reason))
        }
        Ok(())
    }
    fn should_visit_with_debug_log(&self, url: &Url) -> bool {
        if let Err(skip_url_reason) = self.should_visit(&url) {
            let msg = match skip_url_reason {
                SkipUrlReason::AlreadyVisited => {
                    return false
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::FailedWhitelistedSchemeCheck) => {
                    format!("ⓘ Skipping [failed-whitelisted-scheme-check] {:?}", url.to_string())
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::FailedWhitelistedDomainCheck) => {
                    format!("ⓘ Skipping [failed-whitelisted-domain-check] {:?}", url.to_string())
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::IsBlacklistedScheme) => {
                    format!("ⓘ Skipping [is-blacklisted-scheme] {:?}", url.to_string())
                }
            };
            eprintln!("{}", msg.cyan());
            return false
        }
        true
    }
    async fn process_url(&mut self, url: Url, client: &mut web_client_bot::WebClient) {
        let canonical_url = CanonicalUrl::from_url(url.clone());
        // eprintln!("HERE! {:?}", task);
        if !self.should_visit_with_debug_log(&url) {
            return
        }
        if !self.should_visit_with_debug_log(&canonical_url.0) {
            return
        }
        eprintln!("{}", format!("🔎 Visiting: {}", canonical_url.0).bright_green());
        // - -
        let tab = client.open_new_tab_at_url_with_network_tracking(canonical_url.0.as_str()).await;
        let status_code = tab.status_code();
        if status_code != Some(200) {
            eprintln!("{}", format!(
                "❌ Skipping {:?} : Request failed STATUS={status_code:?}",
                url.to_string(),
            ));
            let entry = VisitedPage::Failure {
                url: url.clone().into(),
                http_status: status_code,
            };
            self.snapshot_manifest.visited_pages.insert(entry);
            // tab.close().await;
            // return
        }
        // - -
        // let _ = tokio::time::sleep(Duration::from_secs(3)).await;
        // - -
        tab.wait_for_navigation().await;
        // - -
        let actual_url = {
            let url = get_actual_url(&tab, 0, 3).await.unwrap();
            Url::from_str(&url).unwrap()
        };
        let did_redirect = actual_url != url;
        if did_redirect {
            let entry = VisitedPage::Redirected {
                from: OriginalUrl::from(url.clone()),
                to: OriginalUrl::from(actual_url.clone()),
                http_status: status_code,
            };
            self.snapshot_manifest.visited_pages.insert(entry);
        }
        // - -
        match tab.is_text_html_document().await {
            Ok(true) => (),
            Ok(false) => {
                eprintln!("{}", format!(
                    "ⓘ Skipping {:?} : NOT HTML DOCUMENT",
                    url.to_string()
                ).red());
                tab.close().await;
                return
            },
            Err(error) => {
                eprintln!("{}", format!(
                    "ⓘ Skipping {:?} : ERROR CHECKING FOR HTML DOCUMENT : {error}",
                    url.to_string()
                ).red());
                tab.close().await;
                return
            }
        };
        if let Err(e) = tab.wait_until_fully_settled().await {
            eprint!("{}", format!(
                "❌ Failed to settle: {} | {e}",
                url.as_str()
            ).red());
            tab.close().await;
            return
        }
        // - DOM SNAPSHOT -
        let html = tab.html_content().await;
        let outgoing_anchors_links = tab.scrape_all_anchor_links().await.unwrap();
        let outgoing_anchors = outgoing_anchors_links
            .iter()
            .filter_map(|link| {
                match Url::from_str(&link.href) {
                    Err(error) => {
                        eprintln!("{}", format!(
                            "ⓘ Skipping {:?} : {error}",
                            link.href,
                        ));
                        None
                    }
                    Ok(x) => Some(x),
                }
            })
            .collect::<IndexSet<_>>();
        // - UPDATE QUEUE -
        let enqueue_urls = outgoing_anchors
            .iter()
            .filter(|url| {
                self.should_visit_with_debug_log(&url)
            })
            .collect::<Vec<_>>();
        for next in enqueue_urls {
            self.queue.push_back(next.clone());
        }
        // - FINALIZE -
        let relative_snapshot_path = crate::utils::build_rel_html_snapshot_file_path(url.as_str()).unwrap();
        let out_file_path = self.crawler_settings.file_system_paths.snapshot_directory.join(&relative_snapshot_path.0);
        let outgoing_links = outgoing_anchors
            .iter()
            .map(|url| {
                OriginalUrl::from(url.to_owned())
            })
            .collect::<IndexSet<_>>();
        let webpage_snapshot_metadata = WebpageSnapshotMetadata {
            http_status: status_code,
            original_url: OriginalUrl(url.clone()),
            canonical_url,
            snapshot_path: relative_snapshot_path,
            snapshot_date: SnapshotDate::now(),
            outgoing_links,
            incoming_links: Default::default(),
        };
        std::fs::create_dir_all(out_file_path.parent().unwrap()).unwrap();
        std::fs::write(out_file_path, &html).unwrap();
        self.snapshot_manifest.snapshots.push(webpage_snapshot_metadata);
        self.snapshot_manifest.visited_pages.insert({
            VisitedPage::Success { url: OriginalUrl(url), http_status: status_code }
        });
        // - CLOSE -
        tab.close().await;
    }
}
