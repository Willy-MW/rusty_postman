use clap_derive::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub method: HttpMethod,
    pub url: String,
    #[arg(long)]
    pub headers: Option<String>,
    #[arg(short, long)]
    pub body: Option<String>,
    #[arg(short, long)]
    pub timeout: Option<String>,
    #[arg(short, long)]
    pub follow_redirects: bool,
}

impl FromStr for HttpMethod {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err("Unknown HTTP Method"),
        }
    }
}
