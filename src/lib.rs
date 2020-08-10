#![allow(unused_imports)]

#[macro_use]
extern crate nom;

use serde;

mod errors;
mod json_serde;

use nom::{
    branch::alt,
    bytes::complete::is_not,
    bytes::complete::{tag_no_case, take_while_m_n},
    character::complete::char,
    combinator::map_res,
    sequence::delimited,
    sequence::tuple,
    IResult,
};

use errors::JsonError;
use json_serde::Value;

#[derive(Debug)]
pub struct JsonParser<'a> {
    input: &'a str,
    output: Value,
}

impl<'a> JsonParser<'a> {
    pub fn parse(json: &'a str) -> Result<Self, JsonError> {
        let (json, result) = parse_bool(json)
            .map(|(rest, result)| (rest, Value::Bool(result)))
            .or_else(|e| Err(JsonError::ParseError(e.to_string())))?;
        println!("{:?}", result);
        Ok(JsonParser {
            input: json,
            output: result,
        })
    }
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((tag_no_case("true"), tag_no_case("false")))(input)
        .map(|(rest, b)| (rest, b.parse().unwrap()))
    // .map_err(|e| JsonError::ParseError(ErrorDetail::new(e.to_string())));
}
