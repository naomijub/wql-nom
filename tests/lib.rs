#[cfg(test)]
mod tests {
    use wql_nom::{parse_wql, Wql};

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
}
