use serde::export::fmt::Debug;

use crate::domain::departure::Departure;
use crate::services::web::transport::transport_api::TransportApi;

#[derive(Debug)]
/// Object that manages instances of API clients such as transport api
pub struct FeedClient {
    transport_api: TransportApi,
}

impl FeedClient {
    /// Constructs a new `FeedClient`
    /// # Example
    ///
    /// ```rust
    /// # use local_transport_feed::services::web::transport::transport_api::TransportApi;
    /// # use local_transport_feed::services::web::ExternalWebApiCredential;
    /// # let credentials = ExternalWebApiCredential::new("api_key".to_string(), "app_id".to_string());
    /// # let api_base_url = "http://transportapi.com/".to_string();
    /// # let client = reqwest::Client::new();
    /// let transport_api = TransportApi::new(credentials, api_base_url, client);
    /// ```
    pub fn new(transport_api: TransportApi) -> FeedClient {
        FeedClient { transport_api }
    }

    /// Queries the transport api for live `Departures` leaving from a station (referenced by `station_code`).
    pub async fn get_transport_feed(
        &self,
        station_code: String,
    ) -> Result<Vec<Departure>, reqwest::Error> {
        let results = self.transport_api.get_live_arrivals(station_code).await;
        if let Err(e) = &results {
            error!("an error occurred when getting live arrivals: {:?}", e)
        }
        results
    }
}
