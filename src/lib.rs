use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;

mod cli;

#[derive(Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The types of profiles you want get (org/user)
    pub request: String,

    /// The profile name
    pub name: String,

    /// The property to sort the results by
    #[arg(short, long, default_value_t = String::from("created"), help = "The property to sort the results by (created/updated/pushed/full_name)")]
    pub sort: String,

    /// The order to sort by
    #[arg(short, long, default_value_t = String::from("desc"), help = "The order to sort by (asc/desc)")]
    pub direction: String,
}

impl Config {
    pub fn build(args: Config) -> Result<Config, &'static str> {
        Ok(args)
    }
}

#[derive(Deserialize)]
pub struct RepositoryInfo {
    pub name: Option<String>,

    pub description: Option<String>,

    pub topics: Vec<String>,

    #[serde(rename = "pushed_at")]
    pub last_update: Option<String>,

    pub language: Option<String>,

    pub stargazers_count: i32,

    pub forks_count: i32,
}
impl RepositoryInfo {
    pub fn ref_array(&self) -> [String; 7] {
        [
            self.name().unwrap_or_else(|| String::from("N/A")),
            self.description().unwrap_or_else(|| String::from("N/A")),
            self.topics().unwrap_or_else(|| String::from("N/A")),
            self.last_update().unwrap_or_else(|| String::from("N/A")),
            self.language().unwrap_or_else(|| String::from("N/A")),
            self.stargazers_count().to_string(),
            self.forks_count().to_string(),
        ]
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn topics(&self) -> Option<String> {
        if self.topics.is_empty() {
            None
        } else {
            Some(self.topics.join(", "))
        }
    }

    pub fn last_update(&self) -> Option<String> {
        self.last_update.clone()
    }

    pub fn language(&self) -> Option<String> {
        self.language.clone()
    }

    pub fn stargazers_count(&self) -> String {
        self.stargazers_count.to_string()
    }

    pub fn forks_count(&self) -> String {
        self.forks_count.to_string()
    }
}

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let http_client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    let mut page_number = 1;
    let count_per_page = 100;
    let mut repositories: Vec<RepositoryInfo> = Vec::new();

    let mut has_next_page = true;

    match &config.request[..] {
        "org" => {
            while has_next_page {
                let url = format!(
                    "https://api.github.com/orgs/{}/repos?sort={}&direction={}&per_page={}&page={}",
                    &config.name, &config.sort, &config.direction, count_per_page, page_number
                );
                let http_response = http_client.get(&url).send()?;
                has_next_page = handle_http_response(http_response, &mut repositories)?;
                page_number += 1;
            }
        }
        "user" => {
            while has_next_page {
                let url = format!(
                    "https://api.github.com/users/{}/repos?sort={}&direction={}&per_page={}&page={}",
                    &config.name, &config.sort, &config.direction, count_per_page, page_number
                );
                let http_response = http_client.get(&url).send()?;
                has_next_page = handle_http_response(http_response, &mut repositories)?;
                page_number += 1;
            }
        }
        _ => return Err("The request type is not valid. Choose either 'org' or 'user'".into()),
    }

    info_output(repositories);

    Ok(())
}

pub fn handle_http_response(
    http_response: reqwest::blocking::Response,
    repositories: &mut Vec<RepositoryInfo>,
) -> Result<bool, Box<dyn std::error::Error>> {
    match http_response.status() {
        reqwest::StatusCode::OK => {
            let text = http_response.text()?;
            if text == "[]" {
                return Ok(false);
            }
            let repos: Vec<RepositoryInfo> = serde_json::from_str(&text)?;
            repositories.extend(repos);
        }
        reqwest::StatusCode::NOT_FOUND => println!("This profile was not found."),
        reqwest::StatusCode::FORBIDDEN => {
            println!("The API request limit has been exceeded. Please wait for 60 minutes.")
        }
        _ => println!("Unhandled status code: {}", http_response.status()),
    }
    Ok(true)
}

pub fn info_output(repositories: Vec<RepositoryInfo>) {
    cli::run(repositories);
}
