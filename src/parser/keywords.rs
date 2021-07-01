use crate::model::{CreateOptions, Operation};
use nom::{
    branch::alt,
    bytes::streaming::tag_no_case,
    error::{context, VerboseError},
    Err as NomErr, IResult,
};

const ENTITY: &str = "ENTITY";
const INTO: &str = "INTO";
const WITH: &str = "WITH";
pub const SET: &str = "SET";
pub const CONTENT: &str = "CONTENT";

pub fn operation(input: &str) -> IResult<&str, Operation, VerboseError<&str>> {
    context(
        "operation",
        alt((
            tag_no_case("CREATE"),
            tag_no_case("INSERT"),
            tag_no_case("UPDATE"),
            tag_no_case("DELETE"),
            tag_no_case("MATCH"),
            tag_no_case("EVICT"),
            tag_no_case("SELECT"),
            tag_no_case("CHECK"),
            tag_no_case("RELATION"),
            tag_no_case("JOIN"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn entity(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("entity", tag_no_case(ENTITY))(input).and_then(|(next_input, res)| match res {
        ENTITY => Ok((next_input, ENTITY)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

pub fn into(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("into", tag_no_case(INTO))(input).and_then(|(next_input, res)| match res {
        INTO => Ok((next_input, INTO)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

pub fn with(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("with", tag_no_case(WITH))(input).and_then(|(next_input, res)| match res {
        WITH => Ok((next_input, WITH)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

pub fn set(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("set", tag_no_case(SET))(input).and_then(|(next_input, res)| match res {
        SET => Ok((next_input, SET)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

pub fn content(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("content", tag_no_case(CONTENT))(input).and_then(|(next_input, res)| match res {
        CONTENT => Ok((next_input, CONTENT)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

pub fn create_options(input: &str) -> IResult<&str, CreateOptions, VerboseError<&str>> {
    context(
        "create_options",
        alt((tag_no_case("UNIQUES"), tag_no_case("ENCRYPT"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

#[cfg(test)]
mod operation_test {
    use nom::error::{ErrorKind, VerboseErrorKind};

    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            operation("Create and some random string after"),
            Ok((" and some random string after", Operation::CREATE))
        );

        assert_eq!(
            operation("Error and some random string after"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Nom(ErrorKind::Tag)
                    ),
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Nom(ErrorKind::Alt)
                    ),
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Context("operation")
                    )
                ]
            }))
        );
    }

    #[test]
    fn insert() {
        assert_eq!(
            operation("INSERT and some random string after"),
            Ok((" and some random string after", Operation::INSERT))
        )
    }

    #[test]
    fn update() {
        assert_eq!(
            operation("UPDATE and some random string after"),
            Ok((" and some random string after", Operation::UPDATE))
        )
    }

    #[test]
    fn delete() {
        assert_eq!(
            operation("DELETE and some random string after"),
            Ok((" and some random string after", Operation::DELETE))
        )
    }

    #[test]
    fn match_test() {
        assert_eq!(
            operation("MATCH and some random string after"),
            Ok((" and some random string after", Operation::MATCH_UPDATE))
        )
    }

    #[test]
    fn evict() {
        assert_eq!(
            operation("EVICT and some random string after"),
            Ok((" and some random string after", Operation::EVICT))
        )
    }

    #[test]
    fn select() {
        assert_eq!(
            operation("SELECT and some random string after"),
            Ok((" and some random string after", Operation::SELECT))
        )
    }

    #[test]
    fn check() {
        assert_eq!(
            operation("CHECK and some random string after"),
            Ok((" and some random string after", Operation::CHECK))
        )
    }

    #[test]
    fn relation() {
        assert_eq!(
            operation("RELATION and some random string after"),
            Ok((" and some random string after", Operation::RELATION))
        )
    }

    #[test]
    fn join() {
        assert_eq!(
            operation("JOIN and some random string after"),
            Ok((" and some random string after", Operation::JOIN))
        )
    }

    #[test]
    fn entity_test() {
        assert_eq!(
            entity("ENTITY and some random string after"),
            Ok((" and some random string after", ENTITY))
        );

        assert_eq!(
            entity("Error and some random string after"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Nom(ErrorKind::Tag)
                    ),
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Context("entity")
                    )
                ]
            }))
        );
    }

    #[test]
    fn create_options_test() {
        assert_eq!(
            create_options("EnCryPt and some random string after"),
            Ok((" and some random string after", CreateOptions::ENCRYPT))
        );
        assert_eq!(
            create_options("UniQUES and some random string after"),
            Ok((" and some random string after", CreateOptions::UNIQUES))
        );
        assert_eq!(
            create_options("Error and some random string after"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Nom(ErrorKind::Tag)
                    ),
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Nom(ErrorKind::Alt)
                    ),
                    (
                        "Error and some random string after",
                        VerboseErrorKind::Context("create_options")
                    )
                ]
            }))
        );
    }
}
