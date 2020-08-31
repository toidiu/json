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
use json_serde::Value::{Arr, Bool, Number, Obj, Str};

#[derive(Debug)]
pub struct JsonParser<'a> {
    input: &'a str,
    pub output: Value<'a>,
}

impl<'a> JsonParser<'a> {
    pub fn new(json: &'a str) -> Self {
        let out = JsonParser::parse(json).unwrap();
        JsonParser {
            input: json,
            output: out.1,
        }
    }

    fn parse(json: &'a str) -> IResult<&'a str, Value> {
        context("parse", parse_value)(json)
    }
}

fn parse_value<'a>(input: &'a str) -> IResult<&'a str, Value> {
    context(
        "value",
        alt((
            map(consume_space(parse_num), |v| Number(v)),
            map(consume_space(parse_string), |v| v),
            map(consume_space(parse_bool), |v| Bool(v)),
            map(consume_space(parse_array), |v| Arr(v)),
            map(consume_space(parse_obj), |v| Obj(v)),
        )),
    )(input)
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

fn parse_kv<'a>(i: &'a str) -> IResult<&'a str, (Value, Value)> {
    context(
        "key value",
        map(
            tuple((consume_space(parse_string), tag(":"), parse_value)),
            |k| (k.0, k.2),
        ),
    )(i)
}

fn parse_obj<'a>(i: &'a str) -> IResult<&'a str, HashMap<Value, Value>> {
    context(
        "map",
        preceded(
            consume_space(char('{')),
            terminated(
                map(separated_list(char(','), parse_kv), |tuple_vec| {
                    tuple_vec.into_iter().map(|(k, v)| (k, v)).collect()
                }),
                tuple((
                    consume_space(opt(char(','))), // allow for optional ending comma
                    consume_space(char('}')),
                )),
            ),
        ),
    )(i)
}

fn parse_array<'a>(i: &'a str) -> IResult<&'a str, Vec<Value>> {
    context(
        "array",
        preceded(
            consume_space(char('[')),
            terminated(
                separated_list(char(','), parse_value),
                consume_space(char(']')),
            ),
        ),
    )(i)
}

fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    context("space", take_while(move |c| chars.contains(c)))(i)
}

fn parse_num<'a>(input: &'a str) -> IResult<&'a str, f64> {
    context("number", double)(input)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Value> {
    // fn parse_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        map(
            preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
            |v| Str(v),
        ),
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
    fn test_parse_bool() {
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
    fn test_parse_num() {
        let json = "10";
        let out = parse_num(json);
        assert_eq!(out.unwrap().1, 10.0);
    }

    #[test]
    fn test_parse_str() {
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
    fn test_parse_string() {
        let json = "\"sss\\\"tttrrr\\ning\"";
        let out = parse_string(json);
        let v = out.unwrap();
        assert_eq!(v.1, Str(r#"sss\"tttrrr\ning"#));
    }

    #[test]
    fn test_parse_space() {
        let json = " \t\none";
        let out: IResult<&str, &str> = parse_space(json);
        let v = out.unwrap();
        println!("{:?}", v);
        assert_eq!(v.0, "one");
        assert_eq!(v.1, " \t\n");
    }

    #[test]
    fn test_parse_array() {
        let json = "[ 1 , 2 ]";
        let out = parse_array(json);
        let res: Vec<Value> = vec![Number(1.0), Number(2.0)];
        assert_eq!(out.unwrap().1, res);
    }

    #[test]
    fn test_parse_kv() {
        let json = " \"k\" : 4";
        let out = parse_kv(json).unwrap();
        assert_eq!(out.0, "");
        assert_eq!(out.1, (Str("k"), Number(4.0)));
    }

    #[test]
    fn test_parse_object() {
        let json = " { \"foo\" : 10 , \"bar\": [1, true] , \"baz\": \"sss\", } ";
        let out = parse_obj(json).unwrap();
        let mut res = HashMap::new();
        res.insert(Str("foo"), Number(10.0));
        let vec: Vec<Value> = vec![Number(1.0), Bool(true)];
        res.insert(Str("bar"), Arr(vec));
        res.insert(Str("baz"), Str("sss"));
        assert_eq!(out.0, "");
        assert_eq!(out.1, res);
    }

    #[test]
    fn test_consume_space() {
        let json = "\n true \t ";
        let parser = consume_space(parse_bool);
        let out = parser(json);
        assert_eq!(out.unwrap().1, true);
    }

    #[test]
    fn test_parse_value() {
        let mut json = "\n 3 \t ";
        let mut out = parse_value(json);
        assert_eq!(out.unwrap().1, Value::Number(3.0));

        json = "\n true \t ";
        out = parse_value(json);
        assert_eq!(out.unwrap().1, Value::Bool(true));

        json = "\n \"sss\" \t ";
        out = parse_value(json);
        assert_eq!(out.unwrap().1, Value::Str("sss"));
    }

    #[test]
    fn test_parse_value_arr() {
        let json = "\n [1, true , 3, \"sss\"] \t ";
        let out = parse_value(json);
        let res: Vec<Value> = vec![Number(1.0), Bool(true), Number(3.0), Str("sss")];
        assert_eq!(out.unwrap().1, Arr(res));
    }

    #[test]
    fn test_parse_value_obj() {
        let json = " { \"foo\" : 10 , \"bar\": [1, true] , \"baz\": \"sss\", } ";
        let out = parse_value(json).unwrap();
        let mut res = HashMap::new();
        res.insert(Str("foo"), Number(10.0));
        let vec: Vec<Value> = vec![Number(1.0), Bool(true)];
        res.insert(Str("bar"), Arr(vec));
        res.insert(Str("baz"), Str("sss"));
        assert_eq!(out.0, "");
        assert_eq!(out.1, Obj(res));
    }
}
