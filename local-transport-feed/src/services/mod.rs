pub mod web;

#[derive(Debug, Default)]
pub struct ExternalWebApiCredential {
    api_key: String,
    app_id: String,
}

impl ExternalWebApiCredential {
    fn get_api_key(&self) -> &String {
        &self.api_key
    }

    fn get_app_id(&self) -> &String {
        &self.app_id
    }
}

pub trait ExternalWebApi {}
