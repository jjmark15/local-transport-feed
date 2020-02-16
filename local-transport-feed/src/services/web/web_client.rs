use reqwest::{Error, Response};

use async_trait::async_trait;

#[async_trait]
pub trait WebClient: Sync {
    async fn get(&self, uri: &str) -> Result<Response, Error> {
        reqwest::get(uri).await
    }
}

#[derive(Debug, Default)]
pub struct ProdWebClient {}

impl WebClient for ProdWebClient {}
