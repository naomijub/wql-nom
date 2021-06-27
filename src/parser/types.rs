use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::streaming::{escaped, tag, take_while},
    character::{
        complete::{anychar, char},
        is_alphanumeric, is_newline, is_space,
        streaming::one_of,
    },
    combinator::{cut, map, map_res, recognize, value},
    error::{context, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    AsChar, IResult, InputTakeAtPosition,
};
use uuid::Uuid;

use crate::model::types::{wql_value, Types};

pub fn uuid_parser(s: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    map_res(recognize(alphanumerichyphen), Uuid::parse_str)(s)
}
pub fn alphanumerichyphen1(s: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alphanumerichyphen(s)
}

pub fn alphanumerickey1(s: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alphanumerickey(s)
}

pub fn set(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    context(
        "keys set",
        preceded(
            tag("#{"),
            cut(terminated(
                separated_list0(preceded(char(','), sp), alphanumerickey),
                alt((char('}'), preceded(sp, char('}')))),
            )),
        ),
    )(input)
    .map(|(next_input, res)| {
        let res: Vec<String> = res.iter().map(|e| e.to_string()).collect();
        (next_input, res)
    })
}

pub fn hashmap(input: &str) -> IResult<&str, HashMap<String, Types>, VerboseError<&str>> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, Types), VerboseError<&str>> {
    separated_pair(
        preceded(sp, alphanumerickey),
        cut(preceded(sp, char(':'))),
        wql_value,
    )(input)
}

pub fn vector(input: &str) -> IResult<&str, Vec<Types>, VerboseError<&str>> {
    context(
        "vector",
        preceded(
            tag("["),
            cut(terminated(
                separated_list0(preceded(char(','), sp), wql_value),
                alt((char(']'), preceded(sp, char(']')))),
            )),
        ),
    )(input)
}

pub fn string(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
    .map(|(next, res)| (next, res.to_string()))
}

pub fn boolean(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

fn parse_str(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    escaped(alphanumericall, '\\', one_of("\"n\\"))(i)
}

fn alphanumerichyphen<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '-' || is_alphanumeric(ch as u8))
        },
        ErrorKind::AlphaNumeric,
    )
}

fn alphanumerickey<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '_' || is_alphanumeric(ch as u8))
        },
        ErrorKind::AlphaNumeric,
    )
}

fn alphanumericall<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '_'
                || ch == '-'
                || is_space(ch as u8)
                || is_newline(ch as u8)
                || ch == ','
                || ch == '.'
                || ch == '?'
                || ch == '!'
                || ch == '@'
                || ch == '%'
                || ch == '#'
                || ch == '$'
                || ch == '&'
                || ch == '*'
                || ch == '+'
                || ch == '='
                || ch == '('
                || ch == ')'
                || ch == '['
                || ch == ']'
                || ch == '{'
                || ch == '}'
                || ch == '|'
                || ch == ':'
                || ch == ';'
                || ch == '/'
                || ch == '>'
                || ch == '<'
                || is_alphanumeric(ch as u8))
        },
        ErrorKind::AlphaNumeric,
    )
}

// Any space char
pub(crate) fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn alphanumerichyphen() {
        assert_eq!(
            alphanumerichyphen1("c15a23cd-22d8-4351-b738-396b274599f8 WTF"),
            Ok((" WTF", "c15a23cd-22d8-4351-b738-396b274599f8"))
        );
    }

    #[test]
    fn uuid() {
        assert_eq!(
            uuid_parser("c15a23cd-22d8-4351-b738-396b274599f8 WTF"),
            Ok((
                " WTF",
                Uuid::from_str("c15a23cd-22d8-4351-b738-396b274599f8").unwrap()
            ))
        );
    }

    #[test]
    fn set_test() {
        assert_eq!(
            set("#{hello, hello_world, hello_123} WTF"),
            Ok((
                " WTF",
                vec![
                    "hello".to_owned(),
                    "hello_world".to_owned(),
                    "hello_123".to_owned()
                ]
            ))
        );

        assert_eq!(
            set("#{hello, hello_world, hello_123 } WTF"),
            Ok((
                " WTF",
                vec![
                    "hello".to_owned(),
                    "hello_world".to_owned(),
                    "hello_123".to_owned()
                ]
            ))
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            string("\"hello world\"}"),
            Ok(("}", String::from("hello world")))
        )
    }

    #[test]
    fn vector_of_bools() {
        assert_eq!(
            vector("[\"a\", true, false]"),
            Ok((
                "",
                vec![
                    Types::String("a".to_owned()),
                    Types::Boolean(true),
                    Types::Boolean(false)
                ]
            ))
        )
    }

    #[test]
    fn bools() {
        assert_eq!(boolean("true, "), Ok((", ", true)));
        assert_eq!(boolean("false, "), Ok((", ", false)))
    }
}
