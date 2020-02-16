use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::domain::departure::Departure;
use crate::domain::station::Station;
use crate::services::web::web_client::WebClient;
use crate::services::web::{ExternalWebApi, ExternalWebApiCredential};

#[derive(Debug)]
pub struct TransportApi<T: WebClient> {
    credentials: ExternalWebApiCredential,
    api_base_url: String,
    web_client: T,
}

impl<T: WebClient> TransportApi<T> {
    pub fn new(
        credentials: ExternalWebApiCredential,
        api_base_url: String,
        getter_client: T,
    ) -> TransportApi<T> {
        TransportApi {
            credentials,
            api_base_url,
            web_client: getter_client,
        }
    }

    pub async fn get_live_arrivals(&self, station: Station) -> Result<Vec<Departure>, Error> {
        let request_url = format!(
            "{base_url}/transport/{station_code}",
            station_code = station.station_code,
            base_url = self.api_base_url
        );
        let resp: TransportApiLiveInfoResponse = self
            .web_client
            .get(&request_url)
            .await?
            .json::<TransportApiLiveInfoResponse>()
            .await?;
        println!("{:?}", resp);

        //        Ok(resp.departures.into::<Vec<Departure>>())
        Ok(vec![])
    }
}

impl<T: WebClient> ExternalWebApi for TransportApi<T> {
    fn get_api_base_url(&self) -> &String {
        &self.api_base_url
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportApiLiveInfoResponse {
    departures: Vec<TransportApiDeparture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportApiDeparture {
    aimed_departure_time: String,
    expected_departure_time: String,
    origin_name: String,
    destination_name: String,
}

impl Into<Departure> for TransportApiDeparture {
    fn into(self) -> Departure {
        unimplemented!()
    }
}
