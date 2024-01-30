use github_profile_explorer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_all_args() {
        let args = vec![
            String::from("program_name"),
            String::from("org"),
            String::from("MikeostCorp"),
        ];

        let result = github_profile_explorer::Config::build(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn config_build_not_all_args() {
        let args = vec![
            String::from("program_name"),
            String::from("org"),
        ];

        let result = github_profile_explorer::Config::build(&args);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_request_type() {
        let config = github_profile_explorer::Config {
            request: String::from("invalid_type"),
            name: String::from("test_name"),
        };

        let result = github_profile_explorer::run(config);

        assert_eq!(
            result.err().unwrap().to_string(),
            "The request type is not valid. Choose either 'org' or 'user'"
        );
    }
}