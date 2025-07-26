use std::collections::{HashSet, LinkedList};
use std::path::PathBuf;
use std::{collections::VecDeque, str::FromStr};
use indexmap::IndexSet;
use url::Url;
use colored::Colorize;

use crate::metadata::common::{CanonicalUrl, OriginalUrl, RelativeFilePath, SnapshotDate, SnapshotDirectory, Status};
use crate::metadata::project::ProjectLog;
use crate::metadata::snapshot::{SnapshotLog, TaskLog};
// use crate::db::{SnapshotManifest, VisitedPage, WebpageSnapshotMetadata};
use crate::settings::{FailedFilterReason, SkipUrlReason};

pub use crate::settings::CrawlerSettings;

pub struct WebCrawler {
    crawler_settings: CrawlerSettings,
    project: ProjectLog,
    queue: VecDeque<Url>,
    fully_resolved: HashSet<Url>,
}

// fn get_actual_url<'a>(
//     tab: &'a web_client_bot::LiveWebpage,
//     counter: usize,
//     limit: usize,
// ) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
//     Box::pin(async move {
//         match tab.actual_url().await {
//             Ok(url) => Ok(url),
//             Err(err) => {
//                 if counter < limit {
//                     tokio::time::sleep(Duration::from_secs(1)).await;
//                     let result = get_actual_url(tab, counter + 1, limit).await;
//                     result
//                 } else {
//                     Err(err)
//                 }
//             }
//         }
//     })
// }

// ————————————————————————————————————————————————————————————————————————————
// PUBLIC API
// ————————————————————————————————————————————————————————————————————————————

