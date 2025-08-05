#![allow(unused)]
//! Parser for Google Search `tbm=map` JSON payloads
//!
//! These results are deeply nested, index-based, and undocumented.
//! This parser safely extracts typed structures from real-world SERP payloads.

use crate::map_search::data::*;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use colored::Colorize;

pub fn analyze(value: &Value) {
    let mut businesses = value
        .as_array()
        .unwrap()
        .get(64)
        .unwrap()
        .as_array()
        .unwrap();
    for (ix, business) in businesses.iter().enumerate() {
        if ix == 0 {
            analyze_ad_wrapper_business_listing_entry(&business);
            continue;
        }
        analyze_organic_local_listing(&business)
    }
}

// pub fn process_entry(value: &) {

// }

/// Ad Wrapper + Business Listing
fn analyze_ad_wrapper_business_listing_entry(value: &Value) {
    let value = value.as_array().unwrap();
}

fn analyze_organic_local_listing(value: &Value) {
    let value = value.as_array().unwrap();
    assert!(value.get(0).unwrap().is_null());
    if !value.get(1).unwrap().is_array() {
        // TODO: PROBABLY AD ENTRY/METADATA
        return
    }
    assert!(value.len() == 2);
    let value = value
        .get(1)
        .unwrap()
        .as_array()
        .unwrap();
    let business_token = value.get(0).unwrap();
    let serp_tracking_token = value.get(1).unwrap();
    let address_lines = value.get(2).unwrap();
    let reviews = value.get(4).unwrap();
    {
        let block = if reviews.is_null() {
            None
        } else {
            Some(super::deserialize::ReviewSummaryBlock::from_raw(reviews).unwrap())
        };
    }
    // log(reviews);
    assert!(value.get(5).unwrap().is_null());
    assert!(value.get(6).unwrap().is_null());
    let web = unpack_array_or_null(
            value.get(7).unwrap()
        )
        .map(|values| {
            analyze_business_website_link(values)
        });
    assert!(value.get(8).unwrap().is_null());
    let geo_location = analyze_business_geo_point(
        value.get(9)
            .unwrap()
            .as_array()
            .unwrap()
    );
    {
        let lan_lon = super::deserialize::GeoPoint::from_raw(value.get(9).unwrap()).unwrap();
    }
    let composite_business_id = value.get(10).unwrap();
    let display_name = value.get(11).unwrap();
    assert!(value.get(12).unwrap().is_null());
    let business_category_labels = unpack_array_or_null(value.get(13).unwrap()).map(|data| {
        data
            .into_iter()
            .map(|x| x.as_str().unwrap())
            .collect::<Vec<_>>()
    });
    let neighborhood_name = unpack_str_or_null(value.get(14).unwrap());
    // assert!(value.get(14).unwrap().is_null());
    assert!(value.get(15).unwrap().is_null());
    assert!(value.get(16).unwrap().is_null());
    assert!(value.get(17).unwrap().is_null());
    let display_title_address_line = unpack_str_or_null(value.get(18).unwrap());
    assert!(value.get(19).unwrap().is_null());
    assert!(value.get(20).unwrap().is_null());
    assert!(value.get(21).unwrap().is_null());
    assert!(value.get(22).unwrap().is_null());
    assert!(value.get(23).unwrap().is_null());
    // assert!(value.get(24).unwrap().is_null());
    // assert!(value.get(25).unwrap().is_null());
    let _ = unpack_array_or_null(value.get(24).unwrap()); // SKIP: graph traversal data
    let _ = value.get(25).unwrap().as_array().unwrap(); // Likely indicates an ad (sponsored placement)
    assert!(value.get(26).unwrap().is_null());
    assert!(value.get(27).unwrap().is_null());
    assert!(value.get(28).unwrap().is_null());
    assert!(value.get(29).unwrap().is_null());
    let timezone_field = value.get(30).unwrap().as_str().unwrap(); // IANA string
    assert!(value.get(31).unwrap().is_null());
    // assert!(value.get(32).unwrap().is_null());
    {
        // log(value.get(32).unwrap());
    }
    assert!(value.get(33).unwrap().is_null());
    let hours_of_operation = unpack_array_or_null(value.get(34).unwrap()); // TODO
    {
        // log(value.get(34).unwrap());
    }
    assert!(value.get(35).unwrap().is_null());
    assert!(value.get(36).unwrap().is_null());
    let media_photo_metadata = value.get(37).unwrap().as_array().unwrap(); // TODO
    {
        // TODO
    }
    assert!(value.get(38).unwrap().is_null());
    let formatted_address = unpack_str_or_null(value.get(39).unwrap());
    let _ = unpack_int_or_null(value.get(40).unwrap()); // TODO: WHAT IS THIS? MAYBE: bitflag or tag field
    assert!(value.get(41).unwrap().is_null());
    assert!(value.get(42).unwrap().is_null());
    assert!(value.get(43).unwrap().is_null());
    assert!(value.get(44).unwrap().is_null());
    assert!(value.get(45).unwrap().is_null());
    assert!(value.get(46).unwrap().is_null());
    assert!(value.get(47).unwrap().is_null());
    assert!(value.get(48).unwrap().is_null());
    let claim_this_business_metadata = unpack_array_or_null(value.get(49).unwrap());
    assert!(value.get(50).unwrap().is_null());
    assert!(value.get(51).unwrap().is_null());
    assert!(value.get(52).unwrap().is_null());
    assert!(value.get(53).unwrap().is_null());
    assert!(value.get(54).unwrap().is_null());
    assert!(value.get(55).unwrap().is_null());
    assert!(value.get(56).unwrap().is_null());
    let gmb_account_ownership_metadata_block = unpack_business_owner_metadata(value.get(57).unwrap());
    {
        // TODO
        let business_owner_metadata = super::deserialize::BusinessOwnerMetadata::from_raw(
            value.get(57).unwrap()
        );
    }
    assert!(value.get(58).unwrap().is_null());
    assert!(value.get(59).unwrap().is_null());
    assert!(value.get(60).unwrap().is_null());
    assert!(value.get(61).unwrap().as_i64().unwrap() == 1);
    assert!(value.get(62).unwrap().is_null());
    assert!(value.get(63).unwrap().is_null());
    assert!(value.get(64).unwrap().is_null());
    assert!(value.get(65).unwrap().is_null());
    assert!(value.get(66).unwrap().is_null());
    assert!(value.get(67).unwrap().as_i64().unwrap() == 1);
    assert!(value.get(68).unwrap().is_null());
    assert!(value.get(69).unwrap().is_null());
    // assert!(value.get(70).unwrap().is_null());
    // {
    //     log(value.get(70).unwrap());
    // }
    assert!(value.get(71).unwrap().is_null());
    let business_photo_gallery = unpack_array_or_null(value.get(72).unwrap()); // TODO
    {
        // TODO
    }
    assert!(value.get(73).unwrap().is_null());
    assert!(value.get(74).unwrap().is_null());
    let business_cta_links = unpack_array_or_null(value.get(75).unwrap()).map(|value| {
        unpack_business_cta_links(value)
    });
    {
        // TODO
    }
    let business_categories = unpack_business_category_vector(value.get(76).unwrap());
    {
        // TODO
        let business_category_vector = value.get(76).unwrap();
        let business_category_vector = if business_category_vector.is_null() {
            super::deserialize::BusinessCategoryVector::from_raw(
                business_category_vector
            )
        } else {
            None
        };
    }
    assert!(value.get(77).unwrap().is_null());
    let google_place_id = value.get(78).unwrap();
    assert!(value.get(79).unwrap().is_null());
    assert!(value.get(80).unwrap().is_null());
    assert!(value.get(81).unwrap().is_null());
    let address_block_array = unpack_array_or_null(value.get(82).unwrap()); // TODO
    {
        // TODO
        let business_address_fields = value.get(82).unwrap();
        let business_address_fields = if !business_address_fields.is_null() {
            Some(super::deserialize::BusinessAddressFields::from_raw(
                value.get(82).unwrap()
            ).unwrap())
        } else {
            None
        };
        // log(&business_address_fields);
    }
    assert!(value.get(83).unwrap().is_null());
    assert!(value.get(84).unwrap().is_null());
    assert!(value.get(85).unwrap().is_null());
    assert!(value.get(86).unwrap().is_null());
    assert!(value.get(87).unwrap().is_null());
    let _ = value.get(88).unwrap().as_array().unwrap(); // TODO: business display label vector; an array of UI-facing business name metadata
    {
        // TODO
        let business_display_block = super::deserialize::BusinessDisplayBlock::from_raw(
            value
                .get(88)
                .unwrap()
        ).unwrap();
        // log(&business_display_block);
    }
    let google_knowledge_graph_id = value.get(89).unwrap().as_str().unwrap();
    assert!(value.get(90).unwrap().is_null());
    assert!(value.get(91).unwrap().is_null());
    assert!(value.get(92).unwrap().is_null());
    assert!(value.get(93).unwrap().is_null());
    assert!(value.get(94).unwrap().is_null());
    assert!(value.get(95).unwrap().is_null());
    assert!(value.get(96).unwrap().is_null());
    assert!(value.get(97).unwrap().is_null());
    assert!(value.get(98).unwrap().is_null());
    assert!(value.get(99).unwrap().is_null());
    let local_business_ontology_categories = unpack_array_or_null(value.get(100).unwrap());
    {
        // TODO
        // log(&local_business_ontology_categories);
    }
    assert!(value.get(101).unwrap().is_null());
    assert!(value.get(102).unwrap().is_null());
    assert!(value.get(103).unwrap().is_null());
    assert!(value.get(104).unwrap().is_null());
    let _ = value.get(105).unwrap().as_array().unwrap(); // SKIP: MAYBE heartbeat, fetch acknowledgment, or batched no-op update signal
    assert!(value.get(106).unwrap().is_null());
    assert!(value.get(107).unwrap().is_null());
    assert!(value.get(108).unwrap().is_null());
    assert!(value.get(109).unwrap().is_null());
    let language_code = value.get(110).unwrap().as_str().unwrap();
    assert!(value.get(111).unwrap().is_null());
    let _ = unpack_str_or_null(value.get(112).unwrap()); // LANGUAGE LABEL NOT MOSTLY NULL
    // assert!(value.get(113).unwrap().is_null());
    let another_language_code = unpack_str_or_null(value.get(113).unwrap()); // LANGUAGE CODE USUALLY NULL
    assert!(value.get(114).unwrap().is_null());
    assert!(value.get(115).unwrap().is_null());
    assert!(value.get(116).unwrap().is_null());
    assert!(value.get(117).unwrap().is_null());
    // assert!(value.get(118).unwrap().is_null());
    {
        // TODO
        // sub_log(value.get(118).unwrap());
    }
    assert!(value.get(119).unwrap().is_null());
    assert!(value.get(120).unwrap().is_null());
    assert!(value.get(121).unwrap().is_null());
    assert!(value.get(122).unwrap().is_null());
    assert!(value.get(123).unwrap().is_null());
    assert!(value.get(124).unwrap().is_null());
    let service_taxonomy = unpack_array_or_null(value.get(125).unwrap());
    {
        if let Some(service_taxonomy) = service_taxonomy.as_ref() {
            // log(&service_taxonomy);
        }
    }
    assert!(value.get(126).unwrap().is_null());
    assert!(value.get(127).unwrap().is_null());
    assert!(value.get(128).unwrap().is_null());
    assert!(value.get(129).unwrap().is_null());
    // assert!(value.get(130).unwrap().is_null());
    let some_int = value.get(130).unwrap().as_i64().unwrap();
    assert!(some_int == 0);
    assert!(value.get(131).unwrap().is_null());
    assert!(value.get(132).unwrap().is_null());
    assert!(value.get(133).unwrap().is_null());
    // assert!(value.get(134).unwrap().is_null());
    {
        // TODO
        // sub_log(&value.get(134).unwrap());
    }
    assert!(value.get(135).unwrap().is_null());
    // assert!(value.get(136).unwrap().is_null());
    let some_number = unpack_int_or_null(value.get(136).unwrap());
    assert!(some_number.is_none() || some_number == Some(1));
    assert!(value.get(137).unwrap().is_null());
    assert!(value.get(138).unwrap().is_null());
    assert!(value.get(139).unwrap().is_null());
    assert!(value.get(140).unwrap().is_null());
    assert!(value.get(141).unwrap().is_null());
    // assert!(value.get(142).unwrap().is_null());
    let business_level_attributes = unpack_array_or_null(value.get(142).unwrap());
    assert!(value.get(143).unwrap().is_null());
    assert!(value.get(144).unwrap().is_null());
    assert!(value.get(145).unwrap().is_null());

    // log(value.get(146).unwrap());
    let some_code_array = value.get(146).unwrap().as_array().unwrap();
    assert!(some_code_array.len() == 1);
    let some_code_array_item = some_code_array[0].as_i64().unwrap();

    assert!(value.get(147).unwrap().is_null());
    assert!(value.get(148).unwrap().is_null());
    assert!(value.get(149).unwrap().is_null());
    assert!(value.get(150).unwrap().is_null());
    assert!(value.get(150).unwrap().is_null());
    assert!(value.get(151).unwrap().is_null());
    assert!(value.get(152).unwrap().is_null());
    assert!(value.get(153).unwrap().is_null());
    assert!(value.get(154).unwrap().is_null());
    assert!(value.get(155).unwrap().is_null());
    assert!(value.get(156).unwrap().is_null());

    let google_account_avatars = unpack_str_or_null(value.get(157).unwrap());

    assert!(value.get(158).unwrap().is_null());
    assert!(value.get(159).unwrap().is_null());
    let _ = value.get(160).unwrap(); // SKIP
    assert!(value.get(161).unwrap().is_null());
    assert!(value.get(162).unwrap().is_null());
    assert!(value.get(163).unwrap().is_null());

    
    // Google Maps Local Finder (tbm=map) payloads
    let business_category_label_tuple = unpack_array_or_null(value.get(164).unwrap()); 
    {
        // TODO
    }
    
    
    let _ = unpack_array_or_null(value.get(165).unwrap()); // NO IDEA WHAT THIS IS — MAYBE SKIP
    
    
    let locality_string_city_level_location_names = unpack_str_or_null(value.get(166).unwrap());
    
    
    
    assert!(value.get(167).unwrap().is_null());
    assert!(value.get(168).unwrap().is_null());
    assert!(value.get(169).unwrap().is_null());
    // assert!(value.get(170).unwrap().is_null());

    let _ = unpack_int_or_null(value.get(170).unwrap());
    
    assert!(value.get(171).unwrap().is_null());
    assert!(value.get(172).unwrap().is_null());
    assert!(value.get(173).unwrap().is_null());
    
    // assert!(value.get(174).unwrap().is_null());
    // log(value.get(174).unwrap());

    let indirect_business_profile_links = value
        .get(174)
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .for_each(|x| {
            x.as_str().unwrap();
        });
    
    assert!(value.get(175).unwrap().is_null());
    assert!(value.get(176).unwrap().is_null());
    assert!(value.get(177).unwrap().is_null());
    
    let phone_number_entity_block = unpack_array_or_null(value.get(178).unwrap()).map(|xs| {

    });
    {
        // TODO
    }
    
    assert!(value.get(179).unwrap().is_null());
    assert!(value.get(180).unwrap().is_null());
    
    
    // assert!(value.get(181).unwrap().is_null());
    // log(value.get(181).unwrap());

    let some_user_or_entity_id_map = unpack_array_or_null(value.get(181).unwrap());
    {
        // TODO
    }
    
    
    
    assert!(value.get(182).unwrap().is_null());
    // assert!(value.get(183).unwrap().is_null());
    // log(value.get(183).unwrap());

    let location_normalization_payload = unpack_array_or_null(value.get(183).unwrap()).map(|x| {
        // TODO
    });
    {
        // TODO
    }
    
    
    assert!(value.get(184).unwrap().is_null());
    assert!(value.get(185).unwrap().is_null());
    assert!(value.get(186).unwrap().is_null());
    assert!(value.get(187).unwrap().is_null());
    assert!(value.get(188).unwrap().is_null());
    assert!(value.get(189).unwrap().is_null());
    assert!(value.get(190).unwrap().is_null());
    assert!(value.get(191).unwrap().is_null());
    assert!(value.get(192).unwrap().is_null());
    assert!(value.get(193).unwrap().is_null());
    assert!(value.get(194).unwrap().is_null());
    assert!(value.get(195).unwrap().is_null());
    
    // assert!(value.get(196).unwrap().is_null());
    // log(value.get(196).unwrap());

    let identity_label_metadata = unpack_array_or_null(value.get(196).unwrap()).map(|x| {
        // TODO
    });
    {
        // TODO
    }
    
    
    
    assert!(value.get(197).unwrap().is_null());
    assert!(value.get(198).unwrap().is_null());
    assert!(value.get(199).unwrap().is_null());
    assert!(value.get(200).unwrap().is_null());

    assert!(value.get(201).unwrap().is_null());
    assert!(value.get(202).unwrap().is_null());
    
    // assert!(value.get(203).unwrap().is_null());
    // log(value.get(203).unwrap());

    let structured_business_hours_payload = unpack_array_or_null(value.get(203).unwrap()).map(|x| {
        // TODO
    });
    {
        // TODO
    }


    assert!(value.get(204).unwrap().is_null());
    
    // assert!(value.get(205).unwrap().is_null());
    let _ = unpack_int_or_null(value.get(205).unwrap()); // SKIP

    
    assert!(value.get(206).unwrap().is_null());
    assert!(value.get(207).unwrap().is_null());
    let coordinate_blocks = unpack_array_or_null(value.get(208).unwrap()).map(|x| {
        // TODO
    });
    {
        // TODO
    }
    
    // assert!(value.get(209).unwrap().is_null());
    // log(value.get(209).unwrap());
    let b64_structured_concept_identifiers = value
        .get(209)
        .unwrap()
        .as_str()
        .unwrap();

    assert!(value.get(210).unwrap().is_null());
    assert!(value.get(211).unwrap().is_null());
    assert!(value.get(212).unwrap().is_null());
    assert!(value.get(213).unwrap().is_null());
    
    // assert!(value.get(214).unwrap().is_null());
    let _ = unpack_int_or_null(value.get(214).unwrap()); // SKIP
    
    assert!(value.get(215).unwrap().is_null());
    assert!(value.get(216).unwrap().is_null());
    assert!(value.get(217).unwrap().is_null());
    assert!(value.get(218).unwrap().is_null());
    assert!(value.get(219).unwrap().is_null());
    assert!(value.get(220).unwrap().is_null());
    assert!(value.get(221).unwrap().is_null());
    assert!(value.get(222).unwrap().is_null());
    assert!(value.get(223).unwrap().is_null());
    assert!(value.get(224).unwrap().is_null());
    assert!(value.get(225).unwrap().is_null());
    
    // assert!(value.get(226).unwrap().is_null());
    // log(value.get(226).unwrap());
    let business_claim_urls = unpack_array_or_null(value.get(226).unwrap()).map(|x| {
        // IDK
    });
    {
        // TODO
    }

    let google_maps_internal_business_entity_identifiers = value
        .get(227)
        .unwrap()
        .as_array()
        .unwrap();
    {
        // TODO
    }

    if value.len() == 228 {
        return
    }
    
    assert!(value.get(228).unwrap().is_null());

    assert!(value.get(229).unwrap().is_null());
    assert!(value.get(230).unwrap().is_null());
    assert!(value.get(231).unwrap().is_null());
    assert!(value.get(232).unwrap().is_null());
    assert!(value.get(233).unwrap().is_null());
    assert!(value.get(234).unwrap().is_null());
    assert!(value.get(235).unwrap().is_null());
    assert!(value.get(236).unwrap().is_null());
    assert!(value.get(237).unwrap().is_null());
    assert!(value.get(238).unwrap().is_null());
    assert!(value.get(239).unwrap().is_null());
    assert!(value.get(240).unwrap().is_null());
    assert!(value.get(241).unwrap().is_null());
    assert!(value.get(242).unwrap().is_null());

    let _ = value.get(243).unwrap().as_str().unwrap(); // COUNTRY CODE
    
    assert!(value.get(244).unwrap().is_null());

    let geographic_contextualization_metadata = value.get(245).unwrap().as_array().unwrap(); // TODO: geographic region ids
    {
        // TODO
    }

    assert!(value.len() == 246);
}

