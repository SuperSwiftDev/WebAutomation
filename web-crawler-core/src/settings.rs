use std::{collections::BTreeSet, path::PathBuf, str::FromStr};

use indexmap::IndexSet;
use url::Url;

#[derive(Debug, Clone)]
pub struct UrlVisitorSettings {
    pub domain_whitelist: BTreeSet<String>,
    pub protocol_whitelist: BTreeSet<String>,
    pub protocol_blacklist: BTreeSet<String>,
}

impl UrlVisitorSettings {
    pub fn from_domain_whitelist_with_defaults<P: Into<String>>(domain_whitelist: impl IntoIterator<Item=P>) -> Self {
        let this = Self {
            domain_whitelist: domain_whitelist
                .into_iter()
                .map(|x| x.into())
                .map(|x| x.to_ascii_lowercase())
                .collect(),
            protocol_whitelist: Default::default(),
            protocol_blacklist: Default::default(),
        };
        this.normalize().union(Self::standard_defaults())
    }
    pub fn from_seed_urls_with_defaults(urls: impl AsRef<[Url]>) -> Self {
        let domains = urls
            .as_ref()
            .into_iter()
            .map(|x| x.domain().unwrap())
            .collect::<Vec<_>>();
        Self::from_domain_whitelist_with_defaults(domains)
    }
    pub fn standard_defaults() -> Self {
        Self {
            domain_whitelist: Default::default(),
            // protocol_whitelist: Default::default(),
            protocol_whitelist: BTreeSet::from_iter(vec![
                String::from("http"),
                String::from("https"),
            ]),
            protocol_blacklist: BTreeSet::from_iter(vec![
                String::from("tel"),
            ]),
        }
    }
    pub fn merge_mut(&mut self, other: Self) {
        self.domain_whitelist.extend(other.domain_whitelist);
        self.protocol_whitelist.extend(other.protocol_whitelist);
        self.protocol_blacklist.extend(other.protocol_blacklist);
    }
    pub fn union(mut self, other: Self) -> Self {
        self.merge_mut(other);
        self
    }
    pub fn normalize(self) -> Self {
        Self {
            domain_whitelist: self.domain_whitelist
                .into_iter()
                .map(|x| x.to_ascii_lowercase())
                .collect(),
            protocol_whitelist: self.protocol_whitelist
                .into_iter()
                .map(|x| x.to_ascii_lowercase())
                .collect(),
            protocol_blacklist: self.protocol_blacklist
                .into_iter()
                .map(|x| x.to_ascii_lowercase())
                .collect(),
        }
    }
    pub fn with_whitelisted_domain(mut self, entry: impl AsRef<str>) -> Self {
        match Url::from_str(entry.as_ref()) {
            Ok(url) => {
                self.domain_whitelist.insert(url.domain().unwrap().to_ascii_lowercase().to_string());
            }
            Err(_) => {
                self.domain_whitelist.insert(entry.as_ref().to_ascii_lowercase().to_string());
            }
        }
        self
    }
    pub fn with_whitelisted_protocol(mut self, entry: impl Into<String>) -> Self {
        self.protocol_whitelist.insert(entry.into());
        self
    }
    pub fn with_blacklisted_protocol(mut self, entry: impl Into<String>) -> Self {
        self.protocol_blacklist.insert(entry.into());
        self
    }
    pub fn should_visit(&self, url: &Url) -> Result<(), FailedFilterReason> {
        let use_scheme_whitelist = self.protocol_whitelist.is_empty();
        let is_whitelisted_scheme = self.protocol_whitelist.contains(&url.scheme().to_ascii_lowercase());
        let is_blacklisted_scheme = self.protocol_blacklist.contains(&url.scheme().to_ascii_lowercase());
        let is_whitelisted_domain = url.domain().map(|domain| {
            self.domain_whitelist.contains(&domain.to_ascii_lowercase())
        });
        if !is_whitelisted_domain.unwrap_or(false) {
            return Err(FailedFilterReason::FailedWhitelistedDomainCheck)
        }
        if use_scheme_whitelist && !is_whitelisted_scheme {
            return Err(FailedFilterReason::FailedWhitelistedSchemeCheck)
        }
        if is_blacklisted_scheme {
            return Err(FailedFilterReason::IsBlacklistedScheme)
        }
        Ok(())
    }
    /// Check if a domain is a subdomain of any whitelisted domain
    pub fn is_subdomain_of_whitelisted(&self, url: &Url) -> bool {
        let url_domain = match url.domain() {
            Some(d) => d.to_ascii_lowercase(),
            None => return false,
        };
        self.domain_whitelist.iter().any(|allowed| {
            let allowed = allowed.to_ascii_lowercase();
            url_domain == allowed || url_domain.ends_with(&format!(".{}", allowed))
        })
    }
}

#[derive(Debug, Clone)]
pub struct CrawlerSettings {
    pub seed_urls: IndexSet<Url>,
    pub url_visitor_settings: UrlVisitorSettings,
    pub project_directory: PathBuf,
}

// #[derive(Debug, Clone)]
// pub struct FileSystemPaths {
//     pub snapshot_directory: PathBuf,
//     pub manifest_path: PathBuf,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipUrlReason {
    AlreadyVisited,
    FailedFilter(FailedFilterReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailedFilterReason {
    FailedWhitelistedSchemeCheck,
    FailedWhitelistedDomainCheck,
    IsBlacklistedScheme,
}
