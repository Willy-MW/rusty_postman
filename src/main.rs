use clap::Parser;
use rusty_postman::client::Args;
use rusty_postman::request::RequestConfig;
// use clap_derive::Parser;
// use rusty_postman::client::HttpMethod;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     #[arg(value_enum)]
//     method: HttpMethod,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    println!("Chosen method: {:?}", args.method);
    Ok(())
}
