use crate::apis::api_caller::ApiCaller;
use crate::apis::flight_provider::flight_provider_request::FlightProviderRequest;
use crate::apis::flight_provider::raw_models::flight_raw::FlightRaw;

pub struct FlightProvider {}

impl FlightProvider {
    pub fn new() -> Self {
        FlightProvider {}
    }

    pub async fn get(&self, request: FlightProviderRequest) {
        let url = Self::generate_url(request);
        let api_caller = ApiCaller::new(url);
        let raw_response = api_caller.get().await.unwrap();
        let response = Self::parse_response(raw_response);
    }

    fn generate_url(request: FlightProviderRequest) -> String {
        format!(
            "https://data-live.flightradar24.com/clickhandler/\
                   ?version=1.5&flight={flight_id}",
            flight_id = request.flight_id
        )
    }

    fn parse_response(raw_response: String) -> Option<FlightRaw> {
        let response: Option<FlightRaw> =  serde_json::from_str(raw_response.as_str()).ok();
        println!("{:?}", response);
        response
    }
}



#[cfg(test)]
mod FlightProviderTests {
    use crate::apis::flight_provider::flight_provider::FlightProvider;
    use crate::apis::flight_provider::flight_provider_request::FlightProviderRequest;
    use tokio_test::block_on;

    #[test]
    fn generate_url_works() {
        // 42.858110, 40.032470
        // 38.202162, 51.214457
        println!(
            "{}",
            FlightProvider::generate_url(FlightProviderRequest::new("25cd0734".into()))
        );
    }

    #[test]
    fn get() {
        // 42.858110, 40.032470
        // 38.202162, 51.214457
        block_on(FlightProvider::new().get(FlightProviderRequest::new("25cd0734".into())));
    }
}
