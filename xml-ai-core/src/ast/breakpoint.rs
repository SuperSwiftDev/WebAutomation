use crate::common::message::MessageRole;

#[derive(Debug, Clone)]
pub struct BreakpointNode {
    pub role: MessageRole,
}

impl BreakpointNode {
    pub fn tag_type() -> html_ast::TagBuf {
        html_ast::TagBuf::new("breakpoint")
    }
    pub fn matches(tag: &html_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
}
