use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AirportInstanceInfoRaw {
    pub terminal: Option<String>,
    pub baggage: Option<Value>,
    pub gate: Option<Value>,
}
