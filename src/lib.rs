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
    multi::{fold_many0, separated_list},
    number::complete::double,
    sequence::preceded,
    sequence::tuple,
    sequence::*,
    IResult,
};
use std::collections::HashMap;
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
            preceded(parse_space, delimited(parse_array, parse_bool, parse_array)),
        )(json)
    }
}

fn parse_value<'a>(input: &'a str) -> IResult<&'a str, f64> {
    context("space", consume_space(double))(input)
}

fn consume_space<'a, O, E, F>(f: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    map(
        tuple((
            parse_space, // ignore space
            f,
            parse_space, // ignore space
        )),
        |k| (k.1),
    )
}

// ============ DONE ============
fn parse_obj<'a>(i: &'a str) -> IResult<&'a str, HashMap<&str, f64>> {
    context(
        "object",
        preceded(
            char('{'),
            terminated(
                fold_many0(
                    parse_kv,
                    HashMap::new(),
                    |mut acc: HashMap<_, _>, (k, v)| {
                        acc.insert(k, v);
                        acc
                    },
                ),
                char('}'),
            ),
        ),
    )(i)
}

fn parse_kv<'a>(i: &'a str) -> IResult<&'a str, (&str, f64)> {
    context(
        "key value",
        map(
            tuple((
                consume_space(parse_string),
                tag(":"),
                parse_value,
                opt(tag(",")), // consume comma
            )),
            |k| (k.0, k.2),
        ),
    )(i)
}

fn parse_array<'a>(i: &'a str) -> IResult<&'a str, Vec<f64>> {
    context(
        "array",
        preceded(
            char('['),
            terminated(separated_list(tag(","), parse_value), char(']')),
        ),
    )(i)
}

fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    context("space", take_while(move |c| chars.contains(c)))(i)
}

fn parse_num<'a>(input: &'a str) -> IResult<&'a str, f64> {
    context("space", double)(input)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    // fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
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
        assert_eq!(v.1, r#"ssstttrrr\"ing\n"#);
    }

    // FIXME this needs to work
    // #[test]
    // fn test_str_with_slash() {
    //     let json = "s\\s";
    //     let out: IResult<&str, &str> = parse_str(json);
    //     let v = out.unwrap();
    //     println!("{:?}", v);
    //     assert_eq!(v.1, r#"s\s"#);
    // }

    #[test]
    fn test_string() {
        let json = "\"sss\\\"tttrrr\\ning\"";
        let out: IResult<&str, &str> = parse_string(json);
        let v = out.unwrap();
        assert_eq!(v.1, r#"sss\"tttrrr\ning"#);
    }

    #[test]
    fn test_space() {
        let json = " \t\none";
        let out: IResult<&str, &str> = parse_space(json);
        let v = out.unwrap();
        println!("{:?}", v);
        assert_eq!(v.0, "one");
        assert_eq!(v.1, " \t\n");
    }

    #[test]
    fn test_array() {
        let json = "[ 1 , 2 ]";
        let out = parse_array(json);
        let res: Vec<f64> = vec![1.0, 2.0];
        assert_eq!(out.unwrap().1, res);
    }

    #[test]
    fn test_kv() {
        let json = " \"k\" : 4,";
        let out = parse_kv(json).unwrap();
        assert_eq!(out.0, "");
        assert_eq!(out.1, ("k", 4.0));
    }

    #[test]
    fn test_object() {
        let json = "{\"foo\" :10 ,\"bar\":5}";
        let out = parse_obj(json);
        let mut res: HashMap<&str, f64> = HashMap::new();
        res.insert("foo", 10.0);
        res.insert("bar", 5.0);
        assert_eq!(out.unwrap().1, res);
    }

    #[test]
    fn test_consume_space() {
        let json = "\n true ";
        let parser = consume_space(parse_bool);
        let out = parser(json);
        assert_eq!(out.unwrap().1, true)
    }

    // #[test]
    // fn test_parser() {
    //     let json = " [true, false] ";
    //     let out = JsonParser::parse(json);
    //     println!("======={:?}", out);
    //     assert_eq!(out.unwrap().1, true)
    // }
}
