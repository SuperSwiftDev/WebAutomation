use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum BreakpointType {
    Msg,
}

impl FromStr for BreakpointType {
    type Err = ParseBreakpointTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim();
        match normalized {
            "msg" => Ok(BreakpointType::Msg),
            _ => Err(ParseBreakpointTypeError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseBreakpointTypeError;

impl std::fmt::Display for ParseBreakpointTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unrecognized breakpoint type")
    }
}

impl std::error::Error for ParseBreakpointTypeError {}


