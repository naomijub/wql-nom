use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, hash::Hash};
use std::{collections::HashMap};
use uuid::Uuid;

use crate::logic::integer_decode;

#[allow(clippy::derive_hash_xor_eq)] // for now
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Types {
    Char(char),
    Integer(isize),
    String(String),
    Uuid(Uuid),
    Float(f64),
    Boolean(bool),
    Vector(Vec<Types>),
    Map(HashMap<String, Types>),
    Hash(String),
    Precise(String),
    DateTime(DateTime<Utc>),
    Nil(Nil),
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
            Types::Map(t) => t.into_iter().fold((), |acc, (k, v)| {
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

impl From<isize> for Types {
    fn from(i: isize) -> Self {
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
