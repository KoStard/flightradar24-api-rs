use serde::Deserialize;
use serde_json::Value;

use crate::apis::flight_provider::raw_models::airport_instance_raw::AirportInstanceRaw;

#[derive(Deserialize, Debug)]
pub struct AirportRaw {
    pub real: Option<Value>,
    pub origin: Option<AirportInstanceRaw>,
    pub destination: Option<AirportInstanceRaw>,
}