impl WebCrawler {
    pub fn new(crawler_settings: CrawlerSettings) -> Self {
        let project_directory = crawler_settings.project_directory.as_path();
        let project = ProjectLog::load(project_directory).unwrap();
        // - -
        let mut all_outbound_links = LinkedList::<Url>::new();
        all_outbound_links.extend(crawler_settings.seed_urls.clone());
        for entry in project.snapshot_logs.iter() {
            all_outbound_links.extend({
                entry.outgoing_links
                    .iter()
                    .map(|x| x.0.clone())
            });
        }
        let all_outbound_links = IndexSet::<Url>::from_iter(all_outbound_links);
        let queue = all_outbound_links
            .into_iter()
            .filter(|url| {
                let canonical_url = CanonicalUrl::from_url(url.clone());
                let ( html_output_path, _ ) = Self::snapshot_file_path(url, &crawler_settings);
                let snapshot_directory = SnapshotDirectory(html_output_path.parent().unwrap().to_path_buf());
                let should_visit_original_url = Self::should_visit(url, &snapshot_directory, &crawler_settings, &project).is_ok();
                let should_visit_canonical_url = Self::should_visit(&canonical_url.0, &snapshot_directory, &crawler_settings, &project).is_ok();
                should_visit_original_url && should_visit_canonical_url
            })
            .collect::<VecDeque<_>>();
        // let queue = VecDeque::from_iter(all_outbound_links);
        // - -
        Self {
            queue,
            project,
            crawler_settings,
            fully_resolved: Default::default(),
        }
    }
    // pub fn write_snapshot_manifest(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let file_path = self.crawler_settings.file_system_paths.manifest_path.as_path();
    //     let _ = self.project.write(file_path)?;
    //     Ok(())
    // }
    pub async fn execute(&mut self) {
        let mut client = web_client_bot::WebClient::start().await;
        while let Some(task) = self.queue.pop_front() {
            self.process_url(&task, &mut client).await;
        }
        client.close().await;
        // self.write_snapshot_manifest().unwrap();
    }
    pub fn finalize(self) -> (CrawlerSettings, ProjectLog) {
        (self.crawler_settings, self.project)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// CRAWL URL
// ————————————————————————————————————————————————————————————————————————————

impl WebCrawler {
    fn should_visit(
        url: &Url,
        snapshot_directory: &SnapshotDirectory,
        crawler_settings: &CrawlerSettings,
        project: &ProjectLog,
    ) -> Result<(), SkipUrlReason> {
        let already_visited = !project.should_visit(url, snapshot_directory);
        let filtered_out = crawler_settings.url_visitor_settings.should_visit(url);
        if already_visited {
            return Err(SkipUrlReason::AlreadyVisited)
        }
        if let Err(rejected_url_reason) = filtered_out {
            return Err(SkipUrlReason::FailedFilter(rejected_url_reason))
        }
        Ok(())
    }
    fn should_visit_with_debug_log(
        &self,
        url: &Url,
        snapshot_directory: &SnapshotDirectory,
    ) -> Result<(), SkipUrlReason> {
        if let Err(skip_url_reason) = Self::should_visit(url, snapshot_directory, &self.crawler_settings, &self.project) {
            let msg = match skip_url_reason.clone() {
                SkipUrlReason::AlreadyVisited => {
                    format!("\t ⓘ Skipping [already-visited] {:?}", url.to_string())
                    // return false
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::FailedWhitelistedSchemeCheck) => {
                    format!("\t ⓘ Skipping [failed-whitelisted-scheme-check] {:?}", url.to_string())
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::FailedWhitelistedDomainCheck) => {
                    format!("\t ⓘ Skipping [failed-whitelisted-domain-check] {:?}", url.to_string())
                }
                SkipUrlReason::FailedFilter(FailedFilterReason::IsBlacklistedScheme) => {
                    format!("\t ⓘ Skipping [is-blacklisted-scheme] {:?}", url.to_string())
                }
            };
            eprintln!("{}", msg.cyan());
            return Err(skip_url_reason)
        }
        Ok(())
    }
    fn snapshot_file_path(url: &Url, crawler_settings: &CrawlerSettings) -> ( PathBuf, RelativeFilePath ) {
        let relative_snapshot_path = crate::path_utils::build_rel_html_snapshot_file_path(url.as_str()).unwrap();
        let html_path = crawler_settings.project_directory.join(&relative_snapshot_path.0);
        ( html_path, relative_snapshot_path )
    }
    fn enqueue_urls(
        &mut self,
        urls: impl IntoIterator<Item=Url>,
        snapshot_directory: &SnapshotDirectory,
    ) {
        let enqueue = urls
            .into_iter()
            .map(|x| x.into())
            .filter(|url| {
                let status = self.should_visit_with_debug_log(url, &snapshot_directory);
                // let is_fully_resolved = self.fully_resolved.contains(url);
                // let is_already_visited = &status == &Err(SkipUrlReason::AlreadyVisited);
                // if !skip_already_visited && is_already_visited && !is_fully_resolved {
                //     return true
                // }
                status.is_ok()
            })
            .collect::<Vec<_>>();
        for next in enqueue {
            self.queue.push_back(next.clone());
        }
    }
    async fn process_url(&mut self, url: &Url, client: &mut web_client_bot::WebClient) {
        let canonical_url = CanonicalUrl::from_url(url.clone());
        let ( html_output_path, rel_html_path ) = Self::snapshot_file_path(url, &self.crawler_settings);
        let snapshot_directory = SnapshotDirectory(html_output_path.parent().unwrap().to_path_buf());
        let mut task_log = TaskLog::default();
        // - CHECK SHOULD SKIP -
        {
            let mut status: Result<(), SkipUrlReason> = Ok(());
            if self.fully_resolved.contains(url) || self.fully_resolved.contains(&canonical_url.0) {
                return
            }
            if let Err(error) = self.should_visit_with_debug_log(&url, &snapshot_directory) {
                status = Err(error);
            }
            if let Err(error) = self.should_visit_with_debug_log(&canonical_url.0, &snapshot_directory) {
                status = Err(error);
            }
            if status.is_err() {
                // - TERMINATE -
                return
            }
        }
        eprintln!("{}", format!("🔎 Visiting: {}", canonical_url.0).bright_magenta());
        eprintln!("{}", format!("\t 1. TODO").bright_yellow());
        // - -
        let tab = client.open_new_tab_at_url_with_network_tracking(canonical_url.0.as_str()).await;
        eprintln!("{}", format!("\t 2. TODO").bright_yellow());
        let status_code = tab.status_code();
        if status_code != Some(200) {
            let should_terminate = status_code.is_none();
            if should_terminate {
                eprintln!("{}", format!(
                    "\t Skipping {:?} » status: {status_code:?}",
                    url.to_string(),
                ).bright_red());
            } else {
                eprintln!("{}", format!(
                    "\t Error {:?} » status: {status_code:?}",
                    url.to_string(),
                ).red());
            }
            // - -
            task_log.entries.push(Status::Failure {
                url: OriginalUrl(url.clone()),
                http_status: status_code,
            });
            // - -
            if should_terminate {
                self.project.persist_task_log(&snapshot_directory, task_log).unwrap();
                tab.close().await;
                self.fully_resolved.insert(url.to_owned());
                self.fully_resolved.insert(canonical_url.0.clone());
                // - TERMINATE -
                return
            }
        }
        // - -
        eprintln!("{}", format!("\t 3. TODO").bright_yellow());
        // {
        //     let _ = web_client_bot::utils::with_timeout(
        //         tab.wait_for_navigation(),
        //         std::time::Duration::from_secs(3),
        //     ).await; // IGNORE POSSIBLE TIMEOUT ERRORS
        // }
        // - -
        eprintln!("{}", format!("\t 4. TODO").bright_yellow());
        let actual_url = {
            let url = web_client_bot::utils::retry_async(
                || async {
                    tab.actual_url().await
                },
                3,
                std::time::Duration::from_secs(3),
            ).await;
            let url = url.unwrap();
            Url::from_str(&url).unwrap()
        };
        let did_redirect = &actual_url != url;
        if did_redirect {
            task_log.entries.push(Status::Redirected {
                from: OriginalUrl::from(url.clone()),
                to: OriginalUrl::from(actual_url.clone()),
                http_status: status_code,
            });
        }
        eprintln!("{}", format!("\t 5. TODO").bright_yellow());
        // - -
        {
            let result = web_client_bot::utils::with_timeout_lazy(
                || async {
                    tab.is_text_html_document().await
                },
                std::time::Duration::from_secs(3),
            ).await;
            match result {
                Ok(Ok(true)) => (),
                Ok(Ok(false)) => {
                    eprintln!("{}", format!(
                        "\t ⓘ Skipping {:?} : NOT HTML DOCUMENT",
                        url.to_string()
                    ).red());
                    tab.close().await;
                    // - UPDATE -
                    self.fully_resolved.insert(url.to_owned());
                    self.fully_resolved.insert(canonical_url.0.clone());
                    task_log.entries.push(Status::Failure {
                        url: OriginalUrl(url.clone()),
                        http_status: status_code,
                    });
                    self.project.persist_task_log(&snapshot_directory, task_log).unwrap();
                    // - TERMINATE -
                    return
                },
                Ok(Err(error)) => {
                    eprintln!("{}", format!(
                        "\t ⓘ Skipping {:?} : ERROR CHECKING FOR HTML DOCUMENT : {error}",
                        url.to_string()
                    ).red());
                    tab.close().await;
                    // - UPDATE -
                    self.fully_resolved.insert(url.to_owned());
                    self.fully_resolved.insert(canonical_url.0.clone());
                    task_log.entries.push(Status::Failure {
                        url: OriginalUrl(url.clone()),
                        http_status: status_code,
                    });
                    self.project.persist_task_log(&snapshot_directory, task_log).unwrap();
                    // - TERMINATE -
                    return
                }
                Err(timeout_error) => {
                    unimplemented!("TODO: {timeout_error}")
                }
            }
        }
        // match tab.is_text_html_document().await {
        //     Ok(true) => (),
        //     Ok(false) => {
        //         eprintln!("{}", format!(
        //             "\t ⓘ Skipping {:?} : NOT HTML DOCUMENT",
        //             url.to_string()
        //         ).red());
        //         tab.close().await;
        //         // - UPDATE -
        //         self.fully_resolved.insert(url.to_owned());
        //         self.fully_resolved.insert(canonical_url.0.clone());
        //         task_log.entries.push(Status::Failure {
        //             url: OriginalUrl(url.clone()),
        //             http_status: status_code,
        //         });
        //         self.project.persist_task_log(&snapshot_directory, task_log).unwrap();
        //         // - TERMINATE -
        //         return
        //     },
        //     Err(error) => {
        //         eprintln!("{}", format!(
        //             "\t ⓘ Skipping {:?} : ERROR CHECKING FOR HTML DOCUMENT : {error}",
        //             url.to_string()
        //         ).red());
        //         tab.close().await;
        //         // - UPDATE -
        //         self.fully_resolved.insert(url.to_owned());
        //         self.fully_resolved.insert(canonical_url.0.clone());
        //         task_log.entries.push(Status::Failure {
        //             url: OriginalUrl(url.clone()),
        //             http_status: status_code,
        //         });
        //         self.project.persist_task_log(&snapshot_directory, task_log).unwrap();
        //         // - TERMINATE -
        //         return
        //     }
        // };
        eprintln!("{}", format!("\t 6. TODO").bright_yellow());
        {
            let maybe_fully_settled = web_client_bot::utils::retry_on_timeout(
                "wait_until_fully_settled",
                || async {
                    tab .wait_until_fully_settled()
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                },
                1,
                std::time::Duration::from_secs(1),
                std::time::Duration::from_secs(5),
            ).await;
            if let Err(error) = maybe_fully_settled {
                eprintln!("{}", format!(
                    "\t ❌ Failed to settle: {:?} » {error}",
                    url.as_str()
                ).red());
            }
        }
        // - DOM SNAPSHOT -
        eprintln!("{}", format!("\t 7. TODO").bright_yellow());
        let html = tab.html_content().await;
        eprintln!("{}", format!("\t 8. TODO").bright_yellow());
        let outgoing_anchors_links = tab.scrape_all_anchor_links().await.unwrap();
        let outgoing_anchors = outgoing_anchors_links
            .iter()
            .filter_map(|link| {
                match Url::from_str(&link.href) {
                    Err(error) => {
                        eprintln!("{}", format!(
                            "\t ⓘ Skipping {:?} : {error}",
                            link.href,
                        ));
                        None
                    }
                    Ok(x) => Some(x),
                }
            })
            .collect::<IndexSet<_>>();
        // - UPDATE QUEUE -
        self.enqueue_urls(outgoing_anchors.clone(), &snapshot_directory);
        // - FINALIZE -
        {
            std::fs::create_dir_all(html_output_path.parent().unwrap()).unwrap();
            std::fs::write(html_output_path, &html).unwrap();
            // - -
            let outgoing_links = outgoing_anchors
                .iter()
                .map(|url| {
                    OriginalUrl::from(url.to_owned())
                })
                .collect::<IndexSet<_>>();
            // - SNAPSHOT LOG -
            self.project.persist_snapshot_log(&snapshot_directory, {
                SnapshotLog {
                    http_status: status_code,
                    original_url: OriginalUrl(url.clone()),
                    canonical_url: canonical_url.clone(),
                    snapshot_path: rel_html_path,
                    snapshot_date: SnapshotDate::now(),
                    outgoing_links,
                    incoming_links: Default::default(),
                }
            }).unwrap();
            // - TASK LOG -
            self.project.persist_task_log(&snapshot_directory, {
                task_log.entries.push(Status::Success {
                    url: OriginalUrl(url.clone()),
                    http_status: status_code,
                });
                task_log
            }).unwrap();
            // - UPDATE -
            self.fully_resolved.insert(url.to_owned());
            self.fully_resolved.insert(canonical_url.0.clone());
        }
        eprintln!("{}", format!("\t 9. TODO").bright_yellow());
        // - CLOSE -
        tab.close().await;
    }
}
