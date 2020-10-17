use crate::apis::api_caller::ApiCaller;
use crate::apis::live_area_information_provider::flight_brief_model::FlightBrief;
use crate::models::area::Area;
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use serde_json::Value;

pub struct LiveAreaInformation {}

fn string_handler(value: Option<&serde_json::Value>) -> Result<String, String> {
    Ok(value
        .ok_or("List out of range when deserializing an array from json".to_owned())?
        .as_str()
        .ok_or("The actual type of element in array is not string!".to_owned())?
        .into())
}

fn i64_handler(value: Option<&serde_json::Value>) -> Result<i64, String> {
    value
        .ok_or("List out of range when deserializing an array from json".to_owned())?
        .as_i64()
        .ok_or("The actual type of element in array is not i64!".to_owned())
}

fn f64_handler(value: Option<&serde_json::Value>) -> Result<f64, String> {
    value
        .ok_or("List out of range when deserializing an array from json".to_owned())?
        .as_f64()
        .ok_or("The actual type of element in array is not f64!".to_owned())
}

impl LiveAreaInformation {
    pub fn new() -> Self {
        LiveAreaInformation {}
    }

    pub async fn get(&self, area: Area) -> Result<HashMap<String, FlightBrief>, String> {
        let url = Self::generate_url(area);
        let api_caller = ApiCaller::new(url);
        let raw_response = api_caller.get().await.unwrap();
        Self::parse_response(raw_response)
    }

    fn generate_url(area: Area) -> String {
        format!(
            "https://data-live.flightradar24.com/zones/fcgi/feed.js\
                   ?bounds={south_west_latitude},{south_west_longitude},\
                   {north_east_latitude},{north_east_longitude}\
                   &faa=1&mlat=1&flarm=1&adsb=1&gnd=1&air=1&estimated=1\
                   &maxage=14400&gliders=1&stats=1",
            south_west_latitude = area.left_top.latitude,
            south_west_longitude = area.right_bottom.longitude,
            north_east_latitude = area.right_bottom.latitude,
            north_east_longitude = area.left_top.longitude
        )
    }

    fn parse_response(raw_response: String) -> Result<HashMap<String, FlightBrief>, String> {
        let unclassified_response: HashMap<String, serde_json::Value> =
            serde_json::from_str(raw_response.as_str())
                .map_err(|e| e.to_string())?;
        let mut classified_response: HashMap<String, FlightBrief> = HashMap::new();

        for (key, value) in unclassified_response.iter() {
            if value.is_array() {
                match Self::parse_instance(key, value) {
                    Ok(flight_brief) => {
                        classified_response.insert(flight_brief.id.clone(), flight_brief);
                    }
                    Err(err) => {
                        println!("Could not parse array - {}, skipping, got error - {}", value, err);
                    }
                };
            }
        }
        Ok(classified_response)
    }

    fn parse_instance(key: &String, value: &Value) -> Result<FlightBrief, String> {
        Ok(FlightBrief {
            id: key.clone(),
            mode_s: string_handler(value.get(0))?,
            latitude: f64_handler(value.get(1))?,
            longitude: f64_handler(value.get(2))?,
            track: i64_handler(value.get(3))?,
            altitude: i64_handler(value.get(4))?,
            speed: i64_handler(value.get(5))?,
            squawk: string_handler(value.get(6))?,
            radar: string_handler(value.get(7))?,
            model: string_handler(value.get(8))?,
            registration: string_handler(value.get(9))?,
            undefined: i64_handler(value.get(10))?,
            origin: string_handler(value.get(11))?,
            destination: string_handler(value.get(12))?,
            iata: string_handler(value.get(13))?,
            undefined2: i64_handler(value.get(14))?,
            vertical_speed: i64_handler(value.get(15))?,
            icao: string_handler(value.get(16))?,
            undefined3: i64_handler(value.get(17))?,
            airline: string_handler(value.get(18))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::apis::live_area_information_provider::live_area_information::LiveAreaInformation;
    use crate::models::area::Area;
    use crate::models::point::Point;
    use bigdecimal::{BigDecimal, One, Zero};
    use std::str::FromStr;
    use tokio_test::block_on;

    #[test]
    fn generate_url_works() {
        // 42.858110, 40.032470
        // 38.202162, 51.214457
        println!(
            "{}",
            LiveAreaInformation::generate_url(Area {
                left_top: Point {
                    longitude: BigDecimal::from_str("42.858110").unwrap(),
                    latitude: BigDecimal::from_str("40.032470").unwrap(),
                },
                right_bottom: Point {
                    longitude: BigDecimal::from_str("38.202162").unwrap(),
                    latitude: BigDecimal::from_str("51.214457").unwrap(),
                },
            })
        );
    }

    #[test]
    fn parse_response_works() {
        println!("{:?}", LiveAreaInformation::parse_response("{\"25cd0734\": [\"0AA004\",38.709,-104.266,\"77\",45000,546,\"1013\",\"F-KCOS2\",\"GLF5\",\"8P-MSD\",1602965667,\"BUR\",\"\",\"\",0,64,\"X8PMSD\",0,\"\"]}".into()));
    }

    #[test]
    fn get_with_actual_area_works() {
        // 42.858110, 40.032470
        // 38.202162, 51.214457
        println!("{:?}", block_on(LiveAreaInformation::new().get(Area {
            left_top: Point {
                longitude: BigDecimal::from_str("42.858110").unwrap(),
                latitude: BigDecimal::from_str("40.032470").unwrap(),
            },
            right_bottom: Point {
                longitude: BigDecimal::from_str("38.202162").unwrap(),
                latitude: BigDecimal::from_str("51.214457").unwrap(),
            },
        })));
    }
}
