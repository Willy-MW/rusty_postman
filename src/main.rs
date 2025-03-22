use clap::Parser;
use rusty_postman::client::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    println!("Chosen method: {:?}", args.method);
    Ok(())
}
