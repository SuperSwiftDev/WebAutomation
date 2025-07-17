use crate::{AttributeKeyBuf, AttributeMap, AttributeValueBuf, Fragment, Node, TagBuf};

#[derive(Debug, Clone)]
pub struct ParseResult<T> {
    output: T,
    errors: Vec<String>,
}

impl<T> ParseResult<T> {
    pub fn errors(&self) -> &[String] {
        self.errors.as_ref()
    }
    pub fn log_errors(&self) {
        for error in self.errors.iter() {
            eprintln!("⚠️ {error}")
        }
    }
    pub fn expect(self, message: impl AsRef<str>) -> T {
        let message = message.as_ref();
        if !self.errors.is_empty() {
            self.log_errors();
            panic!("{}", message)
        }
        self.output
    }
}

impl ParseResult<Node> {
    pub fn html(&self) -> Option<&Node> {
        if self.errors.is_empty() {
            Some(&self.output)
        } else {
            None
        }
    }
    pub fn transform(self, apply: impl FnOnce(Node) -> Node) -> Self {
        Self { output: apply(self.output), errors: self.errors }
    }
}

pub fn parse_from_fragment(source: impl AsRef<str>) -> ParseResult<Node> {
    let result = scraper::Html::parse_fragment(source.as_ref());
    transform_scraper_html(result).transform(|node| {
        let nodes = node
            .flatten()
            .into_iter()
            .flat_map(|x| x.extract_elements())
            .map(|element| {
                let tag = TagBuf::from("html");
                if element.tag.matches(&tag) {
                    return Node::Fragment(element.children)
                }
                Node::Element(element)
            })
            .flat_map(|x| x.flatten())
            .collect::<Vec<_>>();
        Node::Fragment(Fragment::from_nodes(nodes))
    })
}

pub fn parse_from_document(source: impl AsRef<str>) -> ParseResult<Node> {
    let result = scraper::Html::parse_document(source.as_ref());
    transform_scraper_html(result)
}

fn transform_scraper_html(html: scraper::Html) -> ParseResult<Node> {
    let errors = html.errors
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let root = html.tree.root();
    let converted = convert_ego_tree(root);
    ParseResult { output: converted, errors }
}

fn convert_ego_tree(node: ego_tree::NodeRef<'_, scraper::node::Node>) -> Node {
    match node.value() {
        scraper::node::Node::Text(text) => {
            Node::text(text.to_string())
        }

        scraper::node::Node::Element(element) => {
            let tag = TagBuf::new(element.name.local.to_string());

            let attributes: AttributeMap = element.attrs.iter()
                .map(|(key, value)| {
                    (
                        AttributeKeyBuf::from(key.local.to_string()),
                        AttributeValueBuf::literal(value.to_string()),
                    )
                })
                .collect();

            let children: Fragment = Fragment::from_iter(
                node.children().map(convert_ego_tree)
            );

            Node::element(tag, attributes, children)
        }

        scraper::node::Node::Comment(_) | scraper::node::Node::Doctype(_) | scraper::node::Node::Document | scraper::node::Node::Fragment => {
            let children: Fragment = Fragment::from_iter(
                node.children().map(convert_ego_tree)
            );
            Node::Fragment(children)
        }

        scraper::node::Node::ProcessingInstruction(_) => {
            // You can choose to skip this, or add a new Node variant if needed.
            Node::empty()
        }

    }
}
