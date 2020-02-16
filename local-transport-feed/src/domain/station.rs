#[derive(Debug)]
pub struct Station {
    pub station_code: String,
}

impl Station {
    pub fn new(station_code: String) -> Station {
        Station { station_code }
    }
}
