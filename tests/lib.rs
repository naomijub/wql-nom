#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};

    use uuid::Uuid;
    use wql_nom::{parse_wql, Types, Wql};

    #[test]
    fn create_content_empty() {
        assert_eq!(
            Ok(Wql::CreateEntity {
                name: "hello_world".to_owned(),
                uniques: None,
                encrypts: None
            }),
            parse_wql("create ENTITY hello_world")
        );
        assert_eq!(
            Ok(Wql::CreateEntity {
                name: "hello_world".to_owned(),
                uniques: None,
                encrypts: None
            }),
            parse_wql("create ENTITY hello_world ")
        );
    }

    #[test]
    fn create_content_uniques() {
        assert_eq!(
            Ok(Wql::CreateEntity {
                name: "hello_world".to_owned(),
                uniques: Some(vec!["hello".to_string(), "world".to_string()]),
                encrypts: None
            }),
            parse_wql("create ENTITY hello_world UNIQUES #{hello, world}")
        );
    }

    #[test]
    fn create_content_encrypt() {
        assert_eq!(
            Ok(Wql::CreateEntity {
                name: "hello_world".to_owned(),
                encrypts: Some(vec!["hello".to_string(), "world".to_string()]),
                uniques: None
            }),
            parse_wql("create ENTITY hello_world Encrypt #{hello, world}")
        );
    }

    #[test]
    fn create_content_both_options() {
        assert_eq!(
            Ok(Wql::CreateEntity {
                name: "hello_world".to_owned(),
                encrypts: Some(vec!["hello2".to_string(), "world2".to_string()]),
                uniques: Some(vec!["hello".to_string(), "world".to_string()]),
            }),
            parse_wql(
                "create ENTITY hello_world UNIQUES #{hello, world} Encrypt #{hello2, world2}"
            )
        );
    }

    #[test]
    fn simple_insert() {
        assert_eq!(
            Ok(Wql::Insert {
                entity: String::from("my_entity"),
                id: None,
                content: vec![
                    (String::from("hello"), Types::String("world".to_string())),
                    (String::from("age"), Types::Integer(30)),
                ]
                .iter()
                .cloned()
                .collect::<HashMap<String, Types>>()
            }),
            parse_wql("Insert {hello: \"world\", age: 30i} INTO my_entity")
        )
    }

    #[test]
    fn with_id_insert() {
        assert_eq!(
            Ok(
                Wql::Insert {
                    entity: String::from("my_entity"),
                    id: Some(Uuid::from_str("2e796540-ee72-40fd-b4a2-a2315d697d00").unwrap()),
                    content: vec![
                        (String::from("hello"), Types::String("world".to_string())),
                        (String::from("age"), Types::Integer(30)),
                    ].iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                }
            ),
            parse_wql("Insert {hello: \"world\", age: 30i} INTO my_entity WITH 2e796540-ee72-40fd-b4a2-a2315d697d00")
        )
    }

    #[test]
    fn update_set() {
        assert_eq!(
            Ok(
                Wql::UpdateSet {
                    name: String::from("this_entity"),
                    id: Uuid::from_str("2e796540-ee72-40fd-b4a2-a2315d697d00").unwrap(),
                    content: vec![
                        (String::from("hello"), Types::String("world".to_string())),
                        (String::from("age"), Types::Integer(30)),
                    ].iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                }
            ),
            parse_wql("UPDATE this_entity SET {hello: \"world\", age: 30i} INTO 2e796540-ee72-40fd-b4a2-a2315d697d00")
        )
    }

    #[test]
    fn update_content() {
        assert_eq!(
            Ok(
                Wql::UpdateContent {
                    name: String::from("this_entity"),
                    id: Uuid::from_str("2e796540-ee72-40fd-b4a2-a2315d697d00").unwrap(),
                    content: vec![
                        (String::from("hello"), Types::String("world".to_string())),
                        (String::from("age"), Types::Integer(30)),
                    ].iter()
                    .cloned()
                    .collect::<HashMap<String, Types>>()
                }
            ),
            parse_wql("UPDATE this_entity CONTENT {hello: \"world\", age: 30i} INTO 2e796540-ee72-40fd-b4a2-a2315d697d00")
        )
    }

    #[test]
    fn evict_id() {
        assert_eq!(
            Ok(Wql::Evict {
                entity: String::from("evict_entity"),
                id: Uuid::parse_str("2e796540-ee72-40fd-b4a2-a2315d697d00").ok()
            }),
            parse_wql("EVICT 2e796540-ee72-40fd-b4a2-a2315d697d00 FROM evict_entity")
        )
    }

    #[test]
    fn evict_entity() {
        assert_eq!(
            Ok(Wql::Evict {
                entity: String::from("evict_entity"),
                id: None
            }),
            parse_wql("EVICT evict_entity")
        )
    }
}
