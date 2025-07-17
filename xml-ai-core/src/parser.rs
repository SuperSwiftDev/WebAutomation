use std::ops::Not;
use std::rc::Rc;
use std::str::FromStr;

use crate::ast::breakpoint::BreakpointNode;
use crate::ast::document::{DocumentChildCode, DocumentNode};
use crate::ast::message::MsgNode;
use crate::ast::prompt::{PromptChildNode, PromptNode};
use crate::ast::set::SetNode;
use crate::common::message::MessageRole;
use crate::common::prompt::PromptSettings;

// ————————————————————————————————————————————————————————————————————————————
// ERROR HANDLING
// ————————————————————————————————————————————————————————————————————————————

pub trait DslFormatError: std::error::Error + std::fmt::Debug {
    fn singleton(&self) -> DslFormatErrorList;
}

#[derive(Debug, Clone)]
pub struct DslFormatErrorList {
    pub errors: Vec<Rc<dyn DslFormatError>>,
}

impl DslFormatErrorList {
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn with_capacity(len: usize) -> Self {
        Self { errors: Vec::<_>::with_capacity(len) }
    }
    pub fn new(item: Rc<dyn DslFormatError>) -> Self {
        let errors = vec![ item ];
        Self { errors }
    }
    pub fn union(mut self, other: Self) -> Self {
        self.errors.extend(other.errors);
        self
    }
    pub fn push<T: DslFormatError + 'static>(&mut self, new: Rc<T>) {
        self.errors.push(new);
    }
    pub fn extend(&mut self, other: Self) {
        self.errors.extend(other.errors);
    }
    pub fn join<T: DslFormatError + 'static>(mut self, next: Rc<T>) {
        self.errors.push(next);
    }
    pub fn joined(&self, separator: impl AsRef<str>) -> String {
        self.errors
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<_>>()
            .join(separator.as_ref())
    }
}

impl std::fmt::Display for DslFormatErrorList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.joined(" ∙ ");
        write!(f, "{}", items)
    }
}

impl<T: DslFormatError> From<T> for DslFormatErrorList {
    fn from(value: T) -> Self {
        value.singleton()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// MESSAGE NODE
// ————————————————————————————————————————————————————————————————————————————

impl MsgNode {
    pub fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidMessageNode)))
        }
        let role = element.attributes
            .get("role")
            .ok_or_else(|| {
                InvalidMessageMissingRole
            })?;
        let role = MessageRole::from_str(role.as_str())
            .map_err(|x| InvalidMessageAttribute(x))?;
        let children = element.children;
        Ok(Self {
            role,
            breakpoint_mode: false,
            children,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageNode;
impl std::fmt::Display for InvalidMessageNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message element")
    }
}
impl std::error::Error for InvalidMessageNode {}
impl DslFormatError for InvalidMessageNode {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageAttribute(pub crate::common::message::ParseRoleError);
impl std::fmt::Display for InvalidMessageAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message attribute: {}", self.0)
    }
}
impl std::error::Error for InvalidMessageAttribute {}
impl DslFormatError for InvalidMessageAttribute {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

#[derive(Debug, Clone)]
pub struct InvalidMessageMissingRole;
impl std::fmt::Display for InvalidMessageMissingRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message: missing role attribute")
    }
}
impl std::error::Error for InvalidMessageMissingRole {}
impl DslFormatError for InvalidMessageMissingRole {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// BREAKPOINT NODE
// ————————————————————————————————————————————————————————————————————————————

impl BreakpointNode {
    pub fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidBreakpointNode)))
        }
        let role = element.attributes
            .get("role")
            .ok_or_else(|| {
                InvalidBreakpointAttribute
            })?;
        let role = MessageRole::from_str(role.as_str())
            .map_err(|_| InvalidBreakpointAttribute)?;
        Ok(Self {
            role,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidBreakpointNode;
impl std::fmt::Display for InvalidBreakpointNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid message element")
    }
}
impl std::error::Error for InvalidBreakpointNode {}
impl DslFormatError for InvalidBreakpointNode {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

#[derive(Debug, Clone)]
pub struct InvalidBreakpointAttribute;
impl std::fmt::Display for InvalidBreakpointAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid breakpoint attribute")
    }
}
impl std::error::Error for InvalidBreakpointAttribute {}
impl DslFormatError for InvalidBreakpointAttribute {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// SET NODE
// ————————————————————————————————————————————————————————————————————————————

impl SetNode {
    pub fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        let mut prompt_settings = PromptSettings::default();
        for (key, value) in element.attributes.iter() {
            match prompt_settings.try_merge(key, value.as_str()) {
                Some(Ok(())) => (),
                Some(Err(_)) => {
                    return Err(DslFormatErrorList::new(Rc::new(InvalidSetAttribute)))
                },
                None => (),
            }
        }
        Ok(Self {
            prompt_settings,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidSetNode;
impl std::fmt::Display for InvalidSetNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid set element")
    }
}
impl std::error::Error for InvalidSetNode {}
impl DslFormatError for InvalidSetNode {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}


#[derive(Debug, Clone)]
pub struct InvalidSetAttribute;
impl std::fmt::Display for InvalidSetAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid set attribute")
    }
}
impl std::error::Error for InvalidSetAttribute {}
impl DslFormatError for InvalidSetAttribute {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// PROMPT CHILD
// ————————————————————————————————————————————————————————————————————————————

impl PromptChildNode {
    pub fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        if MsgNode::matches(&element.tag) {
            return MsgNode::from_element(element).map(Self::Msg)
        }
        if BreakpointNode::matches(&element.tag) {
            return BreakpointNode::from_element(element).map(Self::Breakpoint)
        }
        if SetNode::matches(&element.tag) {
            return SetNode::from_element(element).map(Self::Set)
        }
        Err(DslFormatErrorList::new(Rc::new(InvalidPromptChild)))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPromptChild;
impl std::fmt::Display for InvalidPromptChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt child")
    }
}
impl std::error::Error for InvalidPromptChild {}
impl DslFormatError for InvalidPromptChild {
    fn singleton(&self) -> DslFormatErrorList {
        DslFormatErrorList::new(Rc::new(self.clone()))
    }
}

