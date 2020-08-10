use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum JsonError {
    ParseError(String),
}

// #[derive(Debug)]
// pub struct ErrorDetail {
//     msg: String,
// }

// impl ErrorDetail {
//     pub fn new(msg: String) -> Self {
//         ErrorDetail { msg: msg }
//     }
// }

impl Error for JsonError {
    fn description(&self) -> &str {
        match self {
            JsonError::ParseError(detail) => &detail,
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonError::ParseError(detail) => write!(f, "{:?}", detail),
        }
    }
}

// Convert to IResult<&[u8], &[u8], ErrorStr>
impl From<String> for JsonError {
    fn from(i: String) -> Self {
        JsonError::ParseError(i)
    }
}
