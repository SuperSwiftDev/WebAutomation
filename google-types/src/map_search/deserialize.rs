#![allow(unused)]
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use std::convert::TryFrom;
use serde::Serialize;
use serde_json::Value;
use colored::Colorize;

// ————————————————————————————————————————————————————————————————————————————
// ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

pub fn ingest_payload_extract_business_entities(value: &Value) -> Vec<BusinessEntity> {
    value
        .pointer("/64")
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(ingest_entry)
        .filter_map(|result| {
            match result {
                SearchResultEntity::OrganicListing(x) => Some(x),
                SearchResultEntity::AdvertisedListing => None,
            }
        })
        .collect::<Vec<_>>()
}

fn ingest_entry(value: &Value) -> Option<SearchResultEntity> {
    let items = value.as_array().unwrap();
    assert!(items.get(0).unwrap().is_null());
    if items.len() == 2 && value.get(1).unwrap().is_array() {
        let entity = ingest_organic_entry(value).unwrap();
        return Some(SearchResultEntity::OrganicListing(entity))
    } else {
        let _ = ingest_sponsored_or_other_entry(value);
        return Some(SearchResultEntity::AdvertisedListing)
    }
}

fn ingest_sponsored_or_other_entry(value: &Value) {
    // let items = value.as_array().unwrap();
    //
    // TODO: LOW PRIORITY
    //
}

fn ingest_organic_entry(value: &Value) -> Option<BusinessEntity> {
    let items = value.as_array().unwrap();
    assert!(items.len() == 2);
    assert!(items.get(0).unwrap().is_null());
    // —— MAIN ————————————————————————————————————————————————————————————————
    let items = value.get(1).unwrap().as_array().unwrap();
    // —— MAIN ————————————————————————————————————————————————————————————————
    ingest_organic_entry_business_entity(items)
}

