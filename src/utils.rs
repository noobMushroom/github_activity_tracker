use crate::github::GitHubError;

pub fn get_status(http_res: &str) -> Result<(), GitHubError> {
    let status = http_res
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| GitHubError::HttpParse("Missing status code".into()))?;

    match status {
        "200" => Ok(()),
        "404" => Err(GitHubError::NotFound),
        "403" => Err(GitHubError::Forbidden),
        "503" => Err(GitHubError::ServerUnavailable),
        "304" => Err(GitHubError::NotModified),
        other => match other.parse::<u16>() {
            Ok(code) => Err(GitHubError::UnexpectedStatus(code)),
            Err(_) => Err(GitHubError::HttpParse(format!(
                "Unknown status code: {}",
                other
            ))),
        },
    }
}

pub fn get_request(username: &str) -> String {
    format!(
        "GET /users/{}/events HTTP/1.1\r\n\
    Host: api.github.com\r\n\
    user-agent: rust-cli\r\n\
    connection: close\r\n\r\n",
        username
    )
}
