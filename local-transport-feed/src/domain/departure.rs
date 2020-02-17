#[derive(Eq, PartialEq, Debug)]
pub struct Departure {
    pub origin_name: String,
    pub destination_name: String,
    pub aimed_departure_time: String,
    pub estimated_departure_time: String,
}

impl Departure {
    pub fn new(
        origin_name: String,
        destination_name: String,
        aimed_departure_time: String,
        estimated_departure_time: String,
    ) -> Departure {
        Departure {
            origin_name,
            destination_name,
            aimed_departure_time,
            estimated_departure_time,
        }
    }
}
