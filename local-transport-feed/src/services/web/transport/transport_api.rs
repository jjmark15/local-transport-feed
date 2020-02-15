use hyper::client::HttpConnector;
use hyper::Client;

use crate::services::{ExternalWebApi, ExternalWebApiCredential};

#[derive(Debug, Default)]
pub struct TransportApi {
    credentials: ExternalWebApiCredential,
    api_base_url: String,
    web_client: Client<HttpConnector>,
}

impl TransportApi {
    pub fn new(
        credentials: ExternalWebApiCredential,
        api_base_url: String,
        web_client: Client<HttpConnector>,
    ) -> TransportApi {
        TransportApi {
            credentials,
            api_base_url,
            web_client,
        }
    }
}

impl ExternalWebApi for TransportApi {
    fn get_api_base_url(&self) -> &String {
        &self.api_base_url
    }

    fn get_web_client(&self) -> &Client<HttpConnector> {
        &self.web_client
    }
}
