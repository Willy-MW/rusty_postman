use clap::Parser;
use rusty_postman::client::Args;
use rusty_postman::response::print_response;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let client = reqwest::Client::new();

    println!("Chosen method: {:?}", args.method);

    let mut request = client.request(args.method, args.url);

    if let headers = Some(args.headers) {
        request = request.headers(headers);
    }

    if let body = Some(args.body) {
        request = request.body(body);
    }

    if let Some(timeout) = args.timeout {
        request = request.timeout(Duration::from_secs(timeout));
    }

    let response = request.send().await?;

    print_response(response)?;

    Ok(())
}
