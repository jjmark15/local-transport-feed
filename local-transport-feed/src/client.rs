use serde::export::fmt::Debug;

use crate::domain::departure::Departure;
use crate::services::web::transport::transport_api::TransportApi;

#[derive(Debug)]
pub struct FeedClient {
    transport_api: TransportApi,
}

impl FeedClient {
    pub fn new(transport_api: TransportApi) -> FeedClient {
        FeedClient { transport_api }
    }

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
