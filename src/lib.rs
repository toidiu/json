#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate nom;

use serde;

mod errors;
mod json_serde;

use nom::{
    branch::alt,
    bytes::complete::escaped,
    bytes::complete::is_not,
    bytes::complete::*,
    bytes::complete::{tag_no_case, take_while, take_while_m_n},
    character::complete::one_of,
    character::complete::*,
    character::*,
    combinator::map_res,
    combinator::{cut, map, opt},
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    number::complete::double,
    sequence::preceded,
    sequence::tuple,
    sequence::*,
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
    pub fn parse(json: &'a str) -> IResult<&'a str, bool> {
        context(
            "parse",
            preceded(
                parse_space,
                delimited(parse_arr_bracket, parse_bool, parse_arr_bracket),
            ),
        )(json)
    }
}

fn parse_arr_bracket<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
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

// fn parse_string<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
    // alphanumeric1(i)
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

    // #[test]
    // fn test_bool() {
    //     let mut json = "true";
    //     let mut out = parse_bool(json);
    //     assert_eq!(out.unwrap().1, true);

    //     json = "false";
    //     out = parse_bool(json);
    //     assert_eq!(out.unwrap().1, false);

    //     json = "afalse";
    //     out = parse_bool(json);
    //     assert!(out.is_err());
    // }

    #[test]
    fn test_number() {
        let json = "10";
        let out = parse_num(json);
        assert_eq!(out.unwrap().1, 10.0);
    }

    #[test]
    fn test_str() {
        let json = "ssstttrrr\\\"ing\\n";
        let out: IResult<&str, &str> = parse_str(json);
        let v = out.unwrap();
        println!("{:?}", v);
        assert_eq!(v.1, r#"ssstttrrr\"ing\n"#);
    }

    #[test]
    fn test_str_with_slash() {
        let json = "s\\s";
        let out: IResult<&str, &str> = parse_str(json);
        let v = out.unwrap();
        println!("{:?}", v);
        assert_eq!(v.1, r#"s\s"#);
    }

    #[test]
    fn test_string() {
        let json = "\"sss\\\"tttrrr\\ning\"";
        let out: IResult<&str, &str> = parse_string(json);
        let v = out.unwrap();
        println!("{:?}", v);
        assert_eq!(v.1, r#"sss\"tttrrr\ning"#);
    }

    // #[test]
    // fn test_object() {
    //     let mut json = "{\"foo\":10}";
    //     let mut out = parse_obj(json);
    //     // assert_eq!(out.unwrap().1, {});
    // }

    // #[test]
    // fn test_array() {
    //     let mut json = "[1,2,true]";
    //     let mut out = parse_arr_bracket(json);
    //     // assert_eq!(out.unwrap().1, true);
    // }

    // #[test]
    // fn test_parser() {
    //     let json = " [true, false] ";
    //     let out = JsonParser::parse(json);
    //     println!("======={:?}", out);
    //     assert_eq!(out.unwrap().1, true)
    // }
}
