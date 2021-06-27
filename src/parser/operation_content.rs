use nom::{
    branch::alt,
    error::VerboseError,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    model::CreateOptions,
    parser::{
        keywords::{create_options, entity},
        types::set,
    },
};

use crate::parser::types::sp;

use super::types::alphanumerickey1;

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
            } else {
                if option1 == CreateOptions::UNIQUES {
                    (res.1, (Some(keys1), None))
                } else {
                    (res.1, (None, Some(keys1)))
                }
            }
        } else {
            (res.1, (None, None))
        }
    })
}

fn inner_create_option(
    input: &str,
) -> IResult<&str, (CreateOptions, Vec<String>), VerboseError<&str>> {
    preceded(sp, tuple((create_options, sp, set)))(input).map(|(next, v)| (next, (v.0, v.2)))
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
