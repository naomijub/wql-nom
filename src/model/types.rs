use chrono::{DateTime, Utc};
use nom::branch::alt;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::number::streaming::double;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{cmp::Ordering, hash::Hash};
use uuid::Uuid;

use crate::logic::integer_decode;
use crate::parser::types::{
    boolean, char_parse, datetime_parser, integer, precise_number_parser, sp, vector,
};
use crate::parser::types::{hashmap, string};
use crate::parser::types::{nil, uuid_parser};

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Types {
    Char(char),
    Integer(i128), // Review
    String(String),
    Uuid(Uuid),
    Float(f64),
    Boolean(bool),
    Vector(Vec<Types>),
    Map(HashMap<String, Types>),
    Hash(String), // not to be created like this
    Precise(String),
    DateTime(DateTime<Utc>),
    Nil(Nil),
}

pub fn wql_value(input: &str) -> IResult<&str, Types, VerboseError<&str>> {
    preceded(
        sp,
        alt((
            map(hashmap, Types::Map),
            map(uuid_parser, Types::Uuid),
            map(string, Types::String),
            map(boolean, Types::Boolean),
            map(nil, Types::Nil),
            map(char_parse, Types::Char),
            map(datetime_parser, Types::DateTime),
            map(precise_number_parser, Types::Precise),
            map(integer, Types::Integer),
            map(vector, Types::Vector),
            map(double, Types::Float),
        )),
    )(input)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nil;

impl Types {
    pub fn default_values(&self) -> Types {
        match self {
            Types::Char(_) => Types::Char(' '),
            Types::Integer(_) => Types::Integer(0),
            Types::String(_) => Types::String(String::new()),
            Types::Uuid(_) => Types::Uuid(Uuid::new_v4()),
            Types::Float(_) => Types::Float(0_f64),
            Types::Boolean(_) => Types::Boolean(false),
            Types::Vector(_) => Types::Vector(Vec::new()),
            Types::Map(_) => Types::Map(HashMap::new()),
            Types::Hash(_) => Types::Hash(String::new()),
            Types::Precise(_) => Types::Precise(String::from("0")),
            Types::DateTime(_) => Types::DateTime(Utc::now()),
            Types::Nil(Nil) => Types::Nil(Nil),
        }
    }

    pub fn to_hash(&self, cost: Option<u32>) -> Result<Types, String> {
        use bcrypt::{hash, DEFAULT_COST};
        let value = match self {
            Types::Char(c) => format!("{}", c),
            Types::Integer(i) => format!("{}", i),
            Types::String(s) => s.to_string(),
            Types::DateTime(date) => date.to_string(),
            Types::Uuid(id) => format!("{}", id),
            Types::Float(f) => format!("{:?}", integer_decode(f.to_owned())),
            Types::Boolean(b) => format!("{}", b),
            Types::Vector(vec) => format!("{:?}", vec),
            Types::Map(map) => format!("{:?}", map),
            Types::Precise(p) => p.to_string(),
            Types::Hash(_) => return Err(String::from("Hash cannot be hashed")),
            Types::Nil(_) => return Err(String::from("Nil cannot be hashed")),
        };
        match hash(&value, cost.map_or(DEFAULT_COST, |c| c)) {
            Ok(s) => Ok(Types::Hash(s)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn is_hash(&self) -> bool {
        matches!(self, Types::Hash(_))
    }
}

impl Eq for Types {}
impl PartialOrd for Types {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Types::Integer(a), Types::Integer(b)) => Some(a.cmp(b)),

            (Types::Float(a), Types::Float(b)) => Some(if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }),
            (Types::Integer(a), Types::Float(b)) => Some(if &(*a as f64) > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }),
            (Types::Float(a), Types::Integer(b)) => Some(if a > &(*b as f64) {
                Ordering::Greater
            } else {
                Ordering::Less
            }),
            (Types::Char(a), Types::Char(b)) => Some(a.cmp(b)),
            (Types::String(a), Types::String(b)) | (Types::Precise(a), Types::Precise(b)) => {
                Some(a.cmp(b))
            }
            (Types::Uuid(a), Types::Uuid(b)) => Some(a.cmp(b)),
            (Types::Boolean(a), Types::Boolean(b)) => Some(a.cmp(b)),
            (Types::Vector(a), Types::Vector(b)) => Some(a.len().cmp(&b.len())),
            _ => None,
        }
    }
}

