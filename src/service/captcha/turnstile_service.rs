use log::error;
use reqwest::Client;
use serde::Deserialize;
use std::env;

const TURNSTILE_VERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[derive(Deserialize)]
struct TurnstileVerifyResponse {
    success: bool,
    #[serde(default, rename = "error-codes")]
    error_codes: Vec<String>,
}

/// Verify a Cloudflare Turnstile token via Siteverify API.
pub async fn verify_turnstile_token(token: &str, remote_ip: Option<&str>) -> bool {
    let secret = match env::var("CF_TURNSTILE_SECRET_KEY") {
        Ok(value) if !value.is_empty() => value,
        _ => {
            error!("CF_TURNSTILE_SECRET_KEY is not configured");
            return false;
        }
    };

    if token.trim().is_empty() {
        return false;
    }

    let mut params = vec![("secret", secret.as_str()), ("response", token)];
    if let Some(ip) = remote_ip {
        if !ip.is_empty() {
            params.push(("remoteip", ip));
        }
    }

    let client = Client::new();
    match client
        .post(TURNSTILE_VERIFY_URL)
        .form(&params)
        .send()
        .await
    {
        Ok(response) => match response.json::<TurnstileVerifyResponse>().await {
            Ok(body) => {
                if !body.success {
                    error!("Turnstile verification failed: {:?}", body.error_codes);
                }
                body.success
            }
            Err(err) => {
                error!("Turnstile response parse failed: {}", err);
                false
            }
        },
        Err(err) => {
            error!("Turnstile siteverify request failed: {}", err);
            false
        }
    }
}
