pub struct FlightProviderRequest {
    pub flight_id: String
}


impl FlightProviderRequest {
    pub fn new(flight_id: String) -> Self {
        FlightProviderRequest {
            flight_id
        }
    }
}
