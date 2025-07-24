use std::path::PathBuf;

use url::Url;

use crate::common::RelativeFilePath;

const MAX_SEGMENT_LEN: usize = 100;
const MAX_PATH_LEN: usize = 240;

// const MAX_SEGMENT_LEN: usize = 64;
// const MAX_PATH_LEN: usize = 255;

/// Build relative file path for a given URL.
/// Regarding the parent directory — falls back to hashed folder if path becomes too long or unsafe.
pub fn build_rel_html_snapshot_file_path(url: &str) -> Option<RelativeFilePath> {
    Some(RelativeFilePath(build_rel_html_snapshot_dir(url).map(|base| {
        base.join("snapshot.html")
    })?))
}
/// Build directory path for a given URL, including query parameters.
/// Falls back to hashed folder if path becomes too long or unsafe.
pub fn build_rel_html_snapshot_dir(url: &str) -> Option<PathBuf> {
    use sha2::{Digest, Sha256};

    fn sanitize(s: &str) -> String {
        s.chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
            .collect()
    }

    fn short_hash(s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let hash = hasher.finalize();
        let short = hash[..4]
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        format!("h{short}")
    }

    let parsed = Url::parse(url).ok()?;
    let host = parsed.host_str().unwrap_or("unknown");

    let mut path = parsed.path().trim_matches('/').to_string();
    if path.is_empty() {
        path = "index".to_string();
    }

    let query = parsed.query().unwrap_or("");
    let query_suffix = if !query.is_empty() {
        let sanitized_query = sanitize(query);
        format!("~q~{sanitized_query}")
    } else {
        String::new()
    };

    let mut full_path = PathBuf::from(host);
    let mut total_len = host.len();

    let mut segments: Vec<String> = path
        .split('/')
        .map(sanitize)
        .collect();

    // Append sanitized query to the last segment
    if !query_suffix.is_empty() {
        if let Some(last) = segments.last_mut() {
            last.push_str(&query_suffix);
        }
    }

    for seg in &segments {
        total_len += seg.len() + 1;
        if seg.len() > MAX_SEGMENT_LEN || total_len > MAX_PATH_LEN {
            let hashed = short_hash(url);
            return Some(PathBuf::from(host).join("long").join(hashed));
        }
        full_path = full_path.join(seg);
    }

    Some(full_path)
}
