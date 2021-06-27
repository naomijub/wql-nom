use nom::{
    branch::alt,
    bytes::streaming::{escaped, tag, take_while},
    character::{complete::char, is_alphanumeric, streaming::one_of},
    combinator::{cut, map_res, recognize},
    error::{context, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    sequence::{preceded, terminated},
    AsChar, IResult, InputTakeAtPosition,
};
use uuid::Uuid;

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

fn string(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn parse_str(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    escaped(alphanumerickey, '\\', one_of("\"n\\"))(i)
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
}
