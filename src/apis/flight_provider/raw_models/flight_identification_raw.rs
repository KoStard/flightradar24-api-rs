use serde::Deserialize;

use crate::apis::flight_provider::raw_models::flight_identification_number_raw::FlightIdentificationNumberRaw;

#[derive(Deserialize, Debug)]
pub struct FlightIdentificationRaw {
    pub id: Option<String>,
    pub row: Option<i64>,
    pub number: Option<FlightIdentificationNumberRaw>,
    pub callsign: Option<String>,
}
