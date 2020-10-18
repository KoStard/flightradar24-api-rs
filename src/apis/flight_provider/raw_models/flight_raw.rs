use serde::Deserialize;
use serde_json::Value;

use crate::apis::flight_provider::raw_models::aircraft_raw::AircraftRaw;
use crate::apis::flight_provider::raw_models::airline_raw::AirlineRaw;
use crate::apis::flight_provider::raw_models::airport_raw::AirportRaw;
use crate::apis::flight_provider::raw_models::flight_identification_raw::FlightIdentificationRaw;
use crate::apis::flight_provider::raw_models::flight_status_raw::FlightStatusRaw;
use crate::apis::flight_provider::raw_models::flight_trail_position_raw::FlightTrailPositionRaw;

#[derive(Deserialize, Debug)]
pub struct FlightRaw {
    pub identification: Option<FlightIdentificationRaw>,
    pub status: Option<FlightStatusRaw>,
    pub level: Option<String>,
    pub promote: Option<bool>,
    pub aircraft: Option<AircraftRaw>,
    pub airline: Option<AirlineRaw>,
    pub owner: Option<Value>,
    pub airspace: Option<Value>,
    pub airport: Option<AirportRaw>,
    #[serde(rename = "flightHistory")]
    pub flight_history: Option<Value>,
    pub ems: Option<Value>,
    pub availability: Option<Vec<String>>,
    pub time: Option<Value>,
    pub trail: Option<Vec<FlightTrailPositionRaw>>,
    #[serde(rename = "firstTimestamp")]
    pub first_timestamp: Option<i64>,
    pub s: Option<String>,
}