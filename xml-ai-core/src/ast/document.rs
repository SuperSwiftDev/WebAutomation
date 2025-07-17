use crate::ast::prompt::PromptNode;

#[derive(Debug, Clone)]
pub enum DocumentChildCode {
    Prompt(PromptNode),
}

#[derive(Debug, Clone)]
pub struct DocumentNode {
    pub children: Vec<DocumentChildCode>,
}
