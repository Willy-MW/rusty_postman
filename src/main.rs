use clap::Parser;
use rusty_postman::client::Args;
use rusty_postman::request::RequestConfig;
use rusty_postman::response::print_response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RequestConfig::from_args(Args::parse());
    let client = reqwest::Client::new();
    let request = config.build_request(&client)?;
    let response = client.execute(request).await?;

    print_response(response)?;

    Ok(())
}
