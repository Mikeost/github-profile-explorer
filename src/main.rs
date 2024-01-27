use std::{env, process};
use github_profile_explorer::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });

    if let Err(err) = github_profile_explorer::run(config) {
        eprintln!("Application error: {err}.");
        process::exit(1);
    }
}