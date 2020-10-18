use serde::Deserialize;

use crate::apis::flight_provider::raw_models::airline_code_raw::AirlineCodeRaw;

#[derive(Deserialize, Debug)]
pub struct AirlineRaw {
    pub name: Option<String>,
    pub short: Option<String>,
    pub code: Option<AirlineCodeRaw>,
    pub url: Option<String>,
}
