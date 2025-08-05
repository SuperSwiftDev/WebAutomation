//! # Google Maps Local Entity Extraction (tbm=map Ingest Model)
//!
//! This module provides a resilient, loss-minimizing data model for parsing and structuring
//! Google's `tbm=map` JSON payloads — i.e., the *Map Pack* and Local Finder results embedded in Google Search.
//!
//! These results are *not part of Google Maps API* and are instead derived from obfuscated,
//! undocumented array-based structures returned by the `tbm=map` internal endpoint. This model
//! reverse-engineers and normalizes these payloads into a typed format suitable for:
//!
//! - 📍 **Local business discovery & extraction**
//! - 🎯 **Lead generation pipelines (organic & paid listings)**
//! - 🧠 **Category classification via GCID/Knowledge Graph signals**
//! - 🔎 **Programmatic SEO auditing, brand monitoring, and presence tracking**
//!
//! ---
//!
//! ## 🧠 Domain Context
//!
//! Google Map Pack results play a critical role in the **local intent funnel** — from
//! discovery to conversion — and this model is designed to power use cases such as:
//!
//! - **Reverse engineering local SERPs** to model visibility and rank position
//! - **Entity resolution** across Google’s Place ID, Feature ID, and KG ID namespaces
//! - **Franchise detection** via `parent_chain_name` and location clusters
//! - **Structured citation comparison** for NAP consistency
//! - **Monetized lead capture from GMB presence and clickstream metadata**
//!
//! ---
//!
//! ## 🛠️ Engineering Strategy
//!
//! ✅ **Typed Resilience over Schema Drift**  
//! Google frequently modifies the array layout, keys, and nesting levels.
//! To handle this, we model core fields (`BusinessEntity`, `MapViewport`, `AdMetadata`) using
//! Rust’s `Option<T>`, `Vec<T>`, and typed wrappers where consistency is observed.
//!
//! ✅ **Multiple Identifier Support**  
//! Entities are linkable across ecosystems using any combination of:
//! - `place_id`: Canonical map URL parameter
//! - `kg_id`: Knowledge Graph slug used in `/g/...` URLs
//! - `feature_id`: Deep map tile anchor (e.g. `0x87...:0xdd...`)
//!
//! ✅ **Organic + Paid Results Unified**  
//! Listings are normalized regardless of ad type:
//! - GMB organic pins  
//! - Sponsored Map Ads (AdWords / Local Extensions)  
//! - Service Area Businesses (SABs)  
//! - Franchise locations or co-located chains
//!
//! ✅ **Optimized for Lead Quality Signals**  
//! Fields such as `adwords_url`, `phone_number`, `business_url`, `open_hours`, and `category_label`
//! allow for high-precision contact and conversion scoring.
//!
//! ---
//!
//! ## 🔬 Reverse Engineering Protocol
//!
//! This crate was built via analysis of hundreds of real-world search payloads,
//! including geo-qualified queries (`"roofing contractors near me"`), branded searches,
//! service-category lookups, and proximity-triggered queries.
//!
//! Fields are included **only if they are observed with consistency and semantic utility.**
//!
//! When encountering unexpected arrays or opaque blobs, preserve them using `serde_json::Value` in the `raw` block.
//!
//! ---
//!
//! ## 🔧 Key Types
//!
//! - [`GoogleMapSearchPayload`] — top-level object parsed from `tbm=map` results
//! - [`BusinessEntity`] — normalized representation of a single local listing
//! - [`AdMetadata`] — sponsored content and context keywords (optional)
//! - [`StructuredAddress`] — full breakdown of geographic & postal components
//! - [`PhoneNumbers`] — normalized and canonicalized contact formats
//! - [`BusinessHours`] — parsed weekly schedule with `TimeRange` blocks
//! - [`CategoryInference`] — GCID-ranked output from Google's internal classifier
//!
//! ---
//!
//! ## 📈 Lead Engineering Integration
//!
//! - Combine `BusinessEntity` records with SERP metadata (position, ad/non-ad split)  
//! - Ingest into lead scoring systems based on `rating`, `review_count`, and category match  
//! - Correlate `adwords_url` and `gmb_owner_id` with verified advertiser segments  
//! - Identify duplicate/franchise presences via `parent_chain_name` + `kg_id` clustering  
//!
//! ---
//!
//! ## ⚠️ Caveats & Future Work
//!
//! - Google’s structure is fluid and purposefully obfuscated; weekly regression testing is recommended  
//! - SABs (Service Area Businesses) often lack full address data — expect degraded geolocation accuracy  
//! - Some listings are not anchored to Place IDs (especially new ads); fallback to Feature ID or maps URL  
//! - Rich result components (menus, products, availability) are currently excluded, but mappable  
//!
//! Contributions should aim to extend type coverage, improve drift resistance, and document new dimensions as they emerge.

