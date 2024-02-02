# GitHub Profile Explorer

<p align="center">
  <a href="#github-profile-explorer"><img src="https://github.com/Mikeost/github-profile-explorer/assets/68986785/b3b28a6b-a63b-48ed-a6b4-61b3eaed2d5e" height="250"></a>
</p>

<p align="center">
  <a href="https://github.com/Mikeost/github-profile-explorer/actions/workflows/rust.yml"><img src="https://img.shields.io/github/actions/workflow/status/mikeost/github-profile-explorer/rust.yml?label=build%20%26%20tests" alt="Build Status"></a>
  <a href="https://github.com/Mikeost/github-profile-explorer?tab=MIT-1-ov-file#readme"><img src="https://img.shields.io/github/license/mikeost/github-profile-explorer" alt="License"></a>
  <img src="https://img.shields.io/crates/msrv/reqwest/0.11.23" alt="Minimum suppported rust version">
  <a href="https://app.codacy.com/gh/Mikeost/github-profile-explorer/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade"><img src="https://app.codacy.com/project/badge/Grade/26fe53ed9fbe4b3ebc1a5e9fda20a1a8" alt="Codacy Badge"></a>
  <img src="https://tokei.rs/b1/github/mikeost/github-profile-explorer?category=code&style=flat" alt="Lines of Code">
  <img src="https://img.shields.io/github/languages/code-size/mikeost/github-profile-explorer.svg" alt="Code size"/>
  <img src="https://img.shields.io/github/repo-size/mikeost/github-profile-explorer.svg" alt="GitHub repo size"/>
</p>

## Description

GitHub Profile Explorer is a command-line interface tool that allows you to retrieve public information about GitHub users, organizations, and repositories directly from your terminal. It provides features such as fetching user details, listing repositories, and more, making it convenient for users who prefer a command-line environment for GitHub exploration.

## Features

- Fetch public details about GitHub users and organizations.
- List repositories and their details.
- Perform various actions through a simple command-line interface.

## Usage

To get started, run the following commands:

```bash
git clone https://github.com/Mikeost/github-profile-explorer.git
cd github-profile-explorer
cargo install --path .
gpe <REQUEST> <NAME> [OPTIONS]
```

| Argument | Command | Description |
| ------ | ------- | ----------- |
| Request | `org`, `user` | Retrieve information about a GitHub organization/user |
| Name | `name` | The corresponding GitHub organization/user name you want to explore |

<hr>

| Option | Command | Description |
| ------ | ------- | ----------- |
| Sort | `-s, --sort <SORT>` | The property to sort the results by (created/updated/pushed/full_name) [default: created] |
| Direction | `-d, --direction <DIRECTION>` | The order to sort by (asc/desc) [default: desc] |
| Count Per Page | `-c, --count-per-page <COUNT_PER_PAGE>` | Number of results per page [default: 30, max: 100] |
| Page Number | `-p, --page-number <PAGE_NUMBER>` | Number of page [default: 1] |
| Help | `-h, --help` | Print help |
| Verion | `-V, --version` | Print version |


## Examples

### Example of retrieve information about a GitHub organization

<details>
  <summary>Click to expand!</summary>

```
$ gpe org MikeostCorp -s full_name -d asc

--------------------------------------------------------
        Repo name: Gradify
Description: Information system for accounting for the work of an education institution
Topics: college, database-management-system, information-system, learning
Last update: 2023-12-11T19:46:06Z
Language: C++
Count of stars: 4
Count of forks: 1
--------------------------------------------------------
--------------------------------------------------------
        Repo name: ipCalculator
Description: ipCalculator
Topics: N/A
Last update: 2022-09-20T06:49:45Z
Language: C++
Count of stars: 1
Count of forks: 0
--------------------------------------------------------
--------------------------------------------------------
        Repo name: Math-Algorithms
Description: Math library with algorithms for c++
Topics: algorithms, cpp, math, mathematics
Last update: 2023-09-06T04:40:24Z
Language: C++
Count of stars: 6
Count of forks: 0
--------------------------------------------------------
```

</details>

### Example of retrieve information about a GitHub user

<details>
  <summary>Click to expand!</summary>

```
$ gpe user Mikeost -s pushed -d desc -c 5

--------------------------------------------------------
        Repo name: github-profile-explorer
Description: CLI utility for exploring GitHub profiles and repositories.
Topics: explorer, github, statistics
Last update: 2024-02-01T13:52:50Z
Language: Rust
Count of stars: 0
Count of forks: 0
--------------------------------------------------------
--------------------------------------------------------
        Repo name: mikeost.github.io
Description: website
Topics: website
Last update: 2023-12-31T21:14:59Z
Language: HTML
Count of stars: 0
Count of forks: 0
--------------------------------------------------------
--------------------------------------------------------
        Repo name: developer-roadmap
Description: Interactive roadmaps, guides and other educational content to help developers grow in their careers.
Topics: N/A
Last update: 2023-08-29T17:35:38Z
Language: TypeScript
Count of stars: 0
Count of forks: 0
--------------------------------------------------------
--------------------------------------------------------
        Repo name: arduino-projects
Description: arduino projects
Topics: arduino, cpp
Last update: 2023-07-26T16:47:24Z
Language: C++
Count of stars: 0
Count of forks: 0
--------------------------------------------------------
--------------------------------------------------------
        Repo name: Sudoku-Solver
Description: Simple sudoku solver
Topics: backtracking-algorithm, cpp, sudoku-solver
Last update: 2023-06-28T12:36:00Z
Language: C++
Count of stars: 0
Count of forks: 0
--------------------------------------------------------
```

</details>