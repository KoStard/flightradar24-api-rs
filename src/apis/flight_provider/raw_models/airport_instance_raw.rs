use serde::Deserialize;
use serde_json::Value;

use crate::apis::flight_provider::raw_models::airline_code_raw::AirlineCodeRaw;
use crate::apis::flight_provider::raw_models::airport_instance_info_raw::AirportInstanceInfoRaw;
use crate::apis::flight_provider::raw_models::airport_instance_position_raw::AirportInstancePositionRaw;
use crate::apis::flight_provider::raw_models::airport_instance_timezone_raw::AirportInstanceTimezoneRaw;

#[derive(Deserialize, Debug)]
pub struct AirportInstanceRaw {
    pub name: Option<String>,
    pub code: Option<AirlineCodeRaw>,
    pub position: Option<AirportInstancePositionRaw>,
    pub timezone: Option<AirportInstanceTimezoneRaw>,
    pub visible: Option<bool>,
    pub website: Option<Value>,
    pub info: Option<AirportInstanceInfoRaw>,
}