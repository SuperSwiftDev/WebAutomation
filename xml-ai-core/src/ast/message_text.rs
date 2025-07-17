use html_ast::{Element, Node};

use crate::ast::message::MsgNode;

impl MsgNode {
    pub fn text_content(&self) -> String {
        if let Ok(text_only) = self.children.clone().extract_text_strict() {
            return text_only.join("")
        }
        self.children
            .iter()
            .flat_map(|x| x.to_owned().flatten())
            .filter_map(|x| {
                match x {
                    Node::Element(element) => {
                        Some(to_text_content(element))
                    },
                    Node::Text(_) => {
                        None
                    }
                    Node::Fragment(_) => {
                        panic!("NOT POSSIBLE")
                    },
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn to_text_content(element: Element) -> String {
    element.children.extract_text_strict().unwrap().join("")
}
