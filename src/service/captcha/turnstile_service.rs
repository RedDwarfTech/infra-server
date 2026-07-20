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
    #[serde(default)]
    messages: Vec<String>,
    #[serde(default)]
    hostname: Option<String>,
    #[serde(default, rename = "challenge_ts")]
    challenge_ts: Option<String>,
    #[serde(default)]
    action: Option<String>,
}

fn describe_turnstile_error(code: &str) -> &'static str {
    match code {
        "missing-input-secret" => "未提供 secret 参数",
        "invalid-input-secret" => "secret key 无效或已过期（请核对 Cloudflare 控制台 Secret Key，勿与 Site Key 混用）",
        "missing-input-response" => "未提供 token 参数",
        "invalid-input-response" => "token 无效、格式错误或已过期",
        "bad-request" => "请求格式错误",
        "timeout-or-duplicate" => "token 已过期（5 分钟）或已被重复使用",
        "internal-error" => "Cloudflare 内部错误，可稍后重试",
        _ => "未知错误码",
    }
}

fn mask_secret(secret: &str) -> String {
    let trimmed = secret.trim();
    let len = trimmed.len();
    if len <= 8 {
        return format!("*** (len={len})");
    }
    format!(
        "{}...{} (len={len})",
        &trimmed[..4],
        &trimmed[len - 4..],
        len = len
    )
}

fn mask_token(token: &str) -> String {
    let trimmed = token.trim();
    let len = trimmed.len();
    if len <= 12 {
        return format!("*** (len={len})");
    }
    format!("{}...{} (len={len})", &trimmed[..6], &trimmed[len - 6..])
}

fn log_turnstile_failure(
    body: &TurnstileVerifyResponse,
    secret: &str,
    token: &str,
    remote_ip: Option<&str>,
    http_status: u16,
) {
    let error_details: Vec<String> = body
        .error_codes
        .iter()
        .map(|code| format!("{code} ({})", describe_turnstile_error(code)))
        .collect();

    let secret_trimmed = secret.trim();
    let secret_has_whitespace = secret != secret_trimmed;
    let secret_looks_like_site_key = secret_trimmed.starts_with("0x")
        || (secret_trimmed.len() <= 30 && secret_trimmed.chars().all(|c| c.is_ascii_hexdigit()));

    error!(
        "Turnstile verification failed: http_status={http_status}, \
         error_codes=[{}], messages={:?}, hostname={:?}, challenge_ts={:?}, action={:?}, \
         remote_ip={:?}, token={}, secret={}, \
         secret_has_leading_or_trailing_whitespace={secret_has_whitespace}, \
         secret_may_be_site_key_not_secret={secret_looks_like_site_key}",
        error_details.join("; "),
        body.messages,
        body.hostname,
        body.challenge_ts,
        body.action,
        remote_ip,
        mask_token(token),
        mask_secret(secret),
    );
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
        error!("Turnstile verification skipped: empty token");
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
        Ok(response) => {
            let http_status = response.status().as_u16();
            match response.json::<TurnstileVerifyResponse>().await {
                Ok(body) => {
                    if !body.success {
                        log_turnstile_failure(&body, &secret, token, remote_ip, http_status);
                    }
                    body.success
                }
                Err(err) => {
                    error!(
                        "Turnstile response parse failed: http_status={http_status}, err={err}"
                    );
                    false
                }
            }
        }
        Err(err) => {
            error!("Turnstile siteverify request failed: {}", err);
            false
        }
    }
}
