use crate::common::prompt::PromptSettings;

#[derive(Debug, Clone)]
pub struct SetNode {
    pub prompt_settings: PromptSettings,
}

impl SetNode {
    pub fn tag_type() -> html_ast::TagBuf {
        html_ast::TagBuf::new("set")
    }
    pub fn matches(tag: &html_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
}

