#![allow(unused_imports)]

#[macro_use]
extern crate nom;

use serde;

mod errors;
mod json_serde;

use nom::{
    branch::alt,
    bytes::complete::is_not,
    error::{context, convert_error, ErrorKind, ParseError,VerboseError},
    bytes::complete::{tag_no_case, take_while_m_n, take_while},
    character::complete::char,
    combinator::map_res,
    combinator::{map, opt, cut},
    number::complete::{le_i32, le_u32},
    sequence::delimited,
    sequence::tuple,
    IResult,
};
use std::str;

use errors::JsonError;
use json_serde::Value;

#[derive(Debug)]
pub struct JsonParser<'a> {
    input: &'a [u8],
    output: Vec<Value>,
}

impl<'a> JsonParser<'a> {
    pub fn parse(mut json: &'a str) -> Result<Self, JsonError> {
        let mut bytes: &[u8] = json.as_bytes();
        let mut res: Vec<Value> = Vec::new();
        while bytes.len() > 0 {
            let (temp_json, temp_res) = parse_bool(bytes)
                .map(|(rest, res)| (rest, Value::Bool(res)))
                .or_else(|e| Err(JsonError::ParseError(e.to_string())))?;

            bytes = temp_json;
            res.push(temp_res);
        }
        println!("{:?}", res);
        println!("{} {:?}", str::from_utf8(bytes).unwrap(), res);
        Ok(JsonParser {
            input: bytes,
            output: res,
        })
    }
}

fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_num(input: &[u8]) -> IResult<&[u8], i32> {
    // alt((le_i32, le_u32))(input)
    le_i32(input)
    // .map_err(|e| JsonError::ParseError(ErrorDetail::new(e.to_string())));
}

fn parse_bool(input: &[u8]) -> IResult<&[u8], bool> {
    alt((
        map(tag_no_case("true"), |_| true),
        map(tag_no_case("false"), |_| false)
    ))(input)
}

