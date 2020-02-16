use serde::export::fmt::Debug;

use crate::domain::departure::Departure;
use crate::domain::station::Station;
use crate::services::web::transport::transport_api::TransportApi;
use crate::services::web::web_client::WebClient;

#[derive(Debug)]
pub struct FeedClient<T: WebClient> {
    transport_api: TransportApi<T>,
}

impl<T: WebClient + Debug> FeedClient<T> {
    pub fn new(transport_api: TransportApi<T>) -> FeedClient<T> {
        FeedClient { transport_api }
    }

    pub async fn get_transport_feed(
        &self,
        station: Station,
    ) -> Result<Vec<Departure>, reqwest::Error> {
        let results = self.transport_api.get_live_arrivals(station).await?;
        Ok(vec![])
    }
}
