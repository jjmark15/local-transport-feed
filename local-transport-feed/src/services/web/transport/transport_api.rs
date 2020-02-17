use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::domain::departure::Departure;
use crate::services::web::{ExternalWebApi, ExternalWebApiCredential};

#[derive(Debug)]
pub struct TransportApi {
    credentials: ExternalWebApiCredential,
    api_base_url: String,
    web_client: reqwest::Client,
}

impl TransportApi {
    pub fn new(
        credentials: ExternalWebApiCredential,
        api_base_url: String,
        getter_client: reqwest::Client,
    ) -> TransportApi {
        TransportApi {
            credentials,
            api_base_url,
            web_client: getter_client,
        }
    }

    pub async fn get_live_arrivals(&self, station_code: String) -> Result<Vec<Departure>, Error> {
        let request_url_string = format!(
            "{base_url}/uk/train/station/{station_code}/live.json",
            station_code = station_code,
            base_url = self.api_base_url
        );
        let mut request_url: reqwest::Url = reqwest::Url::parse(&request_url_string).unwrap();
        request_url
            .query_pairs_mut()
            .append_pair("app_key", &self.credentials.api_key);
        request_url
            .query_pairs_mut()
            .append_pair("app_id", &self.credentials.app_id);
        let resp: TransportApiLiveInfoResponse = self
            .web_client
            .get(request_url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?
            .json::<TransportApiLiveInfoResponse>()
            .await?;

        Ok(resp.departures.into())
    }
}

impl ExternalWebApi for TransportApi {
    fn get_api_base_url(&self) -> &String {
        &self.api_base_url
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportApiLiveResponseDepartures {
    all: Vec<TransportApiDeparture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportApiLiveInfoResponse {
    departures: TransportApiLiveResponseDepartures,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransportApiDeparture {
    aimed_departure_time: String,
    expected_departure_time: String,
    origin_name: String,
    destination_name: String,
}

impl Into<Departure> for &TransportApiDeparture {
    fn into(self) -> Departure {
        let origin = self.origin_name.to_owned();
        let destination = self.destination_name.to_owned();
        let aimed_departure_time = self.aimed_departure_time.clone();
        let estimated_departure_time = self.expected_departure_time.clone();
        Departure::new(
            origin,
            destination,
            aimed_departure_time,
            estimated_departure_time,
        )
    }
}

impl Into<Vec<Departure>> for TransportApiLiveResponseDepartures {
    fn into(self) -> Vec<Departure> {
        self.all.iter().map(|t| t.into()).collect()
    }
}
