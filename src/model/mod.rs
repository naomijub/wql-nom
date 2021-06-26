pub mod types;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Wql {
    CreateEntity {
        name: String,
        uniques: Option<Vec<String>>,
        encrupts: Option<Vec<String>>,
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

// impl std::str::FromStr for Wql {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let tokens = s.trim_start();

//         Err("".to_string())
//     }
// }
