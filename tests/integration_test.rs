use github_profile_explorer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_request_type() {
        let config = github_profile_explorer::Config {
            request: String::from("invalid_type"),
            name: String::from("test_name"),
            sort: String::from("pushed"),
            direction: String::from("desk"),
        };

        let result = github_profile_explorer::run(config);

        assert_eq!(
            result.err().unwrap().to_string(),
            "The request type is not valid. Choose either 'org' or 'user'"
        );
    }
}
