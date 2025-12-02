use log::error;
use reqwest::StatusCode;
use serde_json::json;
use std::env;

/// Send alert message to Server酱 (server chan)
///
/// # Arguments
/// * `send_key` - The send key from Server酱 (https://sct.ftqq.com/)
/// * `title` - Message title
/// * `content` - Message content/body
///
/// # Example
/// ```
/// let result = send_to_server_chan("your-send-key", "Alert Title", "Alert content here");
/// ```
pub async fn send_to_server_chan(send_key: &str, title: &str, content: &str) -> bool {
    if send_key.is_empty() {
        error!("Server Chan send_key is empty");
        return false;
    }

    // Server Chan API endpoint
    let url = format!("https://sct.ftqq.com/{}.send", send_key);

    // Prepare request body
    let body = json!({
        "title": title,
        "desp": content
    });

    // Create HTTP client
    let client = reqwest::Client::new();

    match client.post(&url).json(&body).send().await {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    match response.json::<serde_json::Value>().await {
                        Ok(resp_json) => {
                            // Server Chan response format: {"code": 0, "message": "sent", ...}
                            if let Some(code) = resp_json.get("code") {
                                if code == 0 {
                                    return true;
                                } else {
                                    error!("Server Chan API error: {}", resp_json);
                                    return false;
                                }
                            }
                            return false;
                        }
                        Err(e) => {
                            error!("Failed to parse Server Chan response: {}", e);
                            return false;
                        }
                    }
                }
                status => {
                    error!("Server Chan request failed with status: {}", status);
                    return false;
                }
            }
        }
        Err(e) => {
            error!("Failed to send message to Server Chan: {}", e);
            return false;
        }
    }
}

/// Send alert message to Server酱 with webhook format
///
/// This is an alternative method that supports OpenAI-like markdown formatting
///
/// # Arguments
/// * `send_key` - The send key from Server酱
/// * `title` - Message title
/// * `content` - Message content with markdown support
pub async fn send_to_server_chan_with_markdown(send_key: &str, title: &str, content: &str) -> bool {
    if send_key.is_empty() {
        error!("Server Chan send_key is empty");
        return false;
    }

    let url = format!("https://sct.ftqq.com/{}.send", send_key);

    let body = json!({
        "title": title,
        "desp": format!("## {}\n\n{}", title, content)
    });

    let client = reqwest::Client::new();

    match client.post(&url).json(&body).send().await {
        Ok(response) => match response.status() {
            StatusCode::OK => match response.json::<serde_json::Value>().await {
                Ok(resp_json) => {
                    if let Some(code) = resp_json.get("code") {
                        if code == 0 {
                            return true;
                        }
                    }
                    return false;
                }
                Err(e) => {
                    error!("Failed to parse Server Chan response: {}", e);
                    return false;
                }
            },
            status => {
                error!("Server Chan request failed with status: {}", status);
                return false;
            }
        },
        Err(e) => {
            error!("Failed to send markdown message to Server Chan: {}", e);
            return false;
        }
    }
}

/// Get Server Chan send key from environment variable
///
/// Returns the send key if set, None otherwise
pub fn get_server_chan_sendkey() -> Option<String> {
    env::var("SERVER_CHAN_SENDKEY").ok()
}

/// Send alert message to Server酱 using environment variable
///
/// This method reads the send key from SERVER_CHAN_SENDKEY environment variable
pub async fn send_alert(title: &str, content: &str) -> bool {
    match get_server_chan_sendkey() {
        Some(send_key) => send_to_server_chan(&send_key, title, content).await,
        None => {
            error!("SERVER_CHAN_SENDKEY environment variable not set");
            false
        }
    }
}
