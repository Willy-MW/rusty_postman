use std::collections::HashMap;
use std::time::Duration;

pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    timing: Duration,
}
