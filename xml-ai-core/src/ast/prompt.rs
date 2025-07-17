use crate::{ast::{breakpoint::BreakpointNode, message::MsgNode, set::SetNode}, common::prompt::PromptArguments};

// ————————————————————————————————————————————————————————————————————————————
// PROMPT CHILD NODE
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub enum PromptChildNode {
    Msg(MsgNode),
    Breakpoint(BreakpointNode),
    Set(SetNode),
}

// ————————————————————————————————————————————————————————————————————————————
// PROMPT ELEMENT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct PromptNode {
    pub settings: PromptArguments,
    pub children: Vec<PromptChildNode>,
}

impl PromptNode {
    pub fn name(&self) -> &str {
        &self.settings.name
    }
    pub fn tag_type() -> html_ast::TagBuf {
        html_ast::TagBuf::new("prompt")
    }
    pub fn matches(tag: &html_ast::TagBuf) -> bool {
        Self::tag_type().matches(tag)
    }
}
