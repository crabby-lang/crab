use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct PublishRequest {
    username: String,
    password: String,
}

pub async fn publish(username: &str, password: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://crabby.io/api/publish";

    let body = PublishRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Package published successfully!");
        Ok(())
    } else {
        let error_text = response.text().await?;

        Err(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, error_text))
    }
}
