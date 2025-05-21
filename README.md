# GitHub Activity Tracker

A simple command-line interface (CLI) tool written in Rust to fetch and display recent activity of a GitHub user using
the GitHub API.

## Features

- Fetches the latest public activity of a GitHub user.
- Displays human-readable summaries such as:
    - Pushed commits
    - Opened issues
    - Starred repositories
- Gracefully handles common errors (e.g. user not found, API limit exceeded).
- Lightweight and minimal — no HTTP client libraries used.

## Usage

```sh
# Build the project
cargo build --release

# Run the CLI
./github-activity <github-username>
```

## Example

```sh
$ github-activity noobMushroom

- Pushed 3 commits to noobMushroom/dotfiles
- Starred rust-lang/book
- Opened an issue in rust-lang/rust
 ```

## Installation

```shell
git clone https://github.com/your-username/github-activity.git
cd github-activity
cargo build --release
```