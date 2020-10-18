#[derive(Debug)]
pub struct FlightBrief {
    pub id: String,  // Flight's unique ID within FR24
    pub mode_s: String,  // Aircraft Mode-S code
    pub latitude: f64,  // Aircraft latitude
    pub longitude: f64,  // Aircraft longitude
    pub track: i64,  // Aircraft direction in degrees
    pub altitude: i64,  // Aircraft altitude in kilometers
    pub speed: i64,  // Aircraft speed in kilometers per hour
    pub squawk: String,  // Aircraft squawk code
    pub radar: String,  // Current control tower code
    pub model: String,  // Aircraft model
    pub registration: String,  // Aircraft registration
    pub undefined: i64,
    pub origin: String,  // Flight origin code
    pub destination: String,  // Flight destination code
    pub iata: String,  // Flight code in IATA format
    pub undefined2: i64,
    pub vertical_speed: i64,  // Aircraft vertical speed
    pub icao: String,  // Flight code in ICAO format
    pub undefined3: i64,
    pub airline: String,  // Airline code
}
