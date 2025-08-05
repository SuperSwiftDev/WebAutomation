use std::{
    collections::HashMap, path::PathBuf, str::FromStr, sync::Arc
};

use chromiumoxide::{
    browser::{Browser, BrowserConfigBuilder, HeadlessMode},
    cdp::browser_protocol::network::{
        EnableParams, EventLoadingFinished, EventResponseReceived, GetResponseBodyParams,
        RequestId, SetCacheDisabledParams,
    },
    page::Page,
};

use colored::Colorize;
use tokio::sync::Mutex;
use futures::StreamExt;
use url::Url;

use crate::data::Timestamp;

type SharedResponses = Arc<Mutex<HashMap<RequestId, EventResponseReceived>>>;

#[derive(Debug, Clone)]
pub struct Settings {
    pub url: Url,
    pub output_directory: PathBuf,
    pub session_timestamp: Timestamp,
}

pub async fn start(settings: Settings) {
    eprintln!("> RUNNING");

    // 🎬 Launch the browser
    let (browser, mut handler) = Browser::launch(
        BrowserConfigBuilder::default()
            .headless_mode(HeadlessMode::False)
            .window_size(1200, 800)
            .build()
            .unwrap(),
    )
    .await
    .unwrap();

    // 🌐 Spawn handler loop
    tokio::spawn(async move {
        while let Some(event) = handler.next().await {
            if let Err(err) = event {
                eprintln!("⚠️ Chromium handler error: {:#?}", err);
                break;
            }
        }
    });

    // let page = browser.new_page(url.as_str()).await.unwrap();
    
    // Create a new blank page
    let page = browser.new_page("about:blank").await.unwrap();

    // Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.5296.0 Safari/537.36
    page.set_user_agent(
            // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"
            "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.5296.0 Safari/537.36"
        )
        .await
        .unwrap();

    let page = page.goto(settings.url.as_str()).await.unwrap();

    // Setup: Disable cache + Enable network
    page.execute(SetCacheDisabledParams {
        cache_disabled: true,
    })
    .await
    .unwrap();

    page.execute(EnableParams::default()).await.unwrap();

    // 🗂️ Shared map of request_id -> response metadata
    let pending_responses: SharedResponses = Arc::new(Mutex::new(HashMap::new()));

    // 🎧 Listen to ResponseReceived
    {
        let mut responses = page.event_listener::<EventResponseReceived>().await.unwrap();
        let shared_map = Arc::clone(&pending_responses);

        tokio::spawn(async move {
            while let Some(event) = responses.next().await {
                let request_id = event.request_id.clone();
                let mut map = shared_map.lock().await;
                let event = Arc::unwrap_or_clone(event);
                map.insert(request_id, event);
            }
        });
    }

    // 🎧 Listen to LoadingFinished
    {
        let mut finishes = page.event_listener::<EventLoadingFinished>().await.unwrap();
        let page = page.clone();
        let shared_map = Arc::clone(&pending_responses);

        tokio::spawn(async move {
            while let Some(event) = finishes.next().await {
                let request_id = &event.request_id;
                let maybe_resp = {
                    let mut map = shared_map.lock().await;
                    map.remove(request_id)
                };

                if let Some(resp) = maybe_resp {
                    process_complete_response(&page, resp, &settings).await;
                }
            }
        });
    }

    // let _ = page.goto(url.as_str()).await.unwrap();

    // Wait for the page to load
    page.wait_for_navigation().await.unwrap();

    eprintln!("{}", format!("👀 Monitoring... Press Ctrl+C to exit.").green());
    tokio::signal::ctrl_c().await.expect("Failed to wait for Ctrl+C");
    eprintln!("{}", format!("👋 Done.").green());
}

async fn process_complete_response(
    page: &Page,
    resp: EventResponseReceived,
    settings: &Settings
) {
    let mime_type = &resp.response.mime_type;
    let is_json_mime_type = mime_type.to_ascii_lowercase() == "application/json";
    let url = resp.response.url.as_str();

    // Skip irrelevant types
    let is_blacklisted = mime_type.contains("image")
        || mime_type.contains("font")
        || mime_type.contains("application/manifest+json")
        || mime_type.contains("application/javascript")
        || mime_type.contains("text/javascript")
        || mime_type.contains("text/css")
        || mime_type.contains("text/html")
        || mime_type.contains("application/vnd.google.octet-stream-compressible")
        || resp.response.url.contains("https://fonts.gstatic.com/l/font")
        ;

    if is_blacklisted {
        return;
    }

    match page.execute(GetResponseBodyParams { request_id: resp.request_id.clone() }).await {
        Ok(body) => {
            let body_string = if body.base64_encoded {
                use base64::Engine;
                let result = base64::engine::general_purpose::STANDARD.decode(&body.body).unwrap();
                let result = String::from_utf8(result).unwrap();
                result
            } else {
                body.body.clone()
            };
            if is_json_mime_type {
                let directory_tree = crate::path_utils::build_rel_html_snapshot_dir(url).unwrap();
                let output_dir = settings.output_directory.join(directory_tree);
                let mut index = 0usize;
                let timestamp = settings.session_timestamp.0.as_str();
                loop {
                    let out_json_path = output_dir.join(format!("{timestamp}.original.{index}.json"));
                    let output_url_path = output_dir.join(format!("{timestamp}.original.{index}.url"));
                    let output_deserialized_path = output_dir.join(format!("{timestamp}.deserialized.{index}.url"));
                    if !out_json_path.exists() {
                        std::fs::create_dir_all(out_json_path.parent().unwrap()).unwrap();
                        std::fs::write(&out_json_path, &body_string).unwrap();
                        // - -
                        let output_url_str = url.to_string();
                        std::fs::write(&output_url_path, &output_url_str).unwrap();
                        // - -
                        if google_types::map_search::process::is_valid_url(url) {
                            let business_entities =
                                google_types::map_search::process::parse_business_entities(&body_string)
                                .unwrap();
                            let output_str = serde_json::to_string_pretty(&business_entities).unwrap();
                            std::fs::write(&output_deserialized_path, &output_str).unwrap();
                        }
                        // - -
                        break;
                    }
                    index = index + 1;
                };
                eprintln!("{}", format!("❍ {} [{}]: [saved]", url, mime_type.bold()).cyan());
            } else {
                eprintln!("{}", format!(
                    "❍ {} [{}]:\n{}",
                    url, mime_type.bold(), body_string.blue().on_black()
                ).cyan());
            }
        }
        Err(err) => {
            eprintln!("{}", format!(
                "❌ Error fetching body for {}: {:?}",
                resp.response.url, err
            ).red());
        }
    }
}

