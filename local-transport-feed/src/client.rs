#[derive(Default, Debug)]
pub struct FeedClient {}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl FeedClient {
    pub fn new() -> FeedClient {
        FeedClient {}
    }

    pub fn run(&self) -> Result<()> {
        info!("{}", self.log_startup());
        Ok(())
    }

    fn log_startup(&self) -> String {
        format!("FeedClient starting with config: {:?}", self)
    }
}
