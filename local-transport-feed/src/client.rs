use serde::export::fmt::Debug;

use crate::domain::departure::Departure;
use crate::domain::station::Station;
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
        station: Station,
    ) -> Result<Vec<Departure>, reqwest::Error> {
        let results = self.transport_api.get_live_arrivals(station).await;
        if let Err(e) = &results {
            error!("an error occurred when getting live arrivals: {:?}", e)
        }
        results
    }
}
