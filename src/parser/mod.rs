use nom::error::{context, VerboseError};

use crate::model::Operation;
use crate::{
    model::Wql,
    parser::{keywords::operation, operation_content::create_content},
};

pub mod keywords;
pub mod operation_content;
pub mod types;

pub fn parse_wql(input: &str) -> Result<Wql, WqlError> {
    context("parse_wql", operation)(input)
        .map(|(next, op)| match op {
            Operation::CREATE => match create_content(next) {
                Ok((name, (uniques, encrypts))) => Ok(Wql::CreateEntity {
                    name: name.to_owned(),
                    uniques,
                    encrypts,
                }),
                Err(e) => Err(WqlError::Plain(format!(
                    "Couldn't parse input {}.\n Parsing error: {:?}",
                    input, e
                ))),
            },
            _ => unimplemented!(),
        })
        .map_err(|e| WqlError::Parse(e))?
}

#[derive(Debug, PartialEq)]
pub enum WqlError<'b> {
    Plain(String),
    Parse(nom::Err<VerboseError<&'b str>>),
}

impl<'b> From<nom::Err<VerboseError<&'b str>>> for WqlError<'b> {
    fn from(e: nom::Err<VerboseError<&'b str>>) -> Self {
        WqlError::Parse(e)
    }
}
