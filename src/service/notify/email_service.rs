use std::env;

use log::{error, warn};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailSendResult {
    pub ok: bool,
}

/// Send a plain-text email via SMTP.
///
/// SMTP server is configured through environment variables:
/// - `SMTP_HOST` (required)
/// - `SMTP_PORT` (default 465)
/// - `SMTP_USERNAME` (required when auth is enabled)
/// - `SMTP_PASSWORD`
/// - `SMTP_FROM` (sender address, required)
/// - `SMTP_FROM_NAME` (sender display name, optional)
///
/// Test bypass: addresses in `EMAIL_TEST_ADDR` (comma separated) skip real
/// delivery and just log the code, mirroring the SMS_TEST_PHONE behavior.
pub fn send_email(to_addr: &str, subject: &str, body: &str) -> bool {
    let test_addrs: Vec<String> = env::var("EMAIL_TEST_ADDR")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if test_addrs.contains(&to_addr.to_string()) {
        warn!(
            "send_email bypassed by EMAIL_TEST_ADDR, to:{}, subject:{}, body:{}",
            to_addr, subject, body
        );
        return true;
    }
    let host = match env::var("SMTP_HOST") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            error!("send_email SMTP_HOST not configured, to:{}", to_addr);
            return false;
        }
    };
    let from_addr = match env::var("SMTP_FROM") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            error!("send_email SMTP_FROM not configured, to:{}", to_addr);
            return false;
        }
    };
    let port: u16 = env::var("SMTP_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(465);
    let from_name = env::var("SMTP_FROM_NAME").unwrap_or_default();
    let from: Mailbox = if from_name.is_empty() {
        match from_addr.parse::<Mailbox>() {
            Ok(v) => v,
            Err(e) => {
                error!("send_email invalid SMTP_FROM, from:{}, err:{}", from_addr, e);
                return false;
            }
        }
    } else {
        match format!("{} <{}>", from_name, from_addr).parse::<Mailbox>() {
            Ok(v) => v,
            Err(e) => {
                error!("send_email invalid SMTP_FROM, from:{}, err:{}", from_addr, e);
                return false;
            }
        }
    };
    let to: Mailbox = match to_addr.parse::<Mailbox>() {
        Ok(v) => v,
        Err(e) => {
            error!("send_email invalid recipient, to:{}, err:{}", to_addr, e);
            return false;
        }
    };
    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .body(body.to_string())
    {
        Ok(v) => v,
        Err(e) => {
            error!("send_email build message failed, to:{}, err:{}", to_addr, e);
            return false;
        }
    };
    let mut builder = match SmtpTransport::relay(&host) {
        Ok(v) => v,
        Err(e) => {
            error!("send_email relay init failed, host:{}, err:{}", host, e);
            return false;
        }
    };
    if let Ok(user) = env::var("SMTP_USERNAME") {
        if !user.is_empty() {
            let pass = env::var("SMTP_PASSWORD").unwrap_or_default();
            builder = builder.credentials(Credentials::new(user, pass));
        }
    }
    let mailer = builder.port(port).build();
    match mailer.send(&email) {
        Ok(response) => {
            let smtp_message: Vec<&str> = response.message().collect();
            warn!(
                "send_email success, to:{}, subject:{}, smtp_code:{}, smtp_response:{:?}",
                to_addr, subject, response.code(), smtp_message
            );
            true
        }
        Err(e) => {
            error!(
                "send_email failed, to:{}, subject:{}, from:{}, host:{}, port:{}, err:{:?}",
                to_addr, subject, from_addr, host, port, e
            );
            false
        }
    }
}
