use crate::utils::{get_request, get_status};
use rustls::pki_types::ServerName;
use rustls::{ClientConnection, StreamOwned};
use serde_json::Value;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("I/O error {0}")]
    Io(#[from] io::Error),

    #[error("Tls error {0}")]
    Tls(#[from] rustls::Error),

    #[error("GitHub user not found")]
    NotFound,

    #[error("Access forbidden (rate-limited or unauthorized)")]
    Forbidden,

    #[error("Github Service Not available")]
    ServerUnavailable,

    #[error("Not modified (no new events)")]
    NotModified,

    #[error("Unexpected HTTP status code: {0}")]
    UnexpectedStatus(u16),

    #[error("HTTP response parsing error: {0}")]
    HttpParse(String),

    #[error("Failed to parse JSON: {0}")]
    JsonParse(String),

    #[error("Unexpected Response: {0}")]
    UnexpectedResponse(String),
}

fn get_connection() -> Result<StreamOwned<ClientConnection, TcpStream>, GitHubError> {
    let base: ServerName = "api.github.com".try_into().expect("invalid server name");
    let socket = TcpStream::connect("api.github.com:443")?;
    let root_store =
        rustls::RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let rc_config = Arc::new(config);
    let client = rustls::ClientConnection::new(rc_config, base)?;
    Ok(StreamOwned::new(client, socket))
}

fn get_json(val: &str) -> Result<Vec<Value>, GitHubError> {
    let json = serde_json::from_str::<Value>(val)
        .map_err(|_| GitHubError::JsonParse(format!("failed to parse json: {}", val)))?;
    let events = json
        .as_array()
        .ok_or_else(|| GitHubError::JsonParse("expected an array of json".into()))?;

    Ok(events.to_vec())
}

fn print_events(events: &Vec<Value>) {
    for event in events {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let repo_name = event
            .get("repo")
            .and_then(|r| r.get("name"))
            .and_then(Value::as_str)
            .unwrap_or("unknown");

        let repo_url = event
            .get("repo")
            .and_then(|r| r.get("url"))
            .and_then(Value::as_str)
            .unwrap_or("unknown");

        println!("- {}\n on {}\n url {}", event_type, repo_name, repo_url);
    }
}

fn read_response(stream: &mut impl Read) -> Result<String, GitHubError> {
    let mut respones = String::new();
    stream.read_to_string(&mut respones)?;
    Ok(respones)
}

fn read_status_line(response: &str) -> Result<(), GitHubError> {
    let status_line = response.lines().next().unwrap_or("");
    get_status(status_line)
}

fn extract_body(response: &str) -> Result<&str, GitHubError> {
    response
        .split("\r\n\r\n")
        .nth(1)
        .ok_or(GitHubError::UnexpectedResponse("Missing body".into()))
}

pub fn handle_request(username: &str) -> Result<(), GitHubError> {
    let mut stream = get_connection().unwrap();
    let request = get_request(username);
    stream.write_all(request.as_bytes())?;
    let response = read_response(&mut stream)?;
    read_status_line(&response)?;
    let body = extract_body(&response)?;
    let json = get_json(&body)?;
    print_events(&json);
    Ok(())
}