fn analyze_business_website_link(values: &[Value]) {
    let full_clickable_url = values.get(0);
    let display_domain = values.get(1);
    assert!(values.get(2).unwrap().is_null());
    // assert!(values.get(3).unwrap().is_null());
    let _ = values.get(3).unwrap(); // maybe some navigation result token
    let some_url_token = values.get(4);
    let google_serp_click_tracking_token = values.get(5);
}

fn analyze_business_geo_point(values: &[Value]) {
    assert!(values.get(0).unwrap().is_null());
    assert!(values.get(1).unwrap().is_null());
    let lat = values.get(2).unwrap().as_f64().unwrap();
    let lon = values.get(3).unwrap().as_f64().unwrap();
}

fn unpack_business_owner_metadata(value: &Value) {
    let value = value.as_array().unwrap();
    assert!(value.get(0).unwrap().is_null());
    let display_name = value.get(1).unwrap().as_str().unwrap(); // display name (organization or user) + suffix
    let google_identity_id = unpack_str_or_null(value.get(2).unwrap()); // Google identity ID (GMB or personal account)
    assert!(value.get(3).unwrap().is_null());
    assert!(value.get(4).unwrap().is_null());
    assert!(value.get(5).unwrap().is_null());
    assert!(value.get(6).unwrap().is_null());
    assert!(value.get(7).unwrap().is_null());
    let canonical_owner_id = value.get(8).unwrap().as_str().unwrap(); // ❗ Field [2] (Google ID) and [8] (Backup ID) Are Not Always Equal
}

