use hyper::client::HttpConnector;
use hyper::Client;

pub mod web;

#[derive(Debug, Default)]
pub struct ExternalWebApiCredential {
    api_key: String,
    app_id: String,
}

impl ExternalWebApiCredential {
    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn get_app_id(&self) -> &String {
        &self.app_id
    }
}

pub trait ExternalWebApi {
    fn get_api_base_url(&self) -> &String;

    fn get_web_client(&self) -> &Client<HttpConnector>;
}
