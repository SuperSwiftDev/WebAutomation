use serde::{Serialize, Deserialize};

// pub static SCRAPE_ANCHORS: &'static str = include_str!("../snippets/scrape_anchors.js");
pub static SCRAPE_ANCHORS: &'static str = include_str!("../snippets/scrape_anchors.all.robust.js");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Link {
    pub href: String,
    pub text: String,
}

impl Link {
    pub fn parse_list(source: impl AsRef<str>) -> Result<Vec<Link>, serde_json::Error> {
        serde_json::from_str::<Vec<Link>>(source.as_ref())
    }
    pub async fn scrape_all(page: &chromiumoxide::Page) -> Result<Vec<Link>, serde_json::Error> {
        let result = crate::retry::retry_async(
            || async {
                page
                    .evaluate(SCRAPE_ANCHORS)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            },
            5,
            std::time::Duration::from_secs(5),
        ).await.unwrap();
        let json_value = result.clone().into_value::<serde_json::Value>().unwrap();
        // let json_value = serde_json::to_string(&json_value).unwrap();
        // eprintln!("GIVEN: {}", json_value);

        // let result = page
        //     .evaluate(SCRAPE_ANCHORS)
        //     .await
        //     .unwrap();
        // result.into_value::<Vec<Link>>()
        let result = serde_json::from_value::<Vec<Link>>(json_value).unwrap();
        // let result = serde_json::from_str::<Vec<Link>>(&json_value).unwrap();
        Ok(result)
    }
}