fn unpack_business_cta_links(value: &[Value]) {
    for entry in value {
        unpack_main_groups(entry)
    }
    fn unpack_main_groups(value: &Value) {
        let value = value.as_array().unwrap();
        for entry in value {
            unpack_sub_groups(entry);
        }
    }
    fn unpack_sub_groups(value: &Value) {
        let value = value.as_array().unwrap();
        let some_index = value.get(0).unwrap().as_i64().unwrap();
        assert!(some_index == 3 || some_index == 4);
        assert!(value.get(1).unwrap().is_null());
        // log(value.get(2).unwrap());
        // let entries = unpack_array_or_null(value.get(2).unwrap());
        let _ = value
            .get(2)
            .unwrap()
            .as_array()
            .map(|xs| {
                xs
                    .into_iter()
                    .for_each(unpack_main_item_entry);
            });
        assert!(value.get(3).unwrap().is_null());
        let a_number = value.get(4).unwrap().as_i64().unwrap();
    }
    fn unpack_main_item_entry(value: &Value) {
        let value = value.as_array().unwrap();
        let source_and_media = unpack_source_and_media(value.get(0).unwrap());
        let link_and_metadata = unpack_link_and_metadata(value.get(1).unwrap());
    }
    fn unpack_source_and_media(value: &Value) {
        let value = value.as_array().unwrap();
        let display_domain_or_merchant_source = value.get(0).unwrap().as_str().unwrap();
        if value.len() == 1 {
            return
        }
        assert!(value.get(1).unwrap().is_null());
        let image_block = unpack_image_block(value.get(2).unwrap());
        let unknown_numeric_tag = value.get(3).unwrap().as_i64().unwrap();
    }
    fn unpack_link_and_metadata(value: &Value) {
        let value = value.as_array().unwrap();
        assert!(value.get(0).unwrap().is_null());
        assert!(value.get(1).unwrap().is_null());
        let link_and_metadata = unpack_link_group_item(value.get(2).unwrap());
    }
    fn unpack_link_group_item(value: &Value) {
        let value = value.as_array().unwrap();
        let primary_action_url = value.get(0);
        // let sub_group = value.get(1).unwrap().as_array().unwrap();
        // assert!(sub_group.len() == 5);
        // let redundant_url = sub_group.get(0);
        // assert!(sub_group.get(1).unwrap().is_null());
        // assert!(sub_group.get(2).unwrap().is_null());
        // assert!(sub_group.get(3).unwrap().is_null());
        // let some_other_value = sub_group.get(4).unwrap().as_str().unwrap();
    }
    fn unpack_link_group_item_redundant_confirmed_version_with_tracking_context(value: &Value) {
        let value = value.as_array().unwrap();
        let redundant_url = value.get(0);
        assert!(value.get(1).unwrap().is_null());
        assert!(value.get(2).unwrap().is_null());
        assert!(value.get(3).unwrap().is_null());
        let tracking_token = value.get(4).unwrap().as_str().unwrap();
        let some_other_token = value.get(5).unwrap().as_str().unwrap(); // Sometimes blank or missing — reserved slot for secondary token
        let search_context_token = value.get(6).unwrap().as_str().unwrap(); // Search context token (encodes query, position, timestamp)
    }
    fn unpack_image_block(value: &Value) {
        let value = value.as_array().unwrap();
        let image_url = value.get(0).unwrap().as_str().unwrap();
        let image_attribution_or_label = value.get(1).unwrap().as_str().unwrap();
        let image_dimensions = unpack_image_dimensions(value.get(2).unwrap());
    }
    fn unpack_image_dimensions(value: &Value) {
        let value = value.as_array().unwrap();
        let width = value.get(0).unwrap().as_f64();
        let height = value.get(1).unwrap().as_f64();
    }
}

fn unpack_business_category_vector(value: &Value) {
    // if !value.is_array() {
    //     log(value)
    // }
    let value = unpack_array_or_null(value).map(|items| {
        items.into_iter().map(unpack_business_category_entry).collect::<Vec<_>>()
    });
    fn unpack_business_category_entry(value: &Value) {
        let value = value.as_array().unwrap();
        let label = value.get(0).unwrap().as_str().unwrap();
        let label_value = unpack_str_or_null(value.get(1).unwrap());
        let confidence_tier = value.get(2).unwrap().as_i64().unwrap();
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

// ————————————————————————————————————————————————————————————————————————————
// DEBUG HELPERS
// ————————————————————————————————————————————————————————————————————————————

fn log<S: Serialize>(value: &S) {
    eprintln!("{} {}", "[log]".red(), format!("{}", serde_json::to_string(value).unwrap()).cyan())
}

fn sub_log<S: Serialize>(value: &S) {
    eprintln!("{}", format!("»» {}", serde_json::to_string(value).unwrap()).blue())
}

fn alpha_log<S: Serialize>(value: &S) {
    eprintln!("{}", format!("»» {}", serde_json::to_string(value).unwrap()).red())
}

