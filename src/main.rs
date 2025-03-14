use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;
use anyhow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    print_response(response).await?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);

    let json: HashMap<&str, &str> = [("title", "foo"), ("body", "bar"), ("userId", "1")]
        .into_iter()
        .collect();

    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .headers(headers)
        .json(&json)
        .send()
        .await?;

    print_response(response).await?;

    Ok(())
}

async fn print_response(response: reqwest::Response) -> anyhow::Result<()> {
    let status = response.status().to_string();
    println!("Status: {}", status);

    println!("-----------------------------------");

    let headers = response.headers();
    println!("Headers: {:?}", headers);

    println!("-----------------------------------");

    let body = response.text().await?;
    println!("Body: {}", body);

    println!("----------------------------------------------------------------------|");
    Ok(())
}