use std::collections::HashMap;
use serde::Serialize;

/// Root-level representation of a tbm=map search result payload from Google Search.
///
/// This structure includes search metadata, UI rendering instructions, and result blocks.
/// It's not formally documented by Google, so all structure is reverse-engineered.
///
/// ⚠️ Many fields are optional or loosely typed due to Google's obfuscation
/// and version changes. Use Option<T> and Vec<T> liberally.
#[derive(Debug, Clone, Serialize)]
pub struct GoogleMapSearchPayload {
    /// The search query metadata block.
    pub search_metadata: Option<SearchMetadata>,

    /// Camera configuration: viewport size, lat/lng center, zoom level.
    pub map_viewport: Option<MapViewport>,

    /// Possibly the raw zoom or rendering state; structure varies.
    pub view_state: Option<serde_json::Value>,

    /// Session token or identifier (varies by user).
    pub session_token: Option<String>,

    /// Collection of parsed business entities from the result set.
    pub listings: Vec<BusinessEntity>,

    /// Optional internal facets or grouping metadata.
    pub categorization: Option<CategorizationMetadata>,

    /// Optional: result context such as region disambiguation.
    pub resolved_location: Option<LocationContext>,
}


#[derive(Debug, Clone, Serialize)]
pub struct CategorizationMetadata {
    /// Internal codes or tag structure like:
    /// [["software_company", null, 0.97579694], ...]
    pub inferred_categories: Vec<CategoryCandidate>,

