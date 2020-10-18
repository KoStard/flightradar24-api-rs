use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AirportInstanceRegionRaw {
    pub city: Option<String>
}