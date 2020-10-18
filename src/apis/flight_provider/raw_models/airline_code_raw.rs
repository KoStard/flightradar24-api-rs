use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AirlineCodeRaw {
    pub iata: Option<String>,
    pub icao: Option<String>,
}