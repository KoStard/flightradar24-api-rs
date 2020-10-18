use serde::Deserialize;

use crate::apis::flight_provider::raw_models::airport_instance_country_raw::AirportInstanceCountryRaw;
use crate::apis::flight_provider::raw_models::airport_instance_region_raw::AirportInstanceRegionRaw;

#[derive(Deserialize, Debug)]
pub struct AirportInstancePositionRaw {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub country: Option<AirportInstanceCountryRaw>,
    pub region: Option<AirportInstanceRegionRaw>,
}