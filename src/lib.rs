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
        context(
            "parse",
            preceded(parse_space, delimited(parse_arr, parse_bool, parse_arr)),
        )(json)
    }
}

fn parse_arr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = "[]";
    context("arr", take_while(move |c| chars.contains(c)))(i)
}

fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    context("space", take_while(move |c| chars.contains(c)))(i)
}

fn parse_num<'a>(input: &'a str) -> IResult<&'a str, f64> {
    context("space", double)(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    context(
        "bool",
        alt((
            map(tag_no_case("true"), |_| true),
            map(tag_no_case("false"), |_| false),
        )),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bool() {
        let mut json = "true";
        let mut out = parse_bool(json);
        assert_eq!(out.unwrap().1, true);

        json = "false";
        out = parse_bool(json);
        assert_eq!(out.unwrap().1, false);

        json = "afalse";
        out = parse_bool(json);
        assert!(out.is_err());
    }

    #[test]
    fn test_number() {}

    #[test]
    fn test_object() {}

    #[test]
    fn test_array() {}

    #[test]
    fn test_string() {}

    #[test]
    fn test_parser() {
        let json = " [true, false] ";
        let out = JsonParser::parse(json);
        println!("======={:?}", out);
        assert_eq!(out.unwrap().1, true)
    }
}
