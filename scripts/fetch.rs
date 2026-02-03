use reqwest::header::AUTHORIZATION;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Instant;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct AuthenticationResult {
    AccessToken: String,
    ExpiresIn: u64,
    TokenType: String,
    RefreshToken: String,
    IdToken: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct Authentication {
    AuthenticationResult: AuthenticationResult,
}

#[tokio::main]
async fn main() {
    let client = HttpClient::new();
    let start = Instant::now();
    let content = fs::read_to_string(".authentication.json").unwrap();
    let authentication: Authentication = serde_json::from_str(&content).unwrap();

    let mut auth_header_value = "Bearer ".to_string();
    auth_header_value.push_str(&authentication.AuthenticationResult.IdToken);
    let response = client
        .get("https://p7vtih0jq1.execute-api.ap-northeast-1.amazonaws.com/hello")
        .header(AUTHORIZATION, auth_header_value)
        .send()
        .await
        .unwrap();

    let response_content = response.text().await.unwrap();
    let duration = start.elapsed();

    println!("Duration: {duration:?}");
    println!("Response: {response_content}");
}
