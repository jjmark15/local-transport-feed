pub mod web;

trait ExternalWebApiCredential {
    fn get_api_key(&self) -> &String;
    fn get_app_id(&self) -> &String;
}
