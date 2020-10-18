use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AirportInstanceTimezoneRaw {
    pub name: Option<String>,
    pub offset: Option<i64>,
    #[serde(rename = "offsetHours")]
    pub offset_hours: Option<String>,
    pub abbr: Option<String>,
    #[serde(rename = "abbrName")]
    pub abbr_name: Option<String>,
    #[serde(rename = "isDst")]
    pub is_dst: Option<bool>,
}