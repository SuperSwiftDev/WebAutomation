// use std::str::FromStr;
// use url::Url;

pub fn is_valid_url(url: &str) -> bool {
    // let url = Url::from_str(url).unwrap();
    // url.path()
    url.starts_with("https://www.google.com/search?tbm=map")
}

pub fn parse_business_entities(payload: &str) -> Result<Vec<super::deserialize::BusinessEntity>, Box<dyn std::error::Error>> {
    let value = serde_json::from_str::<serde_json::Value>(payload)?;
    let entries = super::deserialize::ingest_payload_extract_business_entities(&value);
    Ok(entries)
}