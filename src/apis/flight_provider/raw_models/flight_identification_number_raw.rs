use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FlightIdentificationNumberRaw {
    pub default: Option<String>,
    pub alternative: Option<String>, // Guess
}