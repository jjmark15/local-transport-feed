use mockito::Matcher;
use spectral::prelude::*;

use local_transport_feed::client::FeedClient;
use local_transport_feed::domain::station::Station;
use local_transport_feed::services::web::transport::transport_api::TransportApi;
use local_transport_feed::services::web::ExternalWebApiCredential;

#[tokio::test]
async fn given_transport_api_gets_live_train_departures() {
    pretty_env_logger::init();

    // Given mocked response
    let _m = mockito::mock("GET", "/uk/train/station/HRN/live.json")
        .match_query(Matcher::UrlEncoded("app_key".into(), "api_key".into()))
        .match_query(Matcher::UrlEncoded("app_id".into(), "app_id".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body_from_file("tests/files/hornsey_live_trains.json")
        .create();

    // Given a web client
    let web_client = reqwest::Client::new();
    // Given dummy web api credentials
    let credentials: ExternalWebApiCredential =
        ExternalWebApiCredential::new("api_key".to_owned(), "app_id".to_owned());

    // Given an instance of transport api
    let api: TransportApi = TransportApi::new(credentials, mockito::server_url(), web_client);
    let client = FeedClient::new(api);

    // When we get the transport feed from the transport api
    let station: Station = Station::new("HRN".to_string());
    let result = client.get_transport_feed(station).await;

    asserting("response does not error")
        .that(&result.is_ok())
        .is_equal_to(true);

    let departures = result.unwrap();
    asserting("there are three departures returned")
        .that(&departures.len())
        .is_equal_to(3);
}
