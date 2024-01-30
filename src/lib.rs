use reqwest::blocking::Client;
use serde::Deserialize;

pub struct Config {
    pub request: String,
    pub name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let request = args[1].clone();
        let name = args[2].clone();

        Ok(Config {
            request,
            name
        })
    }
}

#[derive(Deserialize)]
pub struct ProfileInfo {
    #[serde(rename = "name")]
    repo_name: Option<String>,

    #[serde(rename = "description")]
    repo_description: Option<String>,

    topics: Vec<String>,

    #[serde(rename = "pushed_at")]
    repo_last_update: Option<String>,

    #[serde(rename = "language")]
    repo_language: Option<String>,

    stargazers_count: i32,

    forks_count: i32,
}

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    match &config.request[..] {
        "org" => list_organization_repositories(http_client, &config.name)?,
        "user" => list_user_repositories(http_client, &config.name)?,
        _ => Err("The request type is not valid. Choose either 'org' or 'user'")?,
    }

    Ok(())
}

pub fn list_organization_repositories(http_client: Client, name: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://api.github.com/orgs/{}/repos", name); 
    let http_result = http_client.get(&url).send();

    handle_http_response(http_result);

    Ok(())
}

pub fn list_user_repositories(http_client: Client, name: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://api.github.com/users/{}/repos", name); 
    let http_result = http_client.get(&url).send();

    handle_http_response(http_result);

    Ok(())
}

pub fn handle_http_response(http_result: Result<reqwest::blocking::Response, reqwest::Error>) {
    match http_result {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => match serde_json::from_str::<Vec<ProfileInfo>>(&text) {
                            Ok(profile_info) => info_output(profile_info),
                            Err(err) => println!("Error deserializing JSON.: {}", err),
                    },
                    Err(_) => println!("Error reading response text.")
                }
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                println!("This profile was not found.")
            }
        }
        Err(err) => println!("{:#?}", err.status()),
    }
}

pub fn info_output(profile_info: Vec<ProfileInfo>) {
    for info in profile_info {
        print_repo_info("Repo name", &info.repo_name);

        print_repo_info("Repo description", &info.repo_description);

        if info.topics.len() == 0 {
            println!("Repo topics: N/A");
        } else {
            println!("Repo topics: {}", info.topics.join(", "));
        }

        print_repo_info("Repo last update", &info.repo_last_update);

        print_repo_info("Repo language", &info.repo_language);

        println!("Repo count of stars: {}", info.stargazers_count);

        println!("Repo count of forks: {}", info.forks_count);

        println!("=======================================================");
    }
}

fn print_repo_info(label: &str, value: &Option<String>) {
    match value {
        Some(val) => println!("{}: {}", label, val),
        None => println!("{}: N/A", label)
    }
}