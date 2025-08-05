pub mod data;
pub mod wait_framework;
pub mod utils;

use std::i64;
// use std::pin::Pin;

use chromiumoxide::browser::Browser;
use chromiumoxide::Page;
use futures::StreamExt;
use colored::Colorize;

// ————————————————————————————————————————————————————————————————————————————
// SETTINGS
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HeadlessMode {
    /// The "headful" mode.
    False,
    /// The old headless mode.
    #[default]
    True,
    /// The new headless mode. See also: https://developer.chrome.com/docs/chromium/new-headless
    New,
}

#[derive(Debug, Clone, Default)]
pub struct WebClientSettings {
    pub headless_mode: Option<HeadlessMode>,
}

impl WebClientSettings {
    fn chrome_browser_config_builder(&self) -> chromiumoxide::browser::BrowserConfigBuilder {
        let mut builder = chromiumoxide::browser::BrowserConfigBuilder::default();
        if let Some(headless_mode) = self.headless_mode.as_ref() {
            let headless_mode = match headless_mode {
                HeadlessMode::False => chromiumoxide::browser::HeadlessMode::False,
                HeadlessMode::True => chromiumoxide::browser::HeadlessMode::True,
                HeadlessMode::New => chromiumoxide::browser::HeadlessMode::New,
            };
            builder = builder.headless_mode(headless_mode);
        }
        builder
    }
}

// ————————————————————————————————————————————————————————————————————————————
// WEB CLIENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct WebClient {
    browser: Browser,
}

impl WebClient {
    pub async fn start(web_client_settings: WebClientSettings) -> WebClient {
        let browser_config = web_client_settings
            .chrome_browser_config_builder()
            .build()
            .unwrap();
        let (browser, mut handler) = Browser::launch(browser_config).await.unwrap();
        tokio::spawn(async move {
            while let Some(payload) = handler.next().await {
                match payload {
                    Ok(()) => (),
                    Err(error) => {
                        eprintln!(
                            "{}",
                            format!("⚠️ {error}: {error:#?}")
                        );
                        break;
                    }
                }
            }
        });
        WebClient { browser }
    }
    pub async fn close(mut self) {
        let _ = self.browser.close().await.unwrap();
    }
}



impl WebClient {
    pub async fn open_new_tab_at_url(&mut self, url: impl AsRef<str>) -> LiveWebpage {
        let requested_url = url.as_ref().to_string();

        let page = self.browser.new_page(requested_url.clone()).await.unwrap();
        page.wait_for_navigation().await.unwrap();

        let actual_url = page.evaluate("window.location.href").await.unwrap();
        let actual_url = actual_url.value().unwrap().as_str();

        if let Some(final_url) = actual_url {
            if final_url != requested_url {
                // println!("ⓘ Redirected: {} => {}", requested_url, final_url);
                eprintln!("{}", format!(
                    "ⓘ Redirected: {} => {}",
                    requested_url,
                    final_url,
                ).cyan());
            }
        }

        LiveWebpage { page, status_code: None }
    }
}





// —— OPEN REQUEST ————————————————————————————————————————————————————————————

// #[derive(Debug, Clone)]
// pub struct OpenRequest {
//     url: String,
// }

// impl OpenRequest {
//     pub fn new_page_at_url(url: impl Into<String>) -> Self {
//         Self { url: url.into() }
//     }
// }

// impl Into<CreateTargetParams> for OpenRequest {
//     fn into(self) -> CreateTargetParams {
//         CreateTargetParams::builder()
//             .url(self.url)
//             .build()
//             .unwrap()
//     }
// }

// —— WEB CLIENT TAB ——————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct LiveWebpage {
    page: Page,
    status_code: Option<i64>,
}

impl LiveWebpage {
    /// This resolves once the navigation finished and the page is loaded.
    pub async fn wait_for_navigation(&self) {
        let _ = self.page.wait_for_navigation().await.unwrap();
    }
    /// Returns the HTML content of the page
    pub async fn html_content(&self) -> String {
        self.page.content().await.unwrap()
    }
    /// Scrape all anchor links in the DOM tree.
    pub async fn scrape_all_anchor_links(&self) -> Result<Vec<crate::data::Link>, Box<dyn std::error::Error>> {
        let links = crate::data::Link::scrape_all(&self.page).await?;
        Ok(links)
    }
    /// Returns the current url of the page
    pub async fn url(&self) -> Option<String> {
        self.page.url().await.unwrap()
    }
    /// Close this page.
    pub async fn close(self) {
        self.page.close().await.unwrap()
    }
    pub async fn is_text_html_document(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let js = "document.contentType";
        let value = self.evaluate(js).await?;
    
        Ok(value
            .as_str()
            .map(|s| s.eq_ignore_ascii_case("text/html"))
            .unwrap_or(false))
    }
    pub fn status_code(&self) -> Option<i64> {
        self.status_code.clone()
    }
    pub async fn actual_url(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let actual_url = self.page.evaluate("window.location.href").await?;
        let actual_url = actual_url.value().unwrap().as_str().unwrap_or("").to_string();
        Ok(actual_url)
    }
}


