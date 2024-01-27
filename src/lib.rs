use reqwest::blocking::Client;
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
    repo_name: Option<String>,

    #[serde(rename = "description")]
    repo_description: Option<String>,

    #[serde(rename = "updated_at")]
    repo_last_update: Option<String>,

    #[serde(rename = "language")]
    repo_language: Option<String>
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
                            Err(err) => println!("Error deserializing JSON.: {}", err),
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
        if let Some(repo_name) = info.repo_name {
            println!("Repo name: {}", repo_name);
        } else {
            println!("Repo name: N/A");
        }

        if let Some(repo_description) = info.repo_description {
            println!("Repo description: {}", repo_description);
        } else {
            println!("Repo description: N/A");
        }

        if let Some(repo_last_update) = info.repo_last_update {
            println!("Repo last update: {}", repo_last_update);
        } else {
            println!("Repo last update: N/A");
        }

        if let Some(repo_language) = info.repo_language {
            println!("Repo language: {}", repo_language);
        } else {
            println!("Repo language: N/A");
        }

        println!("=======================================================");
    }
}