// UNSAFE
#[allow(clippy::derive_hash_xor_eq)] // for now
impl Hash for Types {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Types::Char(t) => t.hash(state),
            Types::Integer(t) => t.hash(state),
            Types::String(t) => t.hash(state),
            Types::Uuid(t) => t.hash(state),
            Types::Float(t) => {
                let int_t = integer_decode(t.to_owned());
                int_t.hash(state)
            }
            Types::Boolean(t) => t.hash(state),
            Types::Vector(t) => t.hash(state),
            Types::Map(t) => t.iter().fold((), |acc, (k, v)| {
                k.hash(state);
                v.hash(state);
                acc
            }),
            Types::Hash(t) => t.hash(state),
            Types::Precise(t) => t.hash(state),
            Types::DateTime(t) => t.hash(state),
            Types::Nil(_) => "".hash(state),
        }
    }
}

impl From<char> for Types {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl From<i128> for Types {
    fn from(i: i128) -> Self {
        Self::Integer(i)
    }
}

impl From<String> for Types {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Types {
    fn from(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

impl From<Uuid> for Types {
    fn from(uuid: Uuid) -> Self {
        Self::Uuid(uuid)
    }
}

impl From<f64> for Types {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<bool> for Types {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<DateTime<Utc>> for Types {
    fn from(dt: DateTime<Utc>) -> Self {
        Self::DateTime(dt)
    }
}

impl From<Nil> for Types {
    fn from(n: Nil) -> Self {
        Self::Nil(n)
    }
}

impl From<Vec<Types>> for Types {
    fn from(v: Vec<Types>) -> Self {
        Self::Vector(v)
    }
}

impl From<HashMap<String, Types>> for Types {
    fn from(m: HashMap<String, Types>) -> Self {
        Self::Map(m)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn parse_map_with_uuid() {
        assert_eq!(
            Ok((
                "",
                Types::Map(
                    vec![(
                        "a".to_owned(),
                        Types::Uuid(
                            Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap()
                        )
                    )]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                )
            )),
            wql_value("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8}")
        );
    }

    #[test]
    fn parse_map_with_map_uuid() {
        assert_eq!(
            Ok((
                "",
                Types::Map(
                    vec![(
                        "a".to_owned(),
                        Types::Map(
                            vec![
                                (
                                    "b".to_owned(),
                                    Types::Uuid(
                                        Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8")
                                            .unwrap()
                                    )
                                ),
                                ("c".to_owned(), Types::Char('g'))
                            ]
                            .iter()
                            .cloned()
                            .collect::<HashMap<String, Types>>()
                        )
                    )]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                )
            )),
            wql_value("{a: {b: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8, c: 'g',} }")
        );
    }

    #[test]
    fn parse_map_with_str() {
        assert_eq!(
            Ok((
                "",
                Types::Map(
                    vec![
                        (
                            "a".to_owned(),
                            Types::Uuid(
                                Uuid::from_str("634f6c5b-476f-4cc0-97d0-c1c9468cf8d8").unwrap()
                            )
                        ),
                        (
                            "b".to_owned(),
                            Types::String("this is a string? yes!".to_owned())
                        )
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                )
            )),
            wql_value("{a: 634f6c5b-476f-4cc0-97d0-c1c9468cf8d8, b: \"this is a string? yes!\" }")
        );
    }

    #[test]
    fn float_vectors() {
        assert_eq!(
            Ok((
                "",
                Types::Vector(vec![
                    Types::Float(23.4),
                    Types::Float(345435.6),
                    Types::Float(-2813.4),
                    Types::Precise(String::from("7564")),
                    Types::Integer(74),
                ])
            )),
            wql_value("[23.4, 345435.6, -2813.4, 7564P, 74i]")
        )
    }
}
