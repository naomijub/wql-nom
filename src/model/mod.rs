pub mod error;
pub mod types;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::types::Types;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Wql {
    CreateEntity {
        name: String,
        uniques: Option<Vec<String>>,
        encrypts: Option<Vec<String>>,
    },
    Insert {
        entity: String,
        content: HashMap<String, Types>,
        id: Option<Uuid>,
    },
    UpdateSet {
        name: String,
        id: Uuid,
        content: HashMap<String, Types>,
    },
    UpdateContent {
        name: String,
        id: Uuid,
        content: HashMap<String, Types>,
    },
    Evict {
        entity: String,
        id: Option<Uuid>,
    },
    Delete {
        entity: String,
        id: Uuid,
        // at: DateTime<Utc>
    },
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    #[allow(non_camel_case_types)]
    CREATE,
    #[allow(non_camel_case_types)]
    INSERT,
    #[allow(non_camel_case_types)]
    UPDATE,
    #[allow(non_camel_case_types)]
    DELETE,
    #[allow(non_camel_case_types)]
    MATCH_UPDATE,
    #[allow(non_camel_case_types)]
    EVICT,
    #[allow(non_camel_case_types)]
    SELECT,
    #[allow(non_camel_case_types)]
    CHECK,
    #[allow(non_camel_case_types)]
    RELATION,
    #[allow(non_camel_case_types)]
    JOIN,
}

#[derive(Debug, PartialEq)]
pub enum CreateOptions {
    #[allow(non_camel_case_types)]
    UNIQUES,
    #[allow(non_camel_case_types)]
    ENCRYPT,
}

impl From<&str> for Operation {
    fn from(i: &str) -> Self {
        match i.to_uppercase().as_str() {
            "CREATE" => Operation::CREATE,
            "INSERT" => Operation::INSERT,
            "UPDATE" => Operation::UPDATE,
            "DELETE" => Operation::DELETE,
            "MATCH" => Operation::MATCH_UPDATE,
            "EVICT" => Operation::EVICT,
            "SELECT" => Operation::SELECT,
            "CHECK" => Operation::CHECK,
            "RELATION" => Operation::RELATION,
            "JOIN" => Operation::JOIN,
            _ => unimplemented!("no other operation supported"),
        }
    }
}

impl From<&str> for CreateOptions {
    fn from(i: &str) -> Self {
        match i.to_uppercase().as_str() {
            "UNIQUES" => CreateOptions::UNIQUES,
            "ENCRYPT" => CreateOptions::ENCRYPT,
            _ => unimplemented!("no other create option supported"),
        }
    }
}

// impl std::str::FromStr for Wql {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let tokens = s.trim_start();

//         Err("".to_string())
//     }
// }
