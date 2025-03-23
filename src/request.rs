use reqwest::Method;
use std::collections::HashMap;
use std::time::Duration;
use reqwest::header::HeaderMap;
use crate::client::Args;

#[derive(Debug)]
pub struct RequestConfig {
    pub method: Method,
    pub url: String,
    pub headers: Option<HeaderMap>,
    pub body: Option<RequestBody>,
    pub timeout: Option<Duration>,
    pub follow_redirects: bool,
}

#[derive(Debug)]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Text(String),
    Binary(Vec<u8>),
}

impl RequestConfig {
    pub fn from_args(args: Args) -> Self {
        Self {
            method: args.method,
            url: args.url,
            headers: Self::parse_headers(args.headers),
            body: args.body,
            timeout: args.timeout,
            follow_redirects: args.follow_redirects,
        }
    }

    fn parse_headers(headers: Option<Vec<(String, String)>>) -> Option<HeaderMap> {
        todo!()
    }
}
