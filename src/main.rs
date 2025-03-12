use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    userId: isize,
    id: isize,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    let json = response.json::<Post>().await?;
    println!("{:?}", json);

    Ok(())
}
