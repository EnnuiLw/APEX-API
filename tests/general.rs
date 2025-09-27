#[cfg(test)]
mod tests {
    use apex_api::Client;

    #[test]
    fn dotenvy_works() {
        // the path of args is only works in case that .env on root.
        // like:
        // ├── Cargo.toml
        // ├── .env
        // ├── src
        // │   └── src.rs
        //
        let client = Client::from_env_path(".env");
        assert!(client.is_ok());

        // same.
        let client = Client::from_env();
        assert!(client.is_ok());
    }

    #[test]
    fn to_string_works() {
        let character = apex_api::types::Character::Alter;
        assert_eq!(character.to_string(), "Alter".to_string());
    }
}
