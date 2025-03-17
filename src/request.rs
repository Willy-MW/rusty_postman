use crate::client::HttpMethod;
use std::collections::HashMap;
use std::time::Duration;

pub struct RequestConfig {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<RequestBody>,
    timeout: Option<Duration>,
    follow_redirects: bool,
}

pub enum RequestBody {
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Text(String),
    Binary(Vec<u8>),
}
