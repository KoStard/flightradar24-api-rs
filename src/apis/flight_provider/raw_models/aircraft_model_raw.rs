use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AircraftModelRaw {
    pub code: Option<String>,
    pub text: Option<String>,
}