use nom::{
    character::is_alphanumeric,
    combinator::{map_res, recognize},
    error::{ErrorKind, ParseError, VerboseError},
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