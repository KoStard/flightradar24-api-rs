use reqwest::header::HeaderMap;
use reqwest::header::USER_AGENT;
use reqwest::Client;

// TODO load from https://fake-useragent.herokuapp.com/browsers/0.1.11
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36";

pub struct ApiCaller {
    url: String,
    header_map: HeaderMap,
}

impl ApiCaller {
    pub fn new(url: String) -> Self {
        ApiCaller {
            url,
            header_map: Self::get_header_map(),
        }
    }

    fn get_header_map() -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(USER_AGENT, DEFAULT_USER_AGENT.parse().unwrap());
        header_map
    }

    pub async fn get(&self) -> Result<String, String> {
        // TODO try to use randomized fake user agents
        // TODO better errors handling
        Ok(Client::builder()
            .default_headers(self.header_map.clone())
            .build()
            .map_err(|e| e.to_string())?
            .get(&self.url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?)
    }
}
