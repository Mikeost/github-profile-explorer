use reqwest::{blocking::{Client, ClientBuilder, Response}};
use serde::Deserialize;

pub struct Config {
    pub query: String,
    pub name: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let name = args[2].clone();

        Ok(Config {
            query,
            name
        })
    }
}

#[derive(Deserialize)]
pub struct OrganizationInfo {
    #[serde(rename = "name")]
    repo_name: String,

    #[serde(rename = "description")]
    repo_description: String,

    #[serde(rename = "updated_at")]
    repo_last_update: String,

    #[serde(rename = "language")]
    repo_language: String
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION")
    );

    let http_client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    if config.query == "org" {
        list_organization_repositories(http_client, &config.name);
    } else if config.query == "user" {
        // list_user_repositories();
    }

    Ok(())
}

pub fn list_organization_repositories(http_client: Client, name: &str) {
    let url = format!("https://api.github.com/orgs/{}/repos", name); 
    let http_result = http_client.get(&url).send();
    
    match http_result {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => {
                        match serde_json::from_str::<Vec<OrganizationInfo>>(&text) {
                            Ok(organization_info) => info_output(organization_info),
                            Err(_) => println!("Error deserializing JSON."),
                        }
                    },
                    Err(_) => println!("Error reading response text.")
                }
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                println!("This organization was not found.")
            }
        }
        Err(err) => {
            println!("{:#?}", err.status());
        }
    }
}

pub fn info_output(organization_info: Vec<OrganizationInfo>) {
    for info in organization_info {
        println!("Repo name: {}", info.repo_name);
        println!("Repo description: {}", info.repo_description);
        println!("Repo last update: {}", info.repo_last_update);
        println!("Repo language: {}", info.repo_language);
        println!("=======================================================");
    }
}