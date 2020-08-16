#![allow(unused_imports)]

#[macro_use]
extern crate nom;

use serde;

mod errors;
mod json_serde;

use nom::{
    branch::alt,
    bytes::complete::is_not,
    bytes::complete::{tag_no_case, take_while, take_while_m_n},
    character::complete::char,
    combinator::map_res,
    combinator::{cut, map, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    number::complete::double,
    sequence::delimited,
    sequence::preceded,
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
    pub fn parse(mut json: &'a str) -> IResult<&'a str, bool> {
        preceded(parse_space, parse_bool)(json)
    }
}

fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_num<'a>(input: &'a str) -> IResult<&'a str, f64> {
    double(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((
        map(tag_no_case("true"), |_| true),
        map(tag_no_case("false"), |_| false),
    ))(input)
}

#[test]
fn bool() {
    let json = " truefalse";
    let out = JsonParser::parse(json);
    assert!(out.is_ok())
}
