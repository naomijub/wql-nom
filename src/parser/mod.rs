use nom::error::context;

use crate::model::error::WqlError;
use crate::model::Operation;
use crate::parser::operation_content::insert_content;
use crate::parser::operation_content::update_content;
use crate::{
    model::Wql,
    parser::{
        keywords::{operation, CONTENT, SET},
        operation_content::create_content,
    },
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
                    "Couldn't parse input `{}` as CREATE.\n Parsing error: {:?}",
                    input, e
                ))),
            },
            Operation::INSERT => match insert_content(next) {
                Ok((entity, (content, id))) => Ok(Wql::Insert {
                    entity: entity.to_string(),
                    content,
                    id,
                }),
                Err(e) => Err(WqlError::Plain(format!(
                    "Couldn't parse input `{}` as INSERT.\n Parsing error: {:?}",
                    input, e
                ))),
            },
            Operation::UPDATE => match update_content(next) {
                Ok((entity, (update_type, content, id))) => match update_type {
                    CONTENT => Ok(Wql::UpdateContent {
                        name: entity.to_string(),
                        id,
                        content,
                    }),
                    SET => Ok(Wql::UpdateSet {
                        name: entity.to_string(),
                        id,
                        content,
                    }),
                    _ => Err(WqlError::Plain(format!(
                        "Couldn't parse input `{}` as UPDATE.\n Parsing error: {:?}",
                        input, "UPDATE type not found"
                    ))),
                },
                Err(e) => Err(WqlError::Plain(format!(
                    "Couldn't parse input `{}` as UPDATE.\n Parsing error: {:?}",
                    input, e
                ))),
            },
            _ => unimplemented!(),
        })
        .map_err(|e| WqlError::Parse(e))?
}
