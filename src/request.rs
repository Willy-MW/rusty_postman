use crate::client::Args;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method, Request};
use std::time::Duration;

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
    //Form(HashMap<String, String>),
    Text(String),
    Binary(Vec<u8>),
}

impl RequestConfig {
    pub fn from_args(args: Args) -> Self {
        Self {
            method: args.method,
            url: args.url,
            headers: Self::parse_headers(args.headers),
            body: Self::parse_body(args.body),
            timeout: args.timeout.map(Duration::from_secs),
            follow_redirects: args.follow_redirects,
        }
    }

    fn parse_headers(headers: Option<Vec<(String, String)>>) -> Option<HeaderMap> {
        todo!()
    }

    fn parse_body(body: Option<String>) -> Option<RequestBody> {
        todo!()
    }

    pub fn build_request(self, client: &Client) -> Result<Request, reqwest::Error> {
        let mut builder = client.request(self.method, self.url);

        if let Some(headers) = self.headers {
            builder = builder.headers(headers);
        }

        if let Some(request_body) = self.body {
            let reqwest_body = request_body.try_into()?;
            builder = builder.body(reqwest_body);
        }

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        builder.build()
    }
}

impl TryInto<reqwest::Body> for RequestBody {
    type Error = reqwest::Error;

    fn try_into(self) -> Result<reqwest::Body, Self::Error> {
        match self {
            Self::Json(json) => Ok(reqwest::Body::from(json.to_string())),
            Self::Text(text) => Ok(reqwest::Body::from(text)),
            Self::Binary(bytes) => Ok(reqwest::Body::from(bytes)),
        }
    }
}
