use std::collections::HashMap;

use nom::{
    branch::alt,
    error::VerboseError,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use uuid::Uuid;

use crate::{
    model::{types::Types, CreateOptions},
    parser::keywords::{create_options, entity},
};

use crate::parser::{
    types::sp,
    {
        keywords::into,
        types::{alphanumerickey1, hashmap},
    },
};

use super::{
    keywords::{content, from, set as keyword_set, with},
    types::{alphanumericboth1, set, uuid_parser},
};

pub fn create_content(
    input: &str,
) -> IResult<&str, (Option<Vec<String>>, Option<Vec<String>>), VerboseError<&str>> {
    preceded(
        sp,
        tuple((
            entity,
            alt((
                preceded(sp, alphanumerickey1),
                delimited(sp, alphanumerickey1, sp),
            )),
        )),
    )(input)
    .map(|(next_input, res)| {
        let options = inner_create_option(next_input);
        if let Ok((next, (option1, keys1))) = options {
            let options = inner_create_option(next);
            if let Ok((_, (_, keys2))) = options {
                if option1 == CreateOptions::UNIQUES {
                    (res.1, (Some(keys1), Some(keys2)))
                } else {
                    (res.1, (Some(keys2), Some(keys1)))
                }
            } else if option1 == CreateOptions::UNIQUES {
                (res.1, (Some(keys1), None))
            } else {
                (res.1, (None, Some(keys1)))
            }
        } else {
            (res.1, (None, None))
        }
    })
}

pub fn insert_content(
    input: &str,
) -> IResult<&str, (HashMap<String, Types>, Option<Uuid>), VerboseError<&str>> {
    preceded(
        sp,
        tuple((
            hashmap,
            preceded(sp, into),
            alt((
                preceded(sp, alphanumerickey1),
                delimited(sp, alphanumerickey1, sp),
            )),
        )),
    )(input)
    .map(|(next, res)| match inner_insert(next) {
        Err(_) => (res.2, (res.0, None)),
        Ok((_, id)) => (res.2, (res.0, Some(id))),
    })
}

pub fn update_content(
    input: &str,
) -> IResult<&str, (&str, HashMap<String, Types>, Uuid), VerboseError<&str>> {
    preceded(
        sp,
        tuple((
            alt((
                preceded(sp, alphanumerickey1),
                delimited(sp, alphanumerickey1, sp),
            )),
            alt((preceded(sp, keyword_set), preceded(sp, content))),
            preceded(sp, hashmap),
            preceded(sp, into),
            preceded(sp, uuid_parser),
        )),
    )(input)
    .map(|(_, res)| (res.0, (res.1, res.2, res.4)))
}

pub fn evict_content(input: &str) -> IResult<&str, Option<&str>, VerboseError<&str>> {
    preceded(sp, tuple((preceded(sp, alphanumericboth1),)))(input).map(|(next, res)| {
        match tuple((preceded(sp, from), preceded(sp, alphanumerickey1)))(next) {
            Ok(inner) => (res.0, Some(inner.1 .1)),
            Err(_) => (res.0, None),
        }
    })
}

pub fn delete_content(input: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    preceded(
        sp,
        tuple((
            preceded(sp, uuid_parser),
            preceded(sp, from),
            preceded(sp, alphanumerickey1),
        )),
    )(input)
    .map(|(_, res)| (res.2, res.0))
}

fn inner_create_option(
    input: &str,
) -> IResult<&str, (CreateOptions, Vec<String>), VerboseError<&str>> {
    preceded(sp, tuple((create_options, sp, set)))(input).map(|(next, v)| (next, (v.0, v.2)))
}

fn inner_insert(input: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    preceded(sp, tuple((with, sp, uuid_parser)))(input).map(|(next, v)| (next, v.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_content_empty() {
        assert_eq!(
            Ok(("hello_world", (None, None))),
            create_content("ENTITY hello_world")
        );
        assert_eq!(
            Ok(("hello_world", (None, None))),
            create_content("ENTITY hello_world ")
        );
    }

    #[test]
    fn create_content_uniques() {
        assert_eq!(
            Ok((
                "hello_world",
                (Some(vec!["hello".to_string(), "world".to_string()]), None)
            )),
            create_content("ENTITY hello_world UNIQUES #{hello, world}")
        );
    }

    #[test]
    fn create_content_encrypt() {
        assert_eq!(
            Ok((
                "hello_world",
                (None, Some(vec!["hello".to_string(), "world".to_string()]))
            )),
            create_content("ENTITY hello_world Encrypt #{hello, world}")
        );
    }

    #[test]
    fn create_content_both_options() {
        assert_eq!(
            Ok((
                "hello_world",
                (
                    Some(vec!["hello".to_string(), "world".to_string()]),
                    Some(vec!["hello2".to_string(), "world2".to_string()])
                )
            )),
            create_content("ENTITY hello_world UNIQUES #{hello, world} Encrypt #{hello2, world2}")
        );
    }
}
