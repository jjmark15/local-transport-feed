use crate::services::ExternalWebApiCredential;

#[derive(Debug, Default)]
pub struct TransportApiCredential {
    api_key: String,
    app_id: String,
}

impl ExternalWebApiCredential for TransportApiCredential {
    fn get_api_key(&self) -> &String {
        &self.api_key
    }

    fn get_app_id(&self) -> &String {
        &self.app_id
    }
}
