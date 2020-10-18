use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AirportInstanceCountryRaw {
    pub id: Option<Value>,
    pub name: Option<String>,
    pub code: Option<String>,
}