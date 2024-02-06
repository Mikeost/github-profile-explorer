use clap::Parser;
use github_profile_explorer::Config;
use std::process;

mod cli;
use github_profile_explorer::ProfileInfo;

fn main() {
    let args: Config = Config::parse();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}.", err);
        process::exit(1);
    });

    if let Err(err) = github_profile_explorer::run(config) {
        eprintln!("Application error: {}.", err);
        process::exit(1);
    }
}
