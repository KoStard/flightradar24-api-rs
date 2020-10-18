use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct FlightStatusRaw {
    pub live: Option<bool>,
    pub text: Option<String>,
    pub icon: Option<String>,
    pub estimated: Option<Value>,
    pub ambiguous: Option<bool>,
    pub generic: Option<Value>,
}