    /// Optional Google "tagline" or query-match confidence
    pub overall_confidence: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MapViewport {
    /// Zoom-level bounding box or tile radius.
    pub bounding_distance: f64,

    /// Center longitude
    pub lng: f64,

    /// Center latitude
    pub lat: f64,

    /// Screen dimensions: width x height
    pub screen_size: (u32, u32),

    /// Zoom level (e.g. 13.1)
    pub zoom: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchMetadata {
    /// Original query, like "software company"
    pub query: String,

    /// Optional autocomplete ID or session ID.
    pub session_id: Option<String>,

    /// Unix timestamp (milliseconds) of search execution.
    pub timestamp_ms: Option<i64>,
}

pub struct GoogleEntityIndex {
    pub kg_id: String,
    pub gcid: Option<String>,
    pub name: Option<String>,
    pub canonical_url: Option<String>,
    pub geo: Option<(f64, f64)>,
}

/// Represents a structured business listing result from Google Maps Search (`tbm=map` backend).
///
/// This model unifies organic and sponsored entries into a normalized structure
/// for consistent downstream processing (indexing, ranking, deduplication, UI, etc.).
///
/// ⚠️ Fields are `Option<T>` or `Vec<T>` due to structural variability across
/// listings (ads vs. organic, sparse entries, test buckets, etc.).
///
/// Designed for extensibility — expect `serde_json::Value` in low-confidence zones.
///
/// Examples of business types this handles:
/// - Verified GMB listings
/// - Sponsored Ads (with Adwords metadata)
/// - Map-pack organic results
/// - Service-area businesses
#[derive(Debug, Clone, Serialize)]
pub struct BusinessEntity {
    // ─────────────────────────────────────────────────────────────
    // 🆔 IDENTIFIERS
    // ─────────────────────────────────────────────────────────────

    /// Place ID is Google's primary identifier for physical locations.
    ///
    /// Stable across most APIs (Maps, Places, Local Services).
    ///
    /// Example: `ChIJzz1YxS2AUocR27XFrpjOe90`
    pub place_id: Option<String>,

    /// Feature ID is a lower-level internal identifier often found in URLs.
    ///
    /// Format: `0x<geo>:0x<entity>` — used in tile rendering, internal pin links, or event logs.
    ///
    /// Example: `0x8752802dc5583dcf:0xdd7bce98aec5b5db`
    pub feature_id: Option<String>,

    /// Knowledge Graph ID (KG ID), used across Google's Knowledge Panels.
    ///
    /// Format: `/g/11...`, stable for branded and notable entities.
    ///
    /// Useful for cross-linking search results and disambiguating large brands.
    pub kg_id: Option<String>,

    /// Google My Business (GMB) Owner ID — a long numeric identifier linking to the business account.
    ///
    /// Only appears in certain contributor or ownership contexts.
    ///
    /// Example: `100543022764104941349`
    pub gmb_owner_id: Option<String>,

    // ─────────────────────────────────────────────────────────────
    // 🏷️ CATEGORY INFORMATION
    // ─────────────────────────────────────────────────────────────

    /// GCID (Google Category ID) — internal slug representing business type.
    ///
    /// Example: `software_company`, `general_contractor`, `home_builder`
    pub category_id: Option<String>,

    /// Human-facing label of the category.
    ///
    /// Displayed in UI elements such as headings, breadcrumbs, or chips.
    ///
    /// Example: `"Software company"`
    pub category_label: Option<String>,

    /// Additional categories from structured blocks or taxonomy tags.
    ///
    /// Example: `["General contractor", "Custom home builder"]`
    pub categories: Vec<String>,

    /// Google's inferred business categories with ranking/confidence.
    ///
    /// Extracted from blocks like:
    /// `[[["software_company", null, 0.9757], ...], 0.9757]`
    pub inferred_categories: Option<CategoryInference>,

    /// Optional parent chain or franchise name.
    ///
    /// Used for national or regional brands like `"Domino's Pizza"` or `"FedEx Office"`.
    ///
    /// Useful for visual grouping, deduplication, or analytics rollup.
    pub parent_chain_name: Option<String>,

    // ─────────────────────────────────────────────────────────────
    // 📍 LOCATION & ADDRESSING
    // ─────────────────────────────────────────────────────────────

    /// Full address block (as displayed to user).
    ///
    /// May be missing for service-area businesses or ads.
    pub full_address: Option<String>,

    /// Street-level component, extracted separately if available.
    ///
    /// Example: `"4193 W 2010 N"`
    pub street_address: Option<String>,

    /// Structured breakdown of address components (city, state, zip, etc.)
    ///
    /// Normalized and useful for geographic filters.
    pub structured_address: Option<StructuredAddress>,

    /// Precise location of the business as `lat/lng`.
    ///
    /// Can be extracted from pins or metadata blocks.
    pub location: Option<GeoPoint>,

    /// Approximate distance from the map viewport center (in meters).
    ///
    /// Not directly returned; computed during post-processing.
    pub distance_from_center_meters: Option<f64>,

    /// Timezone in Olson format (e.g. `America/Denver`).
    ///
    /// Derived from regional context or metadata.
    pub timezone: Option<String>,

    /// Maps link or `/g/...` short URL to open the business in Google Maps.
    ///
    /// Can be resolved to a direct Google preview page.
    pub maps_url: Option<String>,

    /// Global GMB entity ID — less common, sometimes used in other APIs.
    ///
    /// Sometimes overlaps with `place_id` or `feature_id`.
    pub gmb_id: Option<String>,

    /// Disambiguated geographic context inferred from the query.
    ///
    /// Example: `"Saratoga Springs, UT"`
    pub resolved_location: Option<LocationContext>,

    // ─────────────────────────────────────────────────────────────
    // 🌐 WEB, PHONE & CONTACT
    // ─────────────────────────────────────────────────────────────

    /// Homepage of the business.
    ///
    /// Includes both the actual link and the display string (e.g. `chatbooks.com`)
    pub homepage: Option<HomepageUrl>,

    /// Canonical contact number in E.164 format: `+1 801-331-7100`
    pub phone_number: Option<String>,

    /// Multiple variants of phone number: raw, tel URI, formatted, etc.
    pub phone: Option<PhoneNumbers>,

    /// Business website URL (alternate to homepage block).
    ///
    /// Sometimes surfaced in structured schema blocks.
    pub website_url: Option<String>,

    // ─────────────────────────────────────────────────────────────
    // 📝 BUSINESS PROFILE & PRESENTATION
    // ─────────────────────────────────────────────────────────────

    /// Display name / title of the business.
    pub name: Option<String>,

    /// Sponsored result ad headline (often contains call-to-action).
    pub ad_headline: Option<String>,

    /// Rating value (e.g. 4.5), out of 5 stars.
    pub rating: Option<f32>,

    /// Total number of user reviews.
    pub review_count: Option<u32>,

    /// Open/closed summary string (human readable).
    ///
    /// Examples:
    /// - `"Closed ⋅ Opens 10 AM Mon"`
    /// - `"Open 24 hours"`
    pub open_status_summary: Option<String>,

    /// Business hours structured by day of week.
    ///
    /// Includes support for split shifts, "Closed" markers, 24hr.
    pub hours: Option<BusinessHours>,

    /// Visual assets — photo IDs, captions, CDN links, dimensions.
    ///
    /// May require deduplication if repeated in multiple sizes.
    pub photos: Vec<Photo>,

    /// Additional structured tags such as:
    /// - `"Online appointments"`
    /// - `"On-site services"`
    pub service_options: Vec<ServiceOption>,

    // ─────────────────────────────────────────────────────────────
    // 💰 ADS & SPONSORED DATA
    // ─────────────────────────────────────────────────────────────

    /// Google AdWords tracking redirect URL.
    ///
    /// Begins with `https://www.googleadservices.com/pagead/aclk?...`
    pub adwords_url: Option<String>,

    /// Metadata specific to sponsored results (e.g. display domain, final URL).
    pub ad_metadata: Option<AdMetadata>,
}


/// Full business homepage info including display string and actual link.
#[derive(Debug, Clone, Serialize)]
pub struct HomepageUrl {
    /// Full canonical URL: `https://www.chatbooks.com/`
    pub url: String,

    /// Displayed UI version: `chatbooks.com`
    pub display: String,
}

/// Structured model output from Google’s category inference engine.
#[derive(Debug, Clone, Serialize)]
pub struct CategoryInference {
    /// Most likely GCID category ID.
    pub primary: String,

    /// Confidence for the primary category.
    pub confidence: f32,

    /// Additional ranked alternate candidates.
    pub alternates: Vec<CategoryCandidate>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryCandidate {
    pub category_id: String,       // e.g., "software_company"
    pub label: Option<String>,     // UI name, if available
    pub confidence: f32,
}

/// Location context resolved from user query.
#[derive(Debug, Clone, Serialize)]
pub struct LocationContext {
    /// Display name, e.g., "Saratoga Springs, UT"
    pub name: String,

    /// Latitude of the resolved location
    pub lat: f64,

    /// Longitude of the resolved location
    pub lng: f64,

    /// Feature ID used for map pins, like `0x874d7ec...:0xc54...`
    pub feature_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoPoint {
    /// Latitude in decimal degrees
    pub lat: f64,

    /// Longitude in decimal degrees
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StructuredAddress {
    /// e.g. `"2408 Alesund Way"`
    pub street: Option<String>,

    /// e.g. `"Lehi"`
    pub city: Option<String>,

    /// e.g. `"UT"` or `"Utah"`
    pub state: Option<String>,

    /// e.g. `"84043"`
    pub postal_code: Option<String>,

    /// e.g. `"US"` or `"United States"`
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhoneNumbers {
    /// Raw user-facing format, e.g. `(408) 930-4963`
    pub raw: Option<String>,

    /// E.164 international format, e.g. `+1 408-930-4963`
    pub international: Option<String>,

    /// URI-style tel link: `tel:+14089304963`
    pub tel_uri: Option<String>,

    /// Sometimes `formatted` differs from `raw` slightly
    pub formatted: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessHours {
    /// Per-day mapping: 0 = Sunday, 1 = Monday, ...
    ///
    /// Use `summary` if you just need a user-friendly label.
    pub weekly: HashMap<u8, DailyHours>,

    /// Full open/close summary text, if provided separately.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyHours {
    /// Label text such as `"Open 24 hours"` or `"Closed"`
    pub label: String,

    /// Parsed numeric intervals, e.g. 12:00–19:00
    pub intervals: Vec<TimeRange>,

    /// Explicit closed status
    pub is_closed: bool,
}

/// Represents a single open–close time window during a business day.
///
/// This structure reflects Google's internal representation of daily hours,
/// encoded as four integers in 24-hour time format:
///
/// - `[start_hour, start_minute, end_hour, end_minute]`
///
/// Used inside per-day schedules where businesses may have one or more
/// open-close blocks (e.g., lunch break closures).
///
/// ⚠️ This is a low-level representation. For display purposes,
/// convert to localized time and format accordingly.
#[derive(Debug, Clone, Serialize)]
pub struct TimeRange {
    /// Hour at which the business opens (24-hour format).
    /// 
    /// Range: 0–23
    pub start_hour: u8,

    /// Minute at which the business opens.
    ///
    /// Range: 0–59
    pub start_minute: u8,

    /// Hour at which the business closes (24-hour format).
    ///
    /// Range: 0–23
    pub end_hour: u8,

    /// Minute at which the business closes.
    ///
    /// Range: 0–59
    pub end_minute: u8,
}


#[derive(Debug, Clone, Serialize)]
pub struct Photo {
    /// Unique Google image ID
    pub photo_id: String,

    /// Optional caption text, if labeled (e.g. "Canyon Ridge Builders")
    pub caption: Option<String>,

    /// Full image URL to CDN-hosted photo
    pub image_url: String,

    /// Dimensions in pixels, if parsed
    pub dimensions: Option<(u32, u32)>,

    /// Embedded lat/lng metadata
    pub location_hint: Option<GeoPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdMetadata {
    /// Title-style heading used in ad creatives
    pub ad_headline: Option<String>,

    /// Body text snippet summarizing offering or value
    pub ad_text: Option<String>,

    /// The display domain, e.g. `"ivoryhomes.com"`
    pub display_url: Option<String>,

    /// Actual final landing URL (if available)
    pub final_url: Option<String>,

    /// Google redirect tracking URL
    pub google_click_url: Option<String>,

    /// Targeting themes (e.g. `"Utah Retirement Homes"`)
    pub context_keywords: Vec<String>,

    /// Explanation for why the ad was shown
    pub ad_disclaimer: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceOption {
    /// Internal service type slug (e.g. `"onsite_services"`)
    pub service_type: String,

    /// Display label like `"Onsite services"`
    pub label: String,

    /// Whether Google indicates this option is available
    pub is_available: bool,

    /// Alternative wording or marketing copy
    pub alt_text: Option<String>,
}

