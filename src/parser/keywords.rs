use crate::model::Operation;
use nom::{
    Err as NomErr,
    branch::alt,
    bytes::streaming::tag_no_case,
    error::{context, VerboseError},
    IResult,
};

const ENTITY: &str = "ENTITY";

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
    context(
        "entity",
        tag_no_case("ENTITY"),

    )(input)
    .and_then(|(next_input, res)| match res {
        ENTITY => Ok((next_input, ENTITY)),
        _ => Err(NomErr::Error(VerboseError { errors: vec![] })),
    })
}

#[cfg(test)]
mod operation_test {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            operation("Create and some random string after"),
            Ok((" and some random string after", Operation::CREATE))
        )
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
        )
    }
}
