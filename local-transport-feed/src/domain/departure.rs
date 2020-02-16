use crate::domain::station::Station;

pub struct Departure {
    pub origin: Station,
    pub destination: Station,
    pub aimed_departure_time: String,
    pub estimated_departure_time: String,
}

impl Departure {
    pub fn new(
        origin: Station,
        destination: Station,
        aimed_departure_time: String,
        estimated_departure_time: String,
    ) -> Departure {
        Departure {
            origin,
            destination,
            aimed_departure_time,
            estimated_departure_time,
        }
    }
}
