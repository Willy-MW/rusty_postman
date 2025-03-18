use clap::Parser;
use clap_derive::Parser;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use rusty_postman::client::HttpMethod;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    method: HttpMethod,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    println!("Chosen method: {:?}", args.method);
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