// ————————————————————————————————————————————————————————————————————————————
// PROMPT
// ————————————————————————————————————————————————————————————————————————————

impl PromptNode {
    pub fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        if Self::matches(&element.tag).not() {
            return Err(DslFormatErrorList::new(Rc::new(InvalidPromptNode)))
        }
        let mut prompt_settings = PromptSettings::default();
        for (key, value) in element.attributes.iter() {
            match prompt_settings.try_merge(key, value.as_str()) {
                Some(Ok(())) => (),
                Some(Err(error)) => {
                    return Err(DslFormatErrorList::new(Rc::new(InvalidPromptAttribute(error))))
                },
                None => (),
            }
        }
        let prompt_attributes = prompt_settings
            .build_to_prompt_arguments()
            .ok_or_else(|| InvalidPromptMissingName)?;
        let mut items = Vec::<PromptChildNode>::with_capacity(element.children.len());
        let mut errors = DslFormatErrorList::with_capacity(element.children.len());
        for child in element.extract_child_elements() {
            match PromptChildNode::from_element(child) {
                Ok(item) => {
                    items.push(item);
                }
                Err(error) => {
                    errors.extend(error);
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors)
        }
        Ok(Self {
            settings: prompt_attributes,
            children: items,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPromptNode;
impl std::fmt::Display for InvalidPromptNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt element")
    }
}
impl std::error::Error for InvalidPromptNode {}
impl DslFormatError for InvalidPromptNode {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

#[derive(Debug, Clone)]
pub struct InvalidPromptMissingName;
impl std::fmt::Display for InvalidPromptMissingName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt: missing name")
    }
}
impl std::error::Error for InvalidPromptMissingName {}
impl DslFormatError for InvalidPromptMissingName {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

#[derive(Debug, Clone)]
pub struct InvalidPromptAttribute(pub crate::common::prompt::InvalidAttribute);
impl std::fmt::Display for InvalidPromptAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid prompt attribute: {}", self.0)
    }
}
impl std::error::Error for InvalidPromptAttribute {}
impl DslFormatError for InvalidPromptAttribute {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT CHILD NODE
// ————————————————————————————————————————————————————————————————————————————

impl DocumentChildCode {
    fn from_element(element: html_ast::Element) -> Result<Self, DslFormatErrorList> {
        if PromptNode::matches(&element.tag) {
            return PromptNode::from_element(element).map(DocumentChildCode::Prompt)
        }
        eprintln!("Failed: {element:?}");
        Err(DslFormatErrorList::new(Rc::new(InvalidDocumentChildNode)))
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDocumentChildNode;
impl std::fmt::Display for InvalidDocumentChildNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid document child node")
    }
}

impl std::error::Error for InvalidDocumentChildNode {}
impl DslFormatError for InvalidDocumentChildNode {
    fn singleton(&self) -> DslFormatErrorList { DslFormatErrorList::new(Rc::new(self.clone())) }
}

// ————————————————————————————————————————————————————————————————————————————
// DOCUMENT
// ————————————————————————————————————————————————————————————————————————————

impl DocumentNode {
    pub fn from_fragment(fragment: html_ast::Fragment) -> Result<Self, DslFormatErrorList> {
        let mut items = Vec::<DocumentChildCode>::with_capacity(fragment.len());
        let mut errors = DslFormatErrorList::with_capacity(fragment.len());
        for child in fragment.extract_elements() {
            match DocumentChildCode::from_element(child) {
                Ok(item) => {
                    items.push(item);
                }
                Err(error) => {
                    errors.extend(error);
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors)
        }
        Ok(Self { children: items })
    }
    pub fn from_node(node: html_ast::Node) -> Result<Self, DslFormatErrorList> {
        Self::from_fragment(html_ast::Fragment::from_nodes(node.flatten()))
    }
}

