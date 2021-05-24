use std::future::Future;

use reqwest::{header::HeaderMap, Client, IntoUrl, Response, Result};
use serde::Serialize;

pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Result<Self> {
        let header_map = HeaderMap::new();

        Client::builder()
            .https_only(true)
            .default_headers(header_map)
            .build()
            .map(|client| Self { client })
    }

    pub fn get<U>(&self, url: U) -> impl Future<Output = Result<Response>>
    where
        U: IntoUrl,
    {
        self.client.get(url).send()
    }

    pub fn post<U, B>(&self, url: U, body: &B) -> impl Future<Output = Result<Response>>
    where
        U: IntoUrl,
        B: Serialize,
    {
        self.client.post(url).json(body).send()
    }
}
