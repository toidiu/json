use serde;

mod errors;
mod json_serde;

use errors::{ErrorDetail, JsonError};
use json_serde::Value;

#[derive(Debug)]
pub struct JsonParser<'a> {
    input: &'a str,
    output: Result<Value, JsonError>,
}

impl<'a> JsonParser<'a> {
    pub fn parse(json: &'a str) -> Self {
        let t = json
            .parse()
            .map(|b| Value::Bool(b))
            .or_else(|e| Err(JsonError::ParseError(ErrorDetail::new(e.to_string()))));
        JsonParser {
            input: json,
            output: t,
        }
    }
}

// impl Deserializer for JsonParser {}
