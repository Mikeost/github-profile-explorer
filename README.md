# GitHub Profile Explorer

<p align="center">
  <img src="https://github.com/Mikeost/github-profile-explorer/assets/68986785/b3b28a6b-a63b-48ed-a6b4-61b3eaed2d5e" height="250">
</p>

## Description

GitHub Profile Explorer is a command-line interface tool that allows you to retrieve public information about GitHub users, organizations, and repositories directly from your terminal. It provides features such as fetching user details, listing repositories, and more, making it convenient for users who prefer a command-line environment for GitHub exploration.

## Features

- Fetch public details about GitHub users and organizations.
- List repositories and their details.
- Perform various actions through a simple command-line interface.

## Usage

To get started, clone the repository and run the following commands:

```bash
git clone https://github.com/Mikeost/github-profile-explorer.git
cd github-profile-explorer
cargo build --release
./target/release/github-profile-explorer <type> <name>
```

Where `type` is one of the following:
- `org`: Retrieve information about a GitHub organization.
- `user`: Retrieve information about a GitHub user.

Replace `name` with the corresponding GitHub organization/user name you want to explore.

### For example

```bash
./target/release/github-profile-explorer org MikeostCorp
```

```
Repo name: Gradify
Repo description: Information system for accounting for the work of an education institution
Repo topics: college database-management-system information-system learning 
Repo last update: 2023-12-11T19:46:06Z
Repo language: C++
Repo count of stars: 4
Repo count of forks: 1
=======================================================
Repo name: Math-Algorithms
Repo description: Math library with algorithms for c++
Repo topics: algorithms cpp math mathematics 
Repo last update: 2023-09-06T04:40:24Z
Repo language: C++
Repo count of stars: 6
Repo count of forks: 0
=======================================================
Repo name: ipCalculator
Repo description: ipCalculator
Repo topics: N/A
Repo last update: 2022-09-20T06:49:45Z
Repo language: C++
Repo count of stars: 1
Repo count of forks: 0
=======================================================
```

```bash
./target/release/github-profile-explorer user Mikeost
```

```
Repo name: github-profile-explorer
Repo description: CLI utility for exploring GitHub profiles and repositories.
Repo topics: explorer github statistics 
Repo last update: 2024-01-27T22:14:09Z
Repo language: Rust
Repo count of stars: 0
Repo count of forks: 0
=======================================================
Repo name: MatPlotLibPython
Repo description: Data visualization using the Matplotlib
Repo topics: charts matplotlib python statistics 
Repo last update: 2023-02-10T15:23:41Z
Repo language: Python
Repo count of stars: 3
Repo count of forks: 1
=======================================================
Repo name: Sudoku-Solver
Repo description: Simple sudoku solver
Repo topics: backtracking-algorithm cpp sudoku-solver 
Repo last update: 2023-06-28T12:36:00Z
Repo language: C++
Repo count of stars: 0
Repo count of forks: 0
=======================================================
...
```
