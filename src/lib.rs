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

    /// The number of results per page (max 100)
    #[arg(
        short,
        long,
        default_value_t = 30,
        help = "Number of results per page, max: 100"
    )]
    pub count_per_page: u8,

    /// The page number of the results to fetch
    #[arg(short, long, default_value_t = 1, help = "Number of page")]
    pub page_number: u8,
}

impl Config {
    pub fn build(args: Config) -> Result<Config, &'static str> {
        Ok(args)
    }
}

#[derive(Deserialize)]
pub struct ProfileInfo {
    #[serde(rename = "name")]
    pub repo_name: Option<String>,

    #[serde(rename = "description")]
    pub repo_description: Option<String>,

    pub topics: Vec<String>,

    #[serde(rename = "pushed_at")]
    pub repo_last_update: Option<String>,

    #[serde(rename = "language")]
    pub repo_language: Option<String>,

    pub stargazers_count: i32,

    pub forks_count: i32,
}

impl ProfileInfo {
    pub fn ref_array(&self) -> [String; 7] {
        [
            self.repo_name().unwrap_or_else(|| String::from("N/A")),
            self.repo_description()
                .unwrap_or_else(|| String::from("N/A")),
            self.topics().unwrap_or_else(|| String::from("N/A")),
            self.repo_last_update()
                .unwrap_or_else(|| String::from("N/A")),
            self.repo_language().unwrap_or_else(|| String::from("N/A")),
            self.stargazers_count().to_string(),
            self.forks_count().to_string(),
        ]
    }

    pub fn repo_name(&self) -> Option<String> {
        self.repo_name.clone()
    }

    pub fn repo_description(&self) -> Option<String> {
        self.repo_description.clone()
    }

    pub fn topics(&self) -> Option<String> {
        if self.topics.is_empty() {
            None
        } else {
            Some(self.topics.join(", "))
        }
    }

    pub fn repo_last_update(&self) -> Option<String> {
        self.repo_last_update.clone()
    }

    pub fn repo_language(&self) -> Option<String> {
        self.repo_language.clone()
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

    match &config.request[..] {
        "org" => list_organization_repositories(
            &http_client,
            &config.name,
            &config.sort,
            &config.direction,
            config.count_per_page,
            config.page_number,
        )?,
        "user" => list_user_repositories(
            &http_client,
            &config.name,
            &config.sort,
            &config.direction,
            config.count_per_page,
            config.page_number,
        )?,
        _ => Err("The request type is not valid. Choose either 'org' or 'user'")?,
    }

    Ok(())
}

pub fn list_organization_repositories(
    http_client: &Client,
    name: &str,
    sort: &str,
    direction: &str,
    count_per_page: u8,
    page_number: u8,
) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.github.com/orgs/{}/repos?sort={}&direction={}&per_page={}&page={}",
        name, sort, direction, count_per_page, page_number
    );
    let http_result = http_client.get(&url).send();

    handle_http_response(http_result);

    Ok(())
}

pub fn list_user_repositories(
    http_client: &Client,
    name: &str,
    sort: &String,
    direction: &String,
    count_per_page: u8,
    page_number: u8,
) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.github.com/users/{}/repos?sort={}&direction={}&per_page={}&page={}",
        name, sort, direction, count_per_page, page_number
    );
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
                    Err(_) => println!("Error reading response text."),
                }
            } else if response.status() == reqwest::StatusCode::NOT_FOUND {
                println!("This profile was not found.")
            } else if response.status() == reqwest::StatusCode::FORBIDDEN {
                println!("The API request limit has been exceeded. Please wait for 60 minutes.")
            }
        }
        Err(err) => println!("{:#?}", err.status()),
    }
}

pub fn info_output(profile_info: Vec<ProfileInfo>) {
    cli::run(profile_info);
}
