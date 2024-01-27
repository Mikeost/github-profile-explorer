# GitHub Profile Explorer

CLI utility for exploring GitHub profiles and repositories.

## Description

GitHub Explorer CLI is a command-line interface tool that allows you to retrieve information about GitHub users, organizations, and repositories directly from your terminal. It provides features such as fetching user details, listing repositories, and more, making it convenient for users who prefer a command-line environment for GitHub exploration.

## Features

- Fetch details about GitHub users and organizations.
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

Replace `name` with the corresponding GitHub organization name you want to explore.

### For example

```bash
./target/release/github-profile-explorer org MikeostCorp
```

```
Repo name: Math-Algorithms
Repo description: Math library with algorithms for C++
Repo last update: 2023-09-06T21:18:02Z
Repo language: C++
=======================================================
Repo name: ipCalculator
Repo description: ipCalculator
Repo last update: 2022-08-31T06:47:26Z
Repo language: C++
=======================================================
Repo name: Gradify
Repo description: Information system for accounting for the work of an education institution
Repo last update: 2024-01-06T13:52:39Z
Repo language: C++
=======================================================
```