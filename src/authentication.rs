use crate::error::{CrabError, CrabResult};
use crate::ui;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PublishRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct PublishResponse {
    success: bool,
    message: String,
}

pub async fn publish(username: &str, password: &str) -> CrabResult<()> {
    if username.is_empty() || password.is_empty() {
        return Err(CrabError::AuthenticationError(
            "Username and password cannot be empty".to_string(),
        ));
    }

    ui::print_info(&format!("Publishing package as '{}'...", username));
    ui::simulate_upload_animation();

    let client = Client::new();
    let url = "https://reefs.io/api/publish";

    let body = PublishRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if response.status().is_success() {
                ui::print_success("Package published successfully!");
                Ok(())
            } else {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(CrabError::NetworkError(format!(
                    "HTTP {}: {}",
                    status, error_text
                )))
            }
        }
        Err(e) => {
            if e.is_connect() {
                Err(CrabError::NetworkError(
                    "Failed to connect to crabby.io. Please check your internet connection."
                        .to_string(),
                ))
            } else if e.is_timeout() {
                Err(CrabError::NetworkError(
                    "Request timed out. Please try again later.".to_string(),
                ))
            } else {
                Err(CrabError::NetworkError(format!(
                    "Failed to publish: {}",
                    e
                )))
            }
        }
    }
}

pub async fn login(username: &str, password: &str) -> CrabResult<String> {
    if username.is_empty() || password.is_empty() {
        return Err(CrabError::AuthenticationError(
            "Username and password cannot be empty".to_string(),
        ));
    }

    ui::print_info("Authenticating...");

    let client = Client::new();
    let url = "https://crabby.io/api/login";

    let body = PublishRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let data: PublishResponse = response.json().await?;
                if data.success {
                    ui::print_success("Logged in successfully!");
                    Ok(data.message)
                } else {
                    Err(CrabError::AuthenticationError(data.message))
                }
            } else {
                Err(CrabError::AuthenticationError(
                    "Invalid username or password".to_string(),
                ))
            }
        }
        Err(e) => Err(CrabError::NetworkError(format!(
            "Login failed: {}",
            e
        ))),
    }
}

pub async fn logout() -> CrabResult<()> {
    ui::print_info("Logging out...");
    ui::print_success("Logged out successfully!");
    Ok(())
}
