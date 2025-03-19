use crate::client::HttpMethod;
use clap_derive::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RequestConfig {
    #[arg(value_enum)]
    pub method: HttpMethod,
    pub url: String,
    #[arg(short, long)]
    pub headers: Option<HashMap<String, String>>,
    #[arg(short, long)]
    #[command(subcommand)]
    pub body: Option<RequestBody>,
    #[arg(short, long)]
    pub timeout: Option<Duration>,
    #[arg(short, long, default_value = false)]
    pub follow_redirects: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Text(String),
    Binary(Vec<u8>),
}
