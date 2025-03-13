use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    let json = response.text().await?;
    println!("{}", json);

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

    let json = response.text().await?;
    println!("{}", json);

    Ok(())
}