fn ingest_organic_entry_business_entity(items: &[Value]) -> Option<BusinessEntity> {
    let mut skip_list = HashSet::<usize>::default();
    let mut todo_skip_list = HashSet::<usize>::default();
    let mut handled_list = Rc::<RefCell<HashSet::<usize>>>::default();
    let get_business_token = || {
        let target_index: usize = 0;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_serp_tracking_token = || {
        let target_index: usize = 1;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_address_lines = || {
        let target_index: usize = 2;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Some(target)
    };
    let get_reviews = || {
        let target_index: usize = 4;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = super::deserialize::ReviewSummaryBlock::from_raw(target).unwrap();
        Some(target)
    };
    let get_business_website_links = || {
        let target_index: usize = 7;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None;
        }
        let target = target.as_array().unwrap();
        let url = target.get(0).unwrap().as_str().unwrap().to_string();
        let domain = target.get(1).unwrap().as_str().unwrap().to_string();
        Some(BusinessWebsiteLinks {
            url,
            domain,
        })
    };
    let get_geo_location = || {
        let target_index: usize = 9;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None;
        }
        let target = target.as_array().unwrap();
        let lat = target.get(2).unwrap().as_f64().unwrap();
        let lon = target.get(3).unwrap().as_f64().unwrap();
        Some(GeoPoint { latitude: lat, longitude: lon })
    };
    let get_composite_business_id = || {
        let target_index: usize = 10;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_display_name = || {
        let target_index: usize = 11;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_business_category_labels = || {
        let target_index: usize = 13;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None;
        }
        let target = target
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| {
                x.as_str().unwrap().to_string()
            })
            .collect::<Vec<_>>();
        Some(target)
    };
    let get_neighborhood_name = || {
        let target_index: usize = 14;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_display_title_address_line = || {
        let target_index: usize = 18;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None;
        }
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    skip_list.insert(24);
    skip_list.insert(25);
    let get_timezone_field = || {
        let target_index: usize = 30;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    skip_list.insert(32);
    let get_hours_of_operation = || {
        let target_index: usize = 34;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: HIGH PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_media_photo_metadata = || {
        let target_index: usize = 37;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target
            .as_array()
            .unwrap()
            .into_iter()
            .filter_map(|item| {
                photo::PhotoWrapper::try_from(item).ok()
            })
            .collect::<Vec<_>>();
        Some(target)
    };
    let get_formatted_address = || {
        let target_index: usize = 39;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    skip_list.insert(40);
    let get_claim_this_business_metadata = || {
        let target_index: usize = 49;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_gmb_account_ownership_metadata_block = || {
        let target_index: usize = 57;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    skip_list.insert(61);
    skip_list.insert(67);
    let get_business_photo_gallery = || {
        let target_index: usize = 72;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target
            .as_array()
            .unwrap()
            .into_iter()
            .filter_map(|item| {
                photo::PhotoWrapper::try_from(item).ok()
            })
            .collect::<Vec<_>>();
        Some(target)
    };
    let get_business_cta_links = || {
        let target_index: usize = 75;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target.pointer("/0/0").unwrap();
        let fulfillment_integration = FulfillmentIntegration::try_from(target).unwrap();
        Some(fulfillment_integration)
    };
    let get_business_categories = || {
        let target_index: usize = 76;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        Some(BusinessCategoryVector::from_raw(target).unwrap())
    };
    let get_google_place_id = || {
        let target_index: usize = 78;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_address_block_array = || {
        let target_index: usize = 82;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_business_display_block = || {
        let target_index: usize = 88;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_google_knowledge_graph_id = || {
        let target_index: usize = 89;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_local_business_ontology_categories = || {
        let target_index: usize = 100;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let feature_set = feature_set::FeatureSet::from_raw(target).unwrap();
        Some(feature_set)
    };
    skip_list.insert(105);
    skip_list.insert(110);
    skip_list.insert(112);
    let get_another_language_code = || {
        let target_index: usize = 113;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        None
    };
    skip_list.insert(118);
    let get_service_taxonomy = || {
        let target_index: usize = 125;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let result = service_cluster::ServiceCluster::try_from(target).unwrap();
        Some(result)
    };
    skip_list.insert(130);
    skip_list.insert(134);
    skip_list.insert(136);
    let get_business_level_attributes = || {
        let target_index: usize = 142;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    skip_list.insert(146);
    let get_google_account_avatars = || {
        let target_index: usize = 157;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    skip_list.insert(160);
    let get_business_category_label_tuple = || {
        let target_index: usize = 164;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: LOW PRIORITY
        // UI RELATED — PROBABLY SKIPPABLE
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    skip_list.insert(165);
    let get_locality_string_city_level_location_names = || {
        let target_index: usize = 166;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    skip_list.insert(170);
    let get_indirect_business_profile_links = || {
        let target_index: usize = 174;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let target = target.as_array().unwrap();
        let target = match &target.as_slice() {
            [x] => x,
            xs => panic!("UNHANDLED CASE: {}", serde_json::to_string(xs).unwrap()),
        };
        let target = target.as_str().unwrap().to_string();
        Some(target)
    };
    let get_phone_number_entity_block = || {
        let target_index: usize = 178;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let phone_entry = phone::PhoneEntry::try_from(target).unwrap();
        Some(phone_entry)
    };
    skip_list.insert(181);
    let get_location_normalization_payload = || {
        let target_index: usize = 183;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None;
        }
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_identity_label_metadata = || {
        let target_index: usize = 196;
        handled_list.borrow_mut().insert(target_index);
        let _ = items.get(target_index).unwrap();
        //
        // NOTE: PROBABLY SKIP THIS
        //
        None
    };
    let get_structured_business_hours_payload = || {
        let target_index: usize = 203;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let business_hours_block = business_hours::BusinessHoursBlock::from_value(target).unwrap();
        Some(business_hours_block)
    };
    skip_list.insert(205);
    let get_coordinate_blocks = || {
        pub fn parse_lat_lng_points(value: &Value) -> anyhow::Result<Vec<GeoPoint>> {
            let outer = value.as_array().ok_or_else(|| anyhow::anyhow!("Expected array of points"))?;

            outer.iter()
                .map(|v| {
                    let inner = v.as_array().ok_or_else(|| anyhow::anyhow!("Expected inner array"))?;
                    let lat = inner.get(2).and_then(|v| v.as_f64()).ok_or_else(|| anyhow::anyhow!("Missing lat"))?;
                    let lng = inner.get(3).and_then(|v| v.as_f64()).ok_or_else(|| anyhow::anyhow!("Missing lng"))?;
                    Ok(GeoPoint { latitude: lat, longitude: lng })
                })
                .collect()
        }
        let target_index: usize = 208;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let geo_point = parse_lat_lng_points(target).unwrap();
        let geo_point = match &geo_point.as_slice() {
            &[x] => x,
            xs => panic!("UNHANDLED: {}", serde_json::to_string(xs).unwrap().red()),
        };
        let geo_point = geo_point.to_owned();
        Some(geo_point)
    };
    let get_b64_structured_concept_identifiers = || {
        let target_index: usize = 209;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        //
        // TODO: NOT SURE HOW TO HANDLE THIS
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        // let target = target.as_str().unwrap();
        // let decoded = {
        //     use base64::Engine;
        //     let padded = match target.len() % 4 {
        //         2 => format!("{target}=="),
        //         3 => format!("{target}="),
        //         0 => target.to_string(),
        //         _ => panic!("Invalid base64 string length"),
        //     };

        //     let result = base64::engine::general_purpose::STANDARD.decode(&padded).unwrap();
        //     let result = String::from_utf8(result).unwrap();
        //     result
        // };
        None
    };
    skip_list.insert(214);
    let get_business_claim_urls = || {
        let target_index: usize = 226;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        //
        // TODO: LOW PRIORITY
        //
        // eprintln!("{}", format!(
        //     "» {}",
        //     target,
        // ).yellow());
        None
    };
    let get_google_maps_internal_business_entity_identifiers = || {
        let target_index: usize = 227;
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        let business_entity_identifier = 
            business_entity_identifiers::BusinessEntityIdentifier::from_value(
                target
            )
            .unwrap();
        // - -
        Some(business_entity_identifier)
    };
    skip_list.insert(243);
    let get_geographic_contextualization_metadata = || {
        let target_index: usize = 245;
        if items.len() != 246 {
            return None
        }
        handled_list.borrow_mut().insert(target_index);
        let target = items.get(target_index).unwrap();
        if target.is_null() {
            return None
        }
        let geo_containment_hierarchy =
            contextualization_metadata::parse_geo_containment_hierarchy(target)
            .unwrap();
        Some(geo_containment_hierarchy)
    };
    let result = BusinessEntity { 
        business_token: get_business_token(),
        serp_tracking_token: get_serp_tracking_token(),
        address_lines: get_address_lines(),
        reviews: get_reviews(),
        business_website_links: get_business_website_links(),
        geo_location: get_geo_location(),
        composite_business_id: get_composite_business_id(),
        display_name: get_display_name(),
        business_category_labels: get_business_category_labels(),
        neighborhood_name: get_neighborhood_name(),
        display_title_address_line: get_display_title_address_line(),
        timezone_field: get_timezone_field(),
        hours_of_operation: get_hours_of_operation(),
        media_photo_metadata: get_media_photo_metadata(),
        formatted_address: get_formatted_address(),
        claim_this_business_metadata: get_claim_this_business_metadata(),
        gmb_account_ownership_metadata_block: get_gmb_account_ownership_metadata_block(),
        business_photo_gallery: get_business_photo_gallery(),
        business_cta_links: get_business_cta_links(),
        business_categories: get_business_categories(),
        google_place_id: get_google_place_id(),
        address_block_array: get_address_block_array(),
        business_display_block: get_business_display_block(),
        google_knowledge_graph_id: get_google_knowledge_graph_id(),
        local_business_ontology_categories: get_local_business_ontology_categories(),
        another_language_code: get_another_language_code(),
        service_taxonomy: get_service_taxonomy(),
        business_level_attributes: get_business_level_attributes(),
        google_account_avatars: get_google_account_avatars(),
        business_category_label_tuple: get_business_category_label_tuple(),
        locality_string_city_level_location_names: get_locality_string_city_level_location_names(),
        indirect_business_profile_links: get_indirect_business_profile_links(),
        phone_number_entity_block: get_phone_number_entity_block(),
        location_normalization_payload: get_location_normalization_payload(),
        identity_label_metadata: get_identity_label_metadata(),
        structured_business_hours_payload: get_structured_business_hours_payload(),
        coordinate_blocks: get_coordinate_blocks(),
        b64_structured_concept_identifiers: get_b64_structured_concept_identifiers(),
        business_claim_urls: get_business_claim_urls(),
        google_maps_internal_business_entity_identifiers: get_google_maps_internal_business_entity_identifiers(),
        geographic_contextualization_metadata: get_geographic_contextualization_metadata(),
    };
    for (index, _) in items.iter().enumerate() {
        if skip_list.contains(&index) {
            continue;
        } else if todo_skip_list.contains(&index) {
            eprintln!("{}", format!(
                "❌ [todo] {index}: {}",
                serde_json::to_string(items.get(index).unwrap()).unwrap(),
            ).yellow());
            continue;
        } else if handled_list.borrow().contains(&index) {
            continue;
        } else {
            if !items.get(index).unwrap().is_null() {
                eprintln!("{}", format!(
                    "❌ [unhandled] {index}: {}",
                    serde_json::to_string(items.get(index).unwrap()).unwrap(),
                ).yellow());
            }
            assert!(items.get(index).unwrap().is_null());
        }
    }
    Some(result)
}


// ————————————————————————————————————————————————————————————————————————————
// DATA TYPES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Serialize)]
pub struct GoogleMapSearchPayload {
    /// The search query metadata block.
    pub search_metadata: Option<SearchMetadata>,
    /// Collection of parsed business entities from the result set.
    pub listings: Vec<SearchResultEntity>,
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

#[derive(Debug, Clone, Serialize)]
pub enum SearchResultEntity {
    AdvertisedListing,
    OrganicListing(BusinessEntity),
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessEntity {
    pub business_token: Option<String>,
    pub serp_tracking_token: Option<String>,
    pub address_lines: Option<Vec<String>>,
    pub reviews: Option<ReviewSummaryBlock>,
    pub business_website_links: Option<BusinessWebsiteLinks>,
    pub geo_location: Option<GeoPoint>,
    pub composite_business_id: Option<String>,
    pub display_name: Option<String>,
    pub business_category_labels: Option<Vec<String>>,
    pub neighborhood_name: Option<String>,
    pub display_title_address_line: Option<String>,
    pub timezone_field: Option<String>,
    pub hours_of_operation: Option<()>,
    pub media_photo_metadata: Option<Vec<photo::PhotoWrapper>>,
    pub formatted_address: Option<String>,
    pub claim_this_business_metadata: Option<()>,
    pub gmb_account_ownership_metadata_block: Option<()>,
    pub business_photo_gallery: Option<Vec<photo::PhotoWrapper>>,
    pub business_cta_links: Option<FulfillmentIntegration>,
    pub business_categories: Option<BusinessCategoryVector>,
    pub google_place_id: Option<String>,
    pub address_block_array: Option<()>,
    pub business_display_block: Option<()>,
    pub google_knowledge_graph_id: Option<String>,
    pub local_business_ontology_categories: Option<feature_set::FeatureSet>,
    pub another_language_code: Option<()>,
    pub service_taxonomy: Option<service_cluster::ServiceCluster>,
    pub business_level_attributes: Option<()>,
    pub google_account_avatars: Option<String>,
    pub business_category_label_tuple: Option<()>,
    pub locality_string_city_level_location_names: Option<String>,
    pub indirect_business_profile_links: Option<String>,
    pub phone_number_entity_block: Option<phone::PhoneEntry>,
    pub location_normalization_payload: Option<()>,
    pub identity_label_metadata: Option<()>,
    pub structured_business_hours_payload: Option<business_hours::BusinessHoursBlock>,
    pub coordinate_blocks: Option<GeoPoint>,
    pub b64_structured_concept_identifiers: Option<()>,
    pub business_claim_urls: Option<()>,
    pub google_maps_internal_business_entity_identifiers: Option<business_entity_identifiers::BusinessEntityIdentifier>,
    pub geographic_contextualization_metadata: Option<contextualization_metadata::GeoContainmentHierarchy>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewSummaryBlock {
    /// The full Google Reviews panel URL for this place.
    pub reviews_url: Option<String>,

    /// Raw string representing review count, e.g., `"469 reviews"`.
    /// Useful for sanity checks or fallback parsing.
    pub reviews_text: Option<String>,

    /// Optional opaque string (likely tracking token or click context).
    pub tracking_token: Option<String>,

    /// Average star rating, e.g., `4.9`.
    pub average_rating: Option<f64>,

    /// Total number of reviews, parsed as an integer, e.g., `469`.
    pub review_count: Option<u32>,
}

impl ReviewSummaryBlock {
    pub fn from_raw(value: &serde_json::Value) -> Option<Self> {
        let arr = value.as_array()?;

        // Defensive bounds
        let rating = arr.get(7).and_then(|v| v.as_f64());
        let count = arr.get(8).and_then(|v| v.as_u64()).map(|v| v as u32);

        let (url, text, token) = match arr.get(3)?.as_array() {
            Some(inner) => {
                let url = inner.get(0).and_then(|v| v.as_str()).map(|s| s.to_string());
                let text = inner.get(1).and_then(|v| v.as_str()).map(|s| s.to_string());
                let token = inner.get(3).and_then(|v| v.as_str()).map(|s| s.to_string());
                (url, text, token)
            }
            None => (None, None, None),
        };

        Some(Self {
            reviews_url: url,
            reviews_text: text,
            tracking_token: token,
            average_rating: rating,
            review_count: count,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessWebsiteLinks {
    pub url: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPoint {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let value = value.as_array()?;
        assert!(value.get(0).unwrap().is_null());
        assert!(value.get(1).unwrap().is_null());
        let latitude = value.get(2).unwrap().as_f64().unwrap();
        let longitude = value.get(3).unwrap().as_f64().unwrap();
        Some(Self { latitude, longitude })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessOwnerMetadata {
    pub display_name: String,
    pub google_identity_id: Option<String>,
    pub canonical_owner_id: String,
}

impl BusinessOwnerMetadata {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let value = value.as_array().unwrap();
        assert!(value.get(0).unwrap().is_null());

        // display name (organization or user) + suffix
        let display_name = value
            .get(1)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        
        // Google identity ID (GMB or personal account)
        let google_identity_id = unpack_str_or_null(value.get(2).unwrap())
            .map(|x| x.to_owned());
        
        assert!(value.get(3).unwrap().is_null());
        assert!(value.get(4).unwrap().is_null());
        assert!(value.get(5).unwrap().is_null());
        assert!(value.get(6).unwrap().is_null());
        assert!(value.get(7).unwrap().is_null());
        
        // ❗ Field [2] (Google ID) and [8] (Backup ID) Are Not Always Equal
        let canonical_owner_id = value
            .get(8)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        
        Some(Self {
            display_name,
            google_identity_id,
            canonical_owner_id,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessCategoryVector(pub Vec<BusinessCategory>);

impl BusinessCategoryVector {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let entries = unpack_array_or_null(value).map(|items| {
            items
                .into_iter()
                .map(|x| BusinessCategory::from_raw(x).unwrap())
                .collect::<Vec<_>>()
        })?;
        Some(Self(entries))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BusinessCategory {
    /// Index 0 — e.g., "software_company"
    pub label: String,
    /// Index 1 — Optional human-readable or localized label
    pub display_name: Option<String>,
    /// Index 2 — typically 0–7, possibly a confidence tier
    pub score: i64,
}

impl BusinessCategory {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let value = value.as_array()?;
        let label = value.get(0)?.as_str()?.to_string();
        let display_name = value.get(1).and_then(|v| v.as_str().map(|s| s.to_string()));
        let score = value.get(2)?.as_i64()?;
        Some(Self {
            label,
            display_name,
            score,
        })
    }
}


/// - If `[0]` is non-null, it's often a neighborhood, development, or subdivision name (like a small area or HOA label).
/// - The redundancy between `[1]` and `[2]` suggests one is used for rendering, the other for matching or indexing.
#[derive(Debug, Clone, Serialize)]
pub struct BusinessAddressFields {
    /// e.g., neighborhood name like "Harvest Hills"
    pub location_hint: Option<String>,
    /// formatted street address
    pub street_address_1: Option<String>,
    /// identical to street_address_1 — redundancy
    pub street_address_2: Option<String>,
    /// locality (e.g. "Lehi", "Draper")
    pub city: Option<String>
}

impl BusinessAddressFields {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let location_hint = unpack_str_or_null(value.get(0).unwrap()).map(ToString::to_string);
        let street_address_1 = unpack_str_or_null(value.get(1).unwrap()).map(ToString::to_string);
        let street_address_2 = unpack_str_or_null(value.get(2).unwrap()).map(ToString::to_string);
        let city = unpack_str_or_null(value.get(3).unwrap()).map(ToString::to_string);
        let all_none = &[&location_hint, &street_address_1, &street_address_2, &city].into_iter().all(|x| x.is_none());
        if *all_none {
            return None
        }
        Some(Self {
            location_hint,
            street_address_1,
            street_address_2,
            city,
        })
    }
}


/// business display label vector — UI-facing business name metadata
#[derive(Debug, Clone, Serialize)]
pub struct BusinessDisplayBlock {
    // pub type_string: Option<String>,         // e.g. "SearchResult.TYPE_SHOPPING"
    pub name: String,                        // business display name
    // layout_metrics: Option<LayoutBlob>,  // array of layout-related numeric values
}

impl BusinessDisplayBlock {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let value = value.as_array().unwrap();
        let name = value.get(3).unwrap().as_str().unwrap().to_string();
        Some(Self { name })
    }
}






/// Represents a complete service taxonomy block extracted from a business entity.
#[derive(Debug, Clone, Serialize)]
pub struct ServiceTaxonomy {
    pub entries: Vec<ServiceTaxonomyCategory>,
}

impl ServiceTaxonomy {
    pub fn from_raw(value: &Value) -> Option<Self> {
        let value = value.as_array().unwrap();
        let value = match &value.as_slice() {
            &[x] => x,
            _ => panic!("WHEN DOES THIS HAPPEN?")
        };
        unimplemented!("TODO")
    }
}

/// A service category (e.g., "Software Company") and its associated offerings.
#[derive(Debug, Clone, Serialize)]
pub struct ServiceTaxonomyCategory {
    pub label: String,
    pub offerings: Vec<ServiceTaxonomyOffering>,
}

/// A specific service offered, potentially with a rich description.
#[derive(Debug, Clone, Serialize)]
pub struct ServiceTaxonomyOffering {
    pub name: String,
    pub description: Option<String>,
}

pub mod photo {
    use super::*;

    /// Represents a single photo or image-related entity associated with a business.
    #[derive(Debug, Clone, Serialize)]
    pub struct PhotoEntry {
        /// Opaque Google image ID, used to construct image URLs.
        pub photo_id: String,

        /// The general media type (1 = photo, 2 = street view, etc.)
        pub media_type: u8,

        /// Optional subtype, often used for classification (e.g., 3 = default photo?)
        pub media_subtype: Option<u8>,

        /// Canonical image URL (can be used to download a thumbnail or full-size image)
        pub image_url: Option<String>,

        /// Human-readable alt text or UI string like "9 Photos"
        pub label: Option<String>,

        /// Raw/original image resolution
        pub original_size: Option<(u32, u32)>,

        /// Optional thumbnail display size
        pub thumbnail_size: Option<(u32, u32)>,

        /// Coordinates where the image was taken (if available)
        pub location: Option<GeoPoint>,

        /// Reference to the business or place this image is tied to
        pub business: BusinessReference,

        /// Upload timestamp of the photo, if known
        pub upload_time: Option<UploadTimestamp>,

        /// Source label like `bizbuilder:gmb_web` or `launch`
        pub source_tag: Option<String>,

        /// A reference type and ID that links this to other internal or external systems
        pub reference: Option<PhotoReference>,

        /// Cluster/group information: how many photos are in this set
        pub cluster_info: Option<PhotoClusterInfo>,

        /// The content type label (e.g., "Photo", "Street View")
        pub content_type_label: Option<String>,
    }

    /// Represents the location coordinates of a photo.
    #[derive(Debug, Clone, Serialize)]
    pub struct GeoPoint {
        pub lat: f64,
        pub lon: f64,
    }

    /// Refers to the business entity a photo is tied to.
    #[derive(Debug, Clone, Serialize)]
    pub struct BusinessReference {
        /// Internal Google entity ID (e.g., `I8OHaNHCHcjIkPIP-MOQyQU`)
        pub entity_id: String,

        /// Cluster ID used in URLs (e.g., `0x8752...`)
        pub cluster_id: Option<String>,

        /// Address string, if available
        pub address: Option<String>,
    }

    /// Represents an uploaded date and time, usually from a 4-element array.
    #[derive(Debug, Clone, Serialize)]
    pub struct UploadTimestamp {
        pub year: u16,
        pub month: u8,  // 1-based
        pub day: u8,
        pub hour: Option<u8>, // Some payloads include hour
    }

    /// Represents references like UGCS or Street View pano links.
    #[derive(Debug, Clone, Serialize)]
    pub struct PhotoReference {
        pub kind: String,      // e.g., "UGCS_REFERENCE", "GEO_PHOTO_REFERENCE"
        pub id: String,        // internal reference ID or compound token
        pub count: Option<u32> // sometimes "1", unclear meaning
    }

    /// Metadata for a photo cluster label (e.g. "3 Photos").
    #[derive(Debug, Clone, Serialize)]
    pub struct PhotoClusterInfo {
        /// Label (e.g., "3 Photos")
        pub label: String,

        /// Total count of photos in the set, if known
        pub count: Option<u32>,

        /// Index within the set, if present
        pub index: Option<u32>,
    }


    impl TryFrom<&Value> for PhotoEntry {
        type Error = &'static str;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            let arr = value.as_array().ok_or("Not an array")?;
            // eprintln!("{}", format!("{}", serde_json::to_string_pretty(arr).unwrap().bright_red()));
            let photo_id = arr.get(0).and_then(Value::as_str).ok_or("Missing photo_id")?.to_string();
            let media_type = arr.get(1).and_then(Value::as_u64).ok_or("Missing media_type")? as u8;
            let media_subtype = arr.get(2).and_then(Value::as_u64).map(|v| v as u8);

            let image_block = arr.get(6).and_then(Value::as_array);
            let image_url = image_block.and_then(|a| a.get(0)).and_then(Value::as_str).map(|s| s.to_string());
            let label = image_block.and_then(|a| a.get(1)).and_then(Value::as_str).map(|s| s.to_string());

            let original_size = image_block.and_then(|a| a.get(2))
                .and_then(|v| v.as_array())
                .and_then(|xy| Some((xy.get(0)?.as_u64()? as u32, xy.get(1)?.as_u64()? as u32)));

            let thumbnail_size = image_block.and_then(|a| a.get(3))
                .and_then(|v| v.as_array())
                .and_then(|xy| Some((xy.get(0)?.as_u64()? as u32, xy.get(1)?.as_u64()? as u32)));

            let geo_block = arr.get(8).and_then(Value::as_array);
            let location = geo_block
                .and_then(|geo| geo.get(0))
                .and_then(|v| v.as_array())
                .and_then(|coords| Some(GeoPoint {
                    lon: coords.get(1)?.as_f64()?,
                    lat: coords.get(2)?.as_f64()?,
                }));

            let entity_id = arr.get(9).and_then(Value::as_str).ok_or("Missing entity ID")?.to_string();
            let cluster_id = arr.get(15)
                .and_then(|v| v.get(0))
                .and_then(|v| v.get(0))
                .and_then(Value::as_str)
                .map(|s| s.to_string());

            let address = arr.get(17)
                .and_then(|v| v.get(0))
                .and_then(Value::as_str)
                .map(|s| s.to_string());

            let content_type_label = arr.get(20).and_then(Value::as_str).map(|s| s.to_string());

            // Upload time and source
            let metadata = arr.get(21).and_then(Value::as_array);
            let upload_time = metadata
                .and_then(|m| m.get(6))
                .and_then(|v| v.get(8))
                .and_then(|ts| ts.as_array())
                .and_then(|ts| Some(UploadTimestamp {
                    year: ts.get(0)?.as_u64()? as u16,
                    month: ts.get(1)?.as_u64()? as u8,
                    day: ts.get(2)?.as_u64()? as u8,
                    hour: ts.get(3).and_then(Value::as_u64).map(|h| h as u8),
                }));

            let source_tag = metadata
                .and_then(|m| m.get(6))
                .and_then(|v| v.get(5))
                .and_then(|v| v.get(2))
                .and_then(Value::as_str)
                .map(|s| s.to_string());

            let reference = metadata
                .and_then(|m| m.get(12))
                .and_then(|v| v.as_array())
                .and_then(|ref_arr| Some(PhotoReference {
                    kind: ref_arr.get(0)?.as_str()?.to_string(),
                    id: ref_arr.get(1)?.as_str()?.to_string(),
                    count: ref_arr.get(2).and_then(Value::as_str).and_then(|s| s.parse().ok()),
                }));

            // Cluster info from the tail payload (optional)
            let cluster_info = value.as_array()
                .and_then(|v| v.last())
                .and_then(Value::as_array)
                .and_then(|tail| {
                    let label = label.clone()?; // reuse the "3 Photos" string
                    let count = tail.get(0)
                        .and_then(|v| v.get(0))
                        .and_then(|v| v.get(1))
                        .and_then(Value::as_u64)
                        .map(|v| v as u32);
                    let index = tail.get(4).and_then(Value::as_u64).map(|v| v as u32);
                    Some(PhotoClusterInfo { label, count, index })
                });

            Ok(PhotoEntry {
                photo_id,
                media_type,
                media_subtype,
                image_url,
                label,
                original_size,
                thumbnail_size,
                location,
                business: BusinessReference {
                    entity_id,
                    cluster_id,
                    address,
                },
                upload_time,
                source_tag,
                reference,
                cluster_info,
                content_type_label,
            })
        }
    }

    /// Wraps a single photo entry with metadata about its cluster context
    #[derive(Debug, Clone, Serialize)]
    pub struct PhotoWrapper {
        /// The core photo content
        pub entry: PhotoEntry,

        /// Total photos (and type breakdown) in the associated photo cluster
        pub cluster_meta: Option<PhotoClusterMeta>,

        /// Entity ID (duplicate of entry.business.entity_id, but useful for fallback or joins)
        pub entity_id: Option<String>,

        /// Opaque analytics or tracking token (rarely used directly)
        pub tracking_token: Option<String>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct PhotoClusterMeta {
        /// List of (media_type, count) pairs — usually [(1, 9)] meaning 9 photos
        pub media_type_counts: Vec<(u32, u32)>,

        /// The index of this photo within the cluster, if known (e.g., 0)
        pub selected_index: Option<u32>,

        /// Duplicate of total_count (redundant but can validate against sum)
        pub total_count: Option<u32>,

        /// Active/highlighted photo index in UI
        pub active_index: Option<u32>,
    }

    impl TryFrom<&Value> for PhotoWrapper {
        type Error = &'static str;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            let wrapper_arr = value.as_array().ok_or("Expected array")?;

            // First field must be the photo entry array
            let photo_entry_val = wrapper_arr.get(0).ok_or("Missing photo entry at index 0")?;
            let entry = PhotoEntry::try_from(photo_entry_val)?;

            // Tracking fields
            let entity_id = wrapper_arr.get(3).and_then(Value::as_str).map(|s| s.to_string());
            let tracking_token = wrapper_arr.get(5).and_then(Value::as_str).map(|s| s.to_string());

            // PhotoClusterMeta from wrapper_arr[8]
            let cluster_meta = wrapper_arr.get(8).and_then(|meta_val| {
                let arr = meta_val.as_array()?;

                let media_type_counts = arr.get(0)?
                    .as_array()?
                    .iter()
                    .filter_map(|pair| {
                        let pair_arr = pair.as_array()?;
                        let t = pair_arr.get(0)?.as_u64()? as u32;
                        let c = pair_arr.get(1)?.as_u64()? as u32;
                        Some((t, c))
                    })
                    .collect::<Vec<_>>();

                Some(PhotoClusterMeta {
                    media_type_counts,
                    selected_index: arr.get(1).and_then(Value::as_u64).map(|v| v as u32),
                    total_count: arr.get(3).and_then(Value::as_u64).map(|v| v as u32),
                    active_index: arr.get(4).and_then(Value::as_u64).map(|v| v as u32),
                })
            });

            Ok(PhotoWrapper {
                entry,
                cluster_meta,
                entity_id,
                tracking_token,
            })
        }
    }

}

#[derive(Debug, Clone, Serialize)]
pub struct FulfillmentIntegration {
    pub vendor_domain: String,          // e.g. "adobe-lehi-service-center.square.site"
    pub vendor_logo_url: Option<String>,// logo/icon (e.g., Square)
    pub vendor_label: Option<String>,   // e.g., "Square"
    pub vendor_logo_size: Option<(u32, u32)>,
    pub destination_url: String,        // full destination link
    pub tracking_info: Option<String>,  // trailing analytics blob
    pub tag: Option<u64>, // The trailing field (e.g. 21634)
}

impl FulfillmentIntegration {
    pub fn try_from_google_managed(arr: &[Value]) -> Result<Self, &'static str> {
        let payload = arr.get(5)
            .and_then(Value::as_array)
            .ok_or("Missing Google fulfillment payload")?;

        let vendor_label = payload.get(0)
            .and_then(Value::as_str)
            .map(str::to_string);

        // The destination URL is actually inside: payload[1][2][0]
        let destination_url = payload.get(1)
            .and_then(|v| v.get(2))
            .and_then(|v| v.get(0))
            .and_then(Value::as_str)
            .ok_or("Missing Google destination URL")?
            .to_string();

        let tracking_token = payload.get(12)
            .and_then(Value::as_str)
            .map(str::to_string);

        let tag = arr.get(4).and_then(Value::as_u64);

        Ok(FulfillmentIntegration {
            vendor_domain: "google.com".to_string(),
            vendor_label,
            vendor_logo_url: None,
            vendor_logo_size: None,
            destination_url,
            tracking_info: tracking_token,
            tag,
        })
    }
}


impl FulfillmentIntegration {
    fn try_from_vendor_block(arr: &[Value]) -> Result<Self, &'static str> {
        // let arr = value.as_array().ok_or("Expected outer array")?;
        if arr.len() < 5 {
            return Err("Array too short");
        }
        
        let vendor_block = arr.get(2)
            .and_then(|v| v.get(0))
            .and_then(|v| v.get(0))
            .ok_or("Missing vendor block")?;
        
        let vendor_arr = vendor_block.as_array().ok_or("Vendor block not an array")?;
        
        let vendor_domain = vendor_arr.get(0)
            .and_then(Value::as_str)
            .ok_or("Missing vendor domain")?
            .to_string();
        
        let logo_block = vendor_arr.get(2).and_then(Value::as_array);
        let vendor_logo_url = logo_block
            .and_then(|v| v.get(0))
            .and_then(Value::as_str)
            .map(str::to_string);
        let vendor_label = logo_block
            .and_then(|v| v.get(1))
            .and_then(Value::as_str)
            .map(str::to_string);
        let vendor_logo_size = logo_block
            .and_then(|v| v.get(2))
            .and_then(|v| v.as_array())
            .and_then(|xy| Some((xy.get(0)?.as_u64()? as u32, xy.get(1)?.as_u64()? as u32)));
        
        let fulfillment_block = arr.get(2)
            .and_then(|v| v.get(0))
            .and_then(|v| v.get(1))
            .and_then(|v| v.get(2))
            .and_then(|v| v.as_array())
            .ok_or("Missing fulfillment data")?;
        
        let destination_url = fulfillment_block.get(0)
            .and_then(Value::as_str)
            .ok_or("Missing destination URL")?
            .to_string();
        
        let tracking_token = fulfillment_block.get(1)
            .and_then(|v| v.get(4))
            .and_then(Value::as_str)
            .map(str::to_string);
        
        let tag = arr.get(4).and_then(Value::as_u64);
        
        Ok(FulfillmentIntegration {
            vendor_domain,
            vendor_label,
            vendor_logo_url,
            vendor_logo_size,
            destination_url,
            tracking_info: tracking_token,
            tag,
        })
    }
}


impl TryFrom<&Value> for FulfillmentIntegration {
    type Error = &'static str;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let arr = value.as_array().ok_or("Expected array")?;

        match arr.get(2) {
            Some(Value::Array(_)) => Self::try_from_vendor_block(arr),
            _ => Self::try_from_google_managed(arr),
        }
    }
}



pub mod feature_set {
    use super::*;
    use serde::Serialize;
    use serde_json::Value;

    /// Root structure of a Google Maps feature set payload.
    ///
    /// This matches arrays like:
    /// `[ [uncategorized], [grouped], null, [uncategorized2], null, [grouped2] ]`
    #[derive(Debug, Clone, Serialize)]
    pub struct FeatureSet {
        /// Top-level features with no category
        pub primary: Vec<Feature>,
        /// Grouped features by UI category
        pub grouped: Vec<FeatureCategory>,
        /// Sometimes there are tail features again
        pub extra: Option<Vec<Feature>>,
        /// Additional grouped section (e.g. amenities v2)
        pub grouped_tail: Option<Vec<FeatureCategory>>,
    }

    /// A user-facing category grouping a set of features.
    #[derive(Debug, Clone, Serialize)]
    pub struct FeatureCategory {
        /// Index 0 — internal category slug (e.g. `"accessibility"`)
        pub slug: String,
        /// Index 1 — display label (e.g. `"Accessibility"`)
        pub display_name: String,
        /// Index 2 — list of feature items
        pub features: Vec<Feature>,
    }

    /// A single UI-labeled feature with structured metadata.
    #[derive(Debug, Clone, Serialize)]
    pub struct Feature {
        /// Index 0 — feature ID (usually a structured type path)
        pub id: String,
        /// Index 1 — user-visible label
        pub label: String,
        /// Index 2 — confidence or UI block, if present
        pub confidence: Option<FeatureConfidence>,
        /// Index 3 — unknown (often null, sometimes 1)
        pub metadata_flag: Option<i64>,
        /// Index 4 — array of weights or ranking factors
        pub weights: Option<Vec<i64>>,
        /// Index 5 — usually 0, sometimes 1 if negated
        pub present_flag: Option<i64>,
    }

    /// Tagging/confidence structure used within a feature.
    #[derive(Debug, Clone, Serialize)]
    pub struct FeatureConfidence {
        /// Confidence score (often `1`)
        pub score: i64,
        /// Labeled confidence tags (UI display terms)
        pub tags: Vec<ConfidenceTag>,
    }

    /// A single display tag for a confidence block.
    #[derive(Debug, Clone, Serialize)]
    pub struct ConfidenceTag {
        pub score: i64,
        pub label: String,
    }

    /// A simplified intermediate format used for raw category extraction (e.g. index 1 block).
    #[derive(Debug, Clone, Serialize)]
    pub struct FeatureSetRaw {
        /// Outer index 1 — list of categories (e.g. "accessibility", "environment", etc.)
        pub categories: Vec<FeatureCategory>,
    }

    impl FeatureSetRaw {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let categories = value
                .get(1)?
                .as_array()?
                .iter()
                .filter_map(|cat| {
                    let arr = cat.as_array()?;
                    Some(FeatureCategory {
                        slug: arr.get(0)?.as_str()?.to_string(),
                        display_name: arr.get(1)?.as_str()?.to_string(),
                        features: arr.get(2)?
                            .as_array()?
                            .iter()
                            .filter_map(parse_feature)
                            .collect(),
                    })
                })
                .collect();

            Some(Self { categories })
        }
    }

    /// Helper function to parse a feature item from a JSON array.
    fn parse_feature(value: &Value) -> Option<Feature> {
        let arr = value.as_array()?;
        Some(Feature {
            id: arr.get(0)?.as_str()?.to_string(),
            label: arr.get(1)?.as_str()?.to_string(),
            confidence: arr.get(2).and_then(parse_confidence),
            metadata_flag: arr.get(3).and_then(Value::as_i64),
            weights: arr.get(4)?.as_array()?.iter().filter_map(Value::as_i64).collect::<Vec<_>>().into(),
            present_flag: arr.get(5).and_then(Value::as_i64),
        })
    }

    /// Parses the confidence/tags block.
    fn parse_confidence(value: &Value) -> Option<FeatureConfidence> {
        let arr = value.as_array()?;
        let score = arr.get(0)?.as_i64()?;

        let tags = arr.get(1)?
            .as_array()?
            .iter()
            .filter_map(|tag| {
                let inner = tag.as_array()?;
                Some(ConfidenceTag {
                    score: inner.get(0)?.as_i64()?,
                    label: inner.get(1)?.as_str()?.to_string(),
                })
            })
            .collect();

        Some(FeatureConfidence { score, tags })
    }

    impl FeatureSet {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let arr = value.as_array()?;
            let primary = arr.get(0).and_then(Self::parse_feature_list);
            let grouped = arr.get(1).and_then(Self::parse_feature_categories);
            let extra = arr.get(2).and_then(Self::parse_feature_list);
            let grouped_tail = arr.get(3).and_then(Self::parse_feature_categories);

            Some(Self {
                primary: primary.unwrap_or_default(),
                grouped: grouped.unwrap_or_default(),
                extra,
                grouped_tail,
            })
        }

        fn parse_feature_list(value: &Value) -> Option<Vec<Feature>> {
            let items = value.as_array()?;
            Some(items.iter().filter_map(Feature::from_raw).collect())
        }

        fn parse_feature_categories(value: &Value) -> Option<Vec<FeatureCategory>> {
            let items = value.as_array()?;
            Some(items.iter().filter_map(FeatureCategory::from_raw).collect())
        }
    }

    impl FeatureCategory {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let arr = value.as_array()?;
            let slug = arr.get(0)?.as_str()?.to_string();
            let display_name = arr.get(1)?.as_str()?.to_string();
            let features_arr = arr.get(2)?.as_array()?;
            let features = features_arr.iter().filter_map(Feature::from_raw).collect();

            Some(Self {
                slug,
                display_name,
                features,
            })
        }
    }

    impl Feature {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let arr = value.as_array()?;
            Some(Self {
                id: arr.get(0)?.as_str()?.to_string(),
                label: arr.get(1)?.as_str()?.to_string(),
                confidence: arr.get(2).and_then(FeatureConfidence::from_raw),
                metadata_flag: arr.get(3)?.as_i64(),
                weights: arr.get(4)?.as_array().map(|v| v.iter().filter_map(|x| x.as_i64()).collect()),
                present_flag: arr.get(5)?.as_i64(),
            })
        }
    }

    impl FeatureConfidence {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let arr = value.as_array()?;
            let score = arr.get(0)?.as_i64()?;
            let tags_raw = arr.get(1)?.as_array()?;
            let tags = tags_raw.iter().filter_map(ConfidenceTag::from_raw).collect();

            Some(Self { score, tags })
        }
    }

    impl ConfidenceTag {
        pub fn from_raw(value: &Value) -> Option<Self> {
            let arr = value.as_array()?;
            Some(Self {
                score: arr.get(0)?.as_i64()?,
                label: arr.get(1)?.as_str()?.to_string(),
            })
        }
    }

}

pub mod service_cluster {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::convert::TryFrom;

    #[derive(Debug, Clone, Serialize)]
    pub struct ServiceCluster {
        pub groups: Vec<ServiceGroup>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct ServiceGroup {
        pub category: String,
        pub topics: Vec<ServiceTopic>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct ServiceTopic {
        pub title: String,
        pub description: Option<String>,
    }

    impl TryFrom<&Value> for ServiceCluster {
        type Error = &'static str;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            // Navigate to root: /0/0/1
            let root = value
                .get(0).and_then(|v| v.get(0)).and_then(|v| v.get(1))
                .ok_or("Missing root structure at [0][0][1]")?;

            let raw_groups = root.as_array().ok_or("Expected array at root")?;
            let mut groups = vec![];

            for raw_group in raw_groups {
                let group_array = raw_group.as_array().ok_or("Group not an array")?;

                let category = group_array
                    .get(0).and_then(|v| v.as_array())
                    .and_then(|arr| arr.get(0))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing group category name")?
                    .to_string();

                let raw_topics = group_array
                    .get(1).and_then(|v| v.as_array())
                    .ok_or("Missing group topics")?;

                let mut topics = vec![];

                for topic_entry in raw_topics {
                    let topic_array = topic_entry
                        .as_array()
                        .and_then(|v| v.get(0))
                        .and_then(|v| v.as_array())
                        .and_then(|v| v.get(0))
                        .and_then(|v| v.as_array())
                        .ok_or("Topic structure invalid")?;

                    let title = topic_array
                        .get(0).and_then(|v| v.as_str())
                        .ok_or("Missing topic title")?
                        .to_string();

                    let description = topic_array
                        .get(1).and_then(|v| v.as_str())
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.to_string());

                    topics.push(ServiceTopic { title, description });
                }

                groups.push(ServiceGroup { category, topics });
            }

            Ok(ServiceCluster { groups })
        }
    }

}

pub mod phone {
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    pub struct PhoneEntry {
        /// Displayed number (e.g. (801) 835-1868)
        pub display: Option<String>,
        /// Variants of the number with rank (e.g. +1 version, unformatted)
        pub variants: Vec<PhoneVariant>,
        /// Canonical number in E.164 format
        pub canonical: Option<String>,
        /// Optional tel: URI (with clicktracking string)
        pub tel_uri: Option<TelUriBlock>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct PhoneVariant {
        /// The phone number variant (e.g. "+1 801-835-1868")
        pub number: String,
        /// Ranking or type index (usually 1 or 2)
        pub rank: i64,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TelUriBlock {
        /// URI string, e.g., "tel:+18018351868"
        pub uri: String,
        /// Optional click-tracking token
        pub click_token: Option<String>,
    }

    impl TryFrom<&Value> for PhoneEntry {
        type Error = &'static str;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            let arr = value.as_array().ok_or("PhoneEntry: not an array")?;

            let display = arr.get(0)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());


            let variants = match arr.get(1) {
                Some(Value::Array(v)) => v.iter()
                    .map(PhoneVariant::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
                Some(Value::Null) | None => Vec::new(),
                _ => return Err("PhoneEntry: invalid type for variants"),
            };

            let canonical = arr.get(3)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let tel_uri = match arr.get(5) {
                Some(Value::Array(inner)) if !inner.is_empty() => Some(TelUriBlock::try_from(inner)?),
                _ => None,
            };

            Ok(Self {
                display,
                variants,
                canonical,
                tel_uri,
            })
        }
    }

    impl TryFrom<&Value> for PhoneVariant {
        type Error = &'static str;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            let arr = value.as_array().ok_or("PhoneVariant: not an array")?;

            let number = arr.get(0)
                .and_then(|v| v.as_str())
                .ok_or("PhoneVariant: missing number")?
                .to_string();

            let rank = arr.get(1)
                .and_then(|v| v.as_i64())
                .ok_or("PhoneVariant: missing rank")?;

            Ok(Self { number, rank })
        }
    }

    impl TryFrom<&Vec<Value>> for TelUriBlock {
        type Error = &'static str;

        fn try_from(arr: &Vec<Value>) -> Result<Self, Self::Error> {
            let uri = arr.get(0)
                .and_then(|v| v.as_str())
                .ok_or("TelUriBlock: missing URI")?
                .to_string();

            // Click token (4th item — index 3), may or may not exist
            let click_token = arr.get(3)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            Ok(Self { uri, click_token })
        }
    }
}

pub mod business_hours {
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    pub struct BusinessHoursBlock {
        pub weekly_schedule: Option<Vec<DailyScheduleEntry>>,  // full or null
        pub today_status: Option<TodayMeta>,                   // full, partial, or null
        pub schedule_type: Option<u8>,
        pub unused_1: Option<()>,
        pub unused_2: Option<()>,
        pub timezone_or_region_code: Option<u8>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct DailyScheduleEntry {
        pub day_name: String,
        pub day_index: u8,
        pub date: [u16; 3],
        pub time_ranges: Vec<DayTimeRange>,
        pub flag: u8,
        pub status_code: u8,
    }

    #[derive(Debug, Clone, Serialize)]
    pub enum DayTimeRange {
        Closed,
        OpenRange {
            label: String,
            bounds: [Vec<u8>; 2],
        }
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TodayMeta {
        pub entry: Option<DailyScheduleEntry>,         // null in abbreviated mode
        pub meta_flag_1: Option<u8>,                   // often 0 or null
        pub open_status_code: Option<u8>,              // 1 = open, 2 = closed
        pub unused: Option<()>,
        pub status_text: Option<DisplayStatus>,
        pub status_text_repeat: Option<DisplayStatus>, // often duplicate
        pub unused_2: Option<()>,
        pub unused_3: Option<()>,
        pub simplified_status: Option<DisplayStatus>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct DisplayStatus {
        pub text: String,
        pub highlights: Vec<StatusHighlight>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct StatusHighlight {
        pub start_char: u32,
        pub end_char: u32,
        pub metadata: Option<HighlightMetadata>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct HighlightMetadata {
        pub placeholder: Option<()>,       // always null
        pub tokens: [u64; 2],              // stable ID pair
    }
    
    use serde_json::Value;
    use anyhow::{Result, anyhow};

    impl BusinessHoursBlock {
        pub fn from_value(value: &Value) -> Result<Self> {
            let arr = value.as_array().ok_or_else(|| anyhow!("Expected top-level array"))?;

            let weekly_schedule = arr.get(0)
                .and_then(|v| v.as_array())
                .map(|entries| {
                    entries.iter()
                        .filter_map(parse_daily_schedule_entry)
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?;

            let today_status = arr.get(1)
                .map(parse_today_meta)
                .transpose()?;

            Ok(BusinessHoursBlock {
                weekly_schedule,
                today_status,
                schedule_type: arr.get(2).and_then(|v| v.as_u64()).map(|v| v as u8),
                unused_1: None,
                unused_2: None,
                timezone_or_region_code: arr.get(5).and_then(|v| v.as_u64()).map(|v| v as u8),
            })
        }
    }

    fn parse_daily_schedule_entry(val: &Value) -> Option<Result<DailyScheduleEntry>> {
        let arr = val.as_array()?;

        let day_name = arr.get(0).and_then(|v| v.as_str())?.to_string();
        let day_index = arr.get(1).and_then(|v| v.as_u64())? as u8;
        let date = arr.get(2)
            .and_then(|v| v.as_array())
            .and_then(|a| Some([
                a.get(0)?.as_u64()? as u16,
                a.get(1)?.as_u64()? as u16,
                a.get(2)?.as_u64()? as u16,
            ]))?;

        let time_ranges = arr.get(3)
            .and_then(|v| v.as_array())?
            .iter()
            .map(|entry| {
                let label = entry.get(0).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                if label == "Closed" {
                    Ok(DayTimeRange::Closed)
                } else if let Some(bounds_value) = entry.get(1) {
                    let bounds: [Vec<u8>; 2] = serde_json::from_value(bounds_value.clone())?;
                    Ok(DayTimeRange::OpenRange { label, bounds })
                } else {
                    // If bounds are missing, assume it's an OpenRange with unknown bounds (e.g. UI-only label)
                    Ok(DayTimeRange::OpenRange { label, bounds: [vec![], vec![]] })
                }
            })
            .collect::<Result<Vec<_>>>();

        match time_ranges {
            Ok(ranges) => Some(Ok(DailyScheduleEntry {
                day_name,
                day_index,
                date,
                time_ranges: ranges,
                flag: arr.get(4).and_then(|v| v.as_u64()).unwrap_or(0) as u8,
                status_code: arr.get(5).and_then(|v| v.as_u64()).unwrap_or(0) as u8,
            })),
            Err(err) => Some(Err(err)),
        }
    }

    fn parse_today_meta(val: &Value) -> Result<TodayMeta> {
        let arr = val.as_array().ok_or_else(|| anyhow!("Expected today meta array"))?;

        let entry = match arr.get(0).and_then(parse_daily_schedule_entry) {
            Some(Ok(e)) => Some(e),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        Ok(TodayMeta {
            entry,
            meta_flag_1: arr.get(1).and_then(|v| v.as_u64()).map(|v| v as u8),
            open_status_code: arr.get(2).and_then(|v| v.as_u64()).map(|v| v as u8),
            unused: None,
            status_text: arr.get(4).map(parse_display_status).transpose()?,
            status_text_repeat: arr.get(5).map(parse_display_status).transpose()?,
            unused_2: None,
            unused_3: None,
            simplified_status: arr.get(8).map(parse_display_status).transpose()?,
        })
    }

    fn parse_display_status(val: &Value) -> Result<DisplayStatus> {
        let arr = val.as_array().ok_or_else(|| anyhow!("Expected display status array"))?;
        let text = arr.get(0).and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let highlights = arr.get(1)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(parse_status_highlight)
            .collect::<Result<Vec<_>>>()?;

        Ok(DisplayStatus { text, highlights })
    }

    fn parse_status_highlight(val: &Value) -> Result<StatusHighlight> {
        let arr = val.as_array().ok_or_else(|| anyhow!("Expected highlight array"))?;

        let start_char = arr.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let end_char = arr.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let metadata = arr.get(2).map(parse_highlight_metadata).transpose()?;

        Ok(StatusHighlight {
            start_char,
            end_char,
            metadata,
        })
    }

    fn parse_highlight_metadata(val: &Value) -> Result<HighlightMetadata> {
        let arr = val.as_array().ok_or_else(|| anyhow!("Expected highlight metadata array"))?;
        let placeholder = Some(());
        let tokens = arr.get(1)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .ok_or_else(|| anyhow!("Missing tokens"))?;

        Ok(HighlightMetadata {
            placeholder,
            tokens,
        })
    }
}

pub mod business_entity_identifiers {
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    pub struct BusinessEntityIdentifier {
        /// Internal composite ID, often in the format "0x...:0x..."
        pub composite_id: String,

        /// Optional Knowledge Graph identifier (e.g. "/g/...")
        pub kg_id: Option<String>,

        /// Optional Google Place ID
        pub place_id: Option<String>,

        /// Optional internal numeric ID (often a canonical business entity ID)
        pub numeric_id: Option<String>,

        /// Optional uploader / account / profile ID (likely a Google+ legacy or owner ID)
        pub owner_id: Option<String>,
    }

    impl BusinessEntityIdentifier {
        pub fn from_value(val: &serde_json::Value) -> anyhow::Result<Self> {
            let arr = val.as_array().ok_or_else(|| anyhow::anyhow!("Expected array"))?;
            let inner = arr.get(0).and_then(|v| v.as_array()).ok_or_else(|| anyhow::anyhow!("Expected inner array"))?;
    
            Ok(Self {
                composite_id: inner.get(0).and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                kg_id: inner.get(3).and_then(|v| v.as_str()).map(str::to_string),
                place_id: inner.get(4).and_then(|v| v.as_str()).map(str::to_string),
                numeric_id: inner.get(5).and_then(|v| v.as_str()).map(str::to_string),
                owner_id: inner.get(6).and_then(|v| v.as_str()).map(str::to_string),
            })
        }
    }
}

pub mod contextualization_metadata {
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    pub struct RegionClassification {
        pub region_id: (String, String),
        pub confidence: f32,
        pub names: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct GeoContainmentHierarchy {
        pub regions: Vec<RegionClassification>,
    }

    pub fn parse_geo_containment_hierarchy(value: &Value) -> anyhow::Result<GeoContainmentHierarchy> {
        let outer = value
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Expected top-level array"))?;

        let inner = outer
            .get(0)
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow::anyhow!("Expected inner array"))?;

        let mut regions = Vec::new();

        for item in inner {
            let arr = item
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Expected region array"))?;

            let id_pair = arr
                .get(0)
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow::anyhow!("Missing region ID pair"))?;

            let id1 = id_pair
                .get(0)
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing id1"))?
                .to_string();

            let id2 = id_pair
                .get(1)
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing id2"))?
                .to_string();

            let confidence = arr
                .get(1)
                .and_then(|v| v.as_f64())
                .ok_or_else(|| anyhow::anyhow!("Missing confidence"))? as f32;

            let names_raw = arr
                .get(2)
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow::anyhow!("Missing names array"))?;

            let names = names_raw
                .iter()
                .filter_map(|v| v.as_array()?.get(0)?.as_str().map(str::to_string))
                .collect::<Vec<_>>();

            regions.push(RegionClassification {
                region_id: (id1, id2),
                confidence,
                names,
            });
        }

        Ok(GeoContainmentHierarchy { regions })
    }

}

// ————————————————————————————————————————————————————————————————————————————
// HELPERS
// ————————————————————————————————————————————————————————————————————————————

fn unpack_array_or_null(value: &Value) -> Option<&[Value]> {
    if value.is_null() {
        return None
    }
    Some(value.as_array().unwrap())
}

fn unpack_str_or_null(value: &Value) -> Option<&str> {
    if value.is_null() {
        return None
    }
    Some(value.as_str().unwrap())
}

fn unpack_int_or_null(value: &Value) -> Option<i64> {
    if value.is_null() {
        return None
    }
    Some(value.as_i64().unwrap())
}
