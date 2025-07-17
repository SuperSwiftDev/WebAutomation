use crate::common::message::MessageRole;

#[derive(Debug, Clone)]
pub struct MsgNode {
    pub role: MessageRole,
    pub breakpoint_mode: bool,
    pub children: html_ast::Fragment,
}

impl MsgNode {
    pub fn tag_type() -> html_ast::TagBuf {
        html_ast::TagBuf::new("msg")
    }
    pub fn matches(tag: &html_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
}

