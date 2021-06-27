use std::collections::HashMap;

use chrono::{DateTime, Utc};
use nom::{
    branch::alt,
    bytes::streaming::{escaped, tag, tag_no_case, take_while},
    character::{
        complete::{anychar, char},
        is_alphanumeric, is_digit, is_newline, is_space,
        streaming::one_of,
    },
    combinator::{cut, map, map_res, recognize, value},
    error::{context, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated},
    AsChar, IResult, InputTakeAtPosition,
};
use uuid::Uuid;

use crate::model::types::{wql_value, Nil, Types};

pub fn uuid_parser(s: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    map_res(recognize(alphanumerichyphen), Uuid::parse_str)(s)
}

pub fn precise_number_parser(num: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "precise_number",
        recognize(terminated(precise_number, char('P'))),
    )(num)
    .map(|(next, res)| (next, res.to_string()))
}

pub fn datetime_parser(datetime: &str) -> IResult<&str, DateTime<Utc>, VerboseError<&str>> {
    map_res(recognize(datetime_chars), underlay_datetimeparser)(datetime)
}

fn underlay_datetimeparser(datetime: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    datetime.parse::<DateTime<Utc>>()
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
                alt((tag("}"), tag(",}"), preceded(sp, tag("}")))),
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
                alt((tag("}"), tag(",}"), preceded(sp, tag("}")))),
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
                alt((tag("]"), tag(",]"), preceded(sp, tag("]")))),
            )),
        ),
    )(input)
}

pub fn string(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(input)
    .map(|(next, res)| (next, res.to_string()))
}

pub fn char_parse(input: &str) -> IResult<&str, char, VerboseError<&str>> {
    context(
        "char",
        preceded(char('\''), cut(terminated(anychar, char('\'')))),
    )(input)
}

pub fn boolean(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

pub fn nil(input: &str) -> IResult<&str, Nil, VerboseError<&str>> {
    let mut parse_nil = value(Nil, tag_no_case("nil"));

    parse_nil(input)
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

fn datetime_chars<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '-' || ch == 'T' || ch == 'Z' || ch == ':' || ch == '+' || is_digit(ch as u8))
        },
        ErrorKind::AlphaNumeric,
    )
}

fn precise_number<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '.' || is_digit(ch as u8))
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
    use chrono::prelude::*;
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
            vector("[\"a\", true, false, nil]"),
            Ok((
                "",
                vec![
                    Types::String("a".to_owned()),
                    Types::Boolean(true),
                    Types::Boolean(false),
                    Types::Nil(Nil)
                ]
            ))
        )
    }

    #[test]
    fn vector_of_bools_comma() {
        assert_eq!(
            vector("[\"a\", true, false, nil,]"),
            Ok((
                "",
                vec![
                    Types::String("a".to_owned()),
                    Types::Boolean(true),
                    Types::Boolean(false),
                    Types::Nil(Nil)
                ]
            ))
        )
    }

    #[test]
    fn vector_of_bools_sp() {
        assert_eq!(
            vector("[\"a\", true, false, nil  ]"),
            Ok((
                "",
                vec![
                    Types::String("a".to_owned()),
                    Types::Boolean(true),
                    Types::Boolean(false),
                    Types::Nil(Nil)
                ]
            ))
        )
    }

    #[test]
    fn bools() {
        assert_eq!(boolean("true, "), Ok((", ", true)));
        assert_eq!(boolean("false, "), Ok((", ", false)))
    }

    #[test]
    fn parse_map_with_uuid() {
        assert_eq!(
            Ok((
                "",
                vec![(
                    "a".to_owned(),
                    Types::Uuid(Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap())
                )]
                .iter()
                .cloned()
                .collect::<HashMap<String, Types>>()
            )),
            hashmap("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8}")
        );
    }

    #[test]
    fn parse_map_with_uuid_trailing_comma() {
        assert_eq!(
            Ok((
                "",
                vec![(
                    "a".to_owned(),
                    Types::Uuid(Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap())
                )]
                .iter()
                .cloned()
                .collect::<HashMap<String, Types>>()
            )),
            hashmap("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8,}")
        );
    }

    #[test]
    fn parse_map_with_uuid_trailing_spaces() {
        assert_eq!(
            Ok((
                "",
                vec![(
                    "a".to_owned(),
                    Types::Uuid(Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap())
                )]
                .iter()
                .cloned()
                .collect::<HashMap<String, Types>>()
            )),
            hashmap("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8   }")
        );
    }

    #[test]
    fn test_char_parse() {
        assert_eq!(char_parse("'h'"), Ok(("", 'h')));
        assert_eq!(char_parse("'3', hello"), Ok((", hello", '3')));
        assert_eq!(char_parse("','"), Ok(("", ',')));
    }

    #[test]
    fn datetime_test() {
        assert_eq!(
            datetime_parser("2014-11-28T12:00:09Z"),
            Ok(("", datetime()))
        );
        assert_eq!(
            datetime_parser("2014-11-28T21:00:09+09:00 wow that was a date?"),
            Ok((" wow that was a date?", datetime()))
        );
    }

    #[test]
    fn parse_map_with_uuid_with_datetime() {
        assert_eq!(
            Ok((
                "",
                vec![
                    (
                        "a".to_owned(),
                        Types::Uuid(
                            Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap()
                        )
                    ),
                    ("date".to_owned(), Types::DateTime(datetime()))
                ]
                .iter()
                .cloned()
                .collect::<HashMap<String, Types>>()
            )),
            hashmap("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8, date: 2014-11-28T12:00:09Z}")
        );
    }

    #[test]
    fn precise_numbers_test() {
        assert_eq!(
            precise_number_parser("124638374P"),
            Ok(("", String::from("124638374P")))
        );
        assert_eq!(
            precise_number_parser("12463.8374P"),
            Ok(("", String::from("12463.8374P")))
        );
    }

    fn datetime() -> DateTime<Utc> {
        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        dt
    }
}
