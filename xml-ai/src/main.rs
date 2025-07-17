use std::path::PathBuf;

use xml_ai_core::runtime::{DocumentInvocation, RuntimeEnvironment};

extern crate super_html_ast as html_ast;

pub mod cli;

#[tokio::main]
async fn main() {
    let source = include_str!("../../notes/StandaloneExamples.html");
    let html_tree = html_ast::parser::parse_from_fragment(source).expect("valid html content");
    let document = xml_ai_core::ast::document::DocumentNode::from_node(html_tree).expect("should be a valid node");
    let api_key = std::fs::read_to_string("secrets/open-ai.key").expect("API key file");
    let document_invocation = DocumentInvocation {
        runtime_environment: RuntimeEnvironment {
            api_key,
        },
        target_prompt: String::from("question-1"),
    };
    let prompt_context = document.invoke(&document_invocation).await.unwrap();
    let conversation_snapshot = prompt_context.to_snapshot();
    let conversation_snapshot_path = PathBuf::from(".xml-ai/latest.json");
    let conversation_snapshot_json = serde_json::to_string_pretty(&conversation_snapshot).unwrap();
    std::fs::create_dir_all(conversation_snapshot_path.parent().unwrap()).unwrap();
    std::fs::write(&conversation_snapshot_path, &conversation_snapshot_json).unwrap();
    println!("DONE:");
    println!("{:#?}", conversation_snapshot);
}
