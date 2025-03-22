use clap_derive::Parser;
use reqwest::Method;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub method: Method,
    pub url: String,
    #[arg(long, value_parser = parse_headers, num_args = 1.., value_delimiter = ' ')]
    pub headers: Option<Vec<(String, String)>>,
    #[arg(short, long)]
    pub body: Option<String>,
    #[arg(short, long)]
    pub timeout: Option<u64>,
    #[arg(short, long, default_value = "true")]
    pub follow_redirects: bool,
}

fn parse_headers(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("Header must be in format 'key:value': {}", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
