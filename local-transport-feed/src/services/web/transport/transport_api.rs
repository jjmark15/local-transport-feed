use crate::services::{ExternalWebApi, ExternalWebApiCredential};

#[derive(Debug, Default)]
pub struct TransportApi {
    credentials: ExternalWebApiCredential,
}

impl TransportApi {
    pub fn new(credentials: ExternalWebApiCredential) -> TransportApi {
        TransportApi { credentials }
    }
}

impl ExternalWebApi for TransportApi {}
