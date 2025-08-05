#![allow(unused)]
use std::path::{Path, PathBuf};

// obfuscated.sample1.json.txt
// obfuscated.sample2.json.txt
// obfuscated.sample3.json.txt

fn main() {
    let output_directory = PathBuf::from(".output");
    let output_file = output_directory.join("all.json");
    let options = vec![
        "obfuscated.sample1.json.txt",
        "obfuscated.sample2.json.txt",
        "obfuscated.sample3.json.txt",
        "obfuscated.sample4.json.txt",
        "obfuscated.sample5.json.txt",
        "obfuscated.sample6.json.txt",
        "obfuscated.sample7.json.txt",
        "obfuscated.sample8.json.txt",
        "obfuscated.sample9.json.txt",
        "obfuscated.sample10.json.txt",
        "obfuscated.sample11.json.txt",
        "obfuscated.sample12.json.txt",
        "obfuscated.sample13.json.txt",
        "obfuscated.sample14.json.txt",
        "obfuscated.sample15.json.txt",
        "obfuscated.sample16.json.txt",
        "obfuscated.sample17.json.txt",
        "obfuscated.sample18.json.txt",
        "obfuscated.sample19.json.txt",
        "obfuscated.sample20.json.txt",
        "obfuscated.sample21.json.txt",
        "obfuscated.sample22.json.txt",
        "obfuscated.sample23.json.txt",
        "obfuscated.sample24.json.txt",
        "obfuscated.sample25.json.txt",
        "obfuscated.sample26.json.txt",
        "obfuscated.sample27.json.txt",
        "obfuscated.sample28.json.txt",
        "obfuscated.sample29.json.txt",
        "obfuscated.sample30.json.txt",
        "obfuscated.sample31.json.txt",
        "obfuscated.sample32.json.txt",
        "obfuscated.sample33.json.txt",
        "obfuscated.sample34.json.txt",
        "obfuscated.sample35.json.txt",
        "obfuscated.sample36.json.txt",
        "obfuscated.sample37.json.txt",
        "obfuscated.sample38.json.txt",
        "obfuscated.sample39.json.txt",
        "obfuscated.sample40.json.txt",
        "obfuscated.sample41.json.txt",
        "obfuscated.sample42.json.txt",
        "obfuscated.sample43.json.txt",
        "obfuscated.sample44.json.txt",
    ];
    let batch_directory = PathBuf::from("/Users/colbyn/Desktop/batch1");
    let values = options
        .iter()
        .map(|option| {
            eprintln!("{}", format!("> {option}"));
            let input_path = batch_directory.join(option);
            let value = ingest(&input_path);
            let debug_output_directory = output_directory
                .join("maps-output")
                .join(option);
            // std::fs::create_dir_all(&debug_output_directory).unwrap();
            // {
            //     let output_path = debug_output_directory.join("payload.json");
            //     let output_str = serde_json::to_string_pretty(&value).unwrap();
            //     std::fs::write(&output_path, &output_str).unwrap();
            // }
            // {
            //     let output_path = debug_output_directory.join("texts.organized.compact.txt");
            //     let chunks = value
            //         .as_array()
            //         .unwrap()
            //         .into_iter()
            //         .filter_map(|x| {
            //             x.as_array()
            //         })
            //         .flat_map(|xs| xs)
            //         .map(|x| {
            //             all_text_values(x.to_owned(), 0).join(" ")
            //         })
            //         .collect::<Vec<_>>()
            //         .join(&format!("\n{HR_RULE}\n"));
            //     let output_str = chunks;
            //     std::fs::write(&output_path, &output_str).unwrap();
            // }
            // {
            //     let output_path = debug_output_directory.join("texts.organized.lines.txt");
            //     let chunks = value
            //         .as_array()
            //         .unwrap()
            //         .into_iter()
            //         .filter_map(|x| {
            //             x.as_array()
            //         })
            //         .flat_map(|xs| xs)
            //         .map(|x| {
            //             all_text_values(x.to_owned(), 0).join("\n")
            //         })
            //         .collect::<Vec<_>>()
            //         .join(&format!("\n{HR_RULE}\n"));
            //     let output_str = chunks;
            //     std::fs::write(&output_path, &output_str).unwrap();
            // }
            {
                let output_path = debug_output_directory.join("deserialized.model.json");
                let entities =
                    google_types::map_search::deserialize::ingest_payload_extract_business_entities(&value);
                let output_str = serde_json::to_string_pretty(&entities).unwrap();
                std::fs::write(&output_path, &output_str).unwrap();
            }
            value
        })
        .collect::<Vec<_>>();
    // let values = serde_json::to_string_pretty(&values).unwrap();
    // std::fs::write(&output_file, &values).unwrap();
    // eprintln!("{values}");
}

fn ingest(input_path: &Path) -> serde_json::Value {
    let input_str = std::fs::read_to_string(&input_path).unwrap();
    let input_str = input_str.strip_suffix("/*\"\"*/").unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&input_str).unwrap();
    let object = json.as_object().unwrap();
    let field = object.get("d").unwrap();
    let target = field.as_str().unwrap();
    let target = target.strip_prefix(")]}'\n").unwrap();
    let value = serde_json::from_str::<serde_json::Value>(target).unwrap();
    value
}

fn all_text_values(value: serde_json::Value, level: usize) -> Vec<String> {
    if level > 4 {
        return Default::default()
    }
    match value {
        serde_json::Value::Null => {
            Default::default()
        }
        serde_json::Value::Bool(_) => {
            Vec::default()
        }
        serde_json::Value::Number(_) => {
            Vec::default()
        }
        serde_json::Value::String(text) => {
            vec![text]
        }
        serde_json::Value::Array(xs) => {
            xs  .into_iter()
                .flat_map(|x| all_text_values(x, level + 1))
                .collect::<Vec<_>>()
        }
        serde_json::Value::Object(xs) => {
            unimplemented!("TODO: {xs:?}")
        }
    }
}

const HR_RULE: &'static str = "———————————————————————————————————————————————————————————————————————————————";
