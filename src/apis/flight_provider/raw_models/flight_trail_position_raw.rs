use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FlightTrailPositionRaw {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub alt: Option<f64>,
    pub spd: Option<f64>,
    pub ts: Option<i64>,
    pub hd: Option<i64>,
}