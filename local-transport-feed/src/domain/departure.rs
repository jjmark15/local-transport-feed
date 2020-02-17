#[derive(Eq, PartialEq, Debug)]
pub struct Departure {
    origin_name: String,
    destination_name: String,
    aimed_departure_time: String,
    estimated_departure_time: String,
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

    pub fn origin_name(&self) -> &String {
        &self.origin_name
    }

    pub fn destination_name(&self) -> &String {
        &self.destination_name
    }

    pub fn aimed_departure_time(&self) -> &String {
        &self.aimed_departure_time
    }

    pub fn estimated_departure_time(&self) -> &String {
        &self.estimated_departure_time
    }
}
