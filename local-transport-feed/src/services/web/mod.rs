pub mod transport;
pub mod web_client;

#[derive(Debug, Default)]
pub struct ExternalWebApiCredential {
    api_key: String,
    app_id: String,
}

impl ExternalWebApiCredential {
    pub fn new(api_key: String, app_id: String) -> ExternalWebApiCredential {
        ExternalWebApiCredential { api_key, app_id }
    }

    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }

    pub fn get_app_id(&self) -> &String {
        &self.app_id
    }
}

pub trait ExternalWebApi {
    fn get_api_base_url(&self) -> &String;
}
