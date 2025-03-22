use reqwest::Method;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct RequestConfig {
    pub method: Method,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
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
