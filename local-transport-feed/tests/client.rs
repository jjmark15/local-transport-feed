use spectral::prelude::*;

use local_transport_feed::client::FeedClient;

#[test]
fn client_can_start() {
    let client = FeedClient::new();
    let result = client.run();

    assert_that(&result.is_ok()).is_equal_to(true);
}
