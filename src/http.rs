use crate::utility::KyaniteUtility;
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

const CONNECT_TIMEOUT: u64 = 20;
const REQUEST_TIMEOUT: u64 = 300;

pub struct KyaniteClient {
    pub client: Client,
}

impl KyaniteClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        let user_agent = format!(
            "kyanite {}/@axaz0r <thealeksaradovic@gmail.com>",
            KyaniteUtility::version()
        );
        headers.append("User-Agent", (&user_agent).parse().unwrap());
        let client = ClientBuilder::new()
            .connect_timeout(Duration::new(CONNECT_TIMEOUT, 0))
            .timeout(Duration::new(REQUEST_TIMEOUT, 0))
            .default_headers(headers)
            .build()
            .unwrap();
        Self { client }
    }
}
