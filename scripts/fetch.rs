use reqwest::Client as HttpClient;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let client = HttpClient::new();
    let start = Instant::now();
    let response = client
        .get("https://p7vtih0jq1.execute-api.ap-northeast-1.amazonaws.com/hello")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let duration = start.elapsed();
    print!("Duration: {duration:?}\n");
    print!("Response: {response}\n");
}