impl WebClient {
    pub async fn open_new_tab_at_url_with_network_tracking(
        &mut self,
        url: impl AsRef<str>,
    ) -> LiveWebpage {
        use chromiumoxide::cdp::browser_protocol::network::{EnableParams, EventResponseReceived, ResourceType};
        use futures::StreamExt;

        let requested_url = url.as_ref().to_string();

        // Create a new blank page
        let page = self.browser.new_page("about:blank").await.unwrap();

        page.enable_stealth_mode().await.unwrap();

        // Get the main frame ID (used to identify top-level responses)
        let main_frame_id = page.mainframe().await.unwrap().unwrap();

        // Enable network tracking
        page.execute(EnableParams::default()).await.unwrap();

        // Start listening to response events BEFORE navigation
        let mut responses = page.event_listener::<EventResponseReceived>().await.unwrap();

        // Start navigation, and allow it to fail without panic
        {
            let nav_result = crate::utils::with_timeout_lazy(
                || async {
                    utils::retry_async(
                        || async {
                            page.goto(&requested_url)
                                .await
                                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                        },
                        2,
                        std::time::Duration::from_secs(3),
                    ).await
                },
                std::time::Duration::from_secs(10),
            ).await;
            match nav_result {
                Ok(Ok(_)) => (),
                Ok(Err(error)) => {
                    eprintln!("\t ⚠️ Navigation to {requested_url:?} failed: {error} — continuing anyway.");
                }
                Err(error) => {
                    eprintln!("\t ⚠️ Navigation to {requested_url:?} failed: {error} — continuing anyway.");
                }
            }
        }

        {
            let result = crate::utils::with_timeout_lazy(
                || async {
                    page.wait_for_navigation().await
                },
                std::time::Duration::from_secs(3),
            ).await;
            match result {
                Ok(Ok(_)) => {}
                Ok(Err(error)) => {
                    eprintln!("\t ⚠️ `wait_for_navigation` failed: {error} — falling back to JS polling.");
                    loop {
                        let ready = page.evaluate("document.readyState").await.unwrap();
                        let state = ready.value().unwrap().as_str().unwrap_or("");
                        if state == "complete" || state == "interactive" {
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
                Err(error) => {
                    eprintln!("\t ⚠️ `wait_for_navigation` failed: {error} — falling back to JS polling.");
                    loop {
                        let ready = page.evaluate("document.readyState").await.unwrap();
                        let state = ready.value().unwrap().as_str().unwrap_or("");
                        if state == "complete" || state == "interactive" {
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }
        }

        // Extract the HTTP status code for the main document
        let mut status_code: Option<i64> = None;
        {
            let deadline = tokio::time::sleep(std::time::Duration::from_secs(5));
            tokio::pin!(deadline);

            loop {
                tokio::select! {
                    Some(event) = responses.next() => {
                        if event.r#type != ResourceType::Document {
                            continue;
                        }

                        let frame_match = event.frame_id.as_ref() == Some(&main_frame_id);
                        let url_match = event.response.url == requested_url
                            || event.response.url.starts_with(&requested_url);

                        if frame_match || url_match {
                            status_code = Some(event.response.status);
                            break;
                        }
                    }
                    _ = &mut deadline => {
                        eprintln!("\t ⚠️ Timed out waiting for document response from {:?}", requested_url);
                        break;
                    }
                }
            }
        }

        // Confirm where we landed
        {
            let actual_url = crate::utils::retry_async(
                || async {
                    page.evaluate("window.location.href")
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                },
                3,
                std::time::Duration::from_secs(1),
            ).await;
            let actual_url = actual_url.unwrap();
            let actual_url = actual_url.value().unwrap().as_str().unwrap_or("").to_string();
            if actual_url != requested_url {
                eprintln!("{}", format!(
                    "\t ⓘ Redirected: {} => {}",
                    requested_url,
                    actual_url,
                ).cyan());
            }
        }

        LiveWebpage {
            page,
            status_code,
        }
    }
}
