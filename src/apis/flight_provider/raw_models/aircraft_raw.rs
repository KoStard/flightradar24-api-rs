use serde::Deserialize;
use serde_json::Value;

use crate::apis::flight_provider::raw_models::aircraft_model_raw::AircraftModelRaw;

#[derive(Deserialize, Debug)]
pub struct AircraftRaw {
    pub model: Option<AircraftModelRaw>,
    #[serde(rename = "countryId")]
    pub country_id: Option<i32>,
    pub registration: Option<String>,
    pub hex: Option<String>,
    pub age: Option<Value>,
    pub msn: Option<Value>,
    pub images: Option<Value>,
}