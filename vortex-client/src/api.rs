// -- leaked by @azixi0 on github
use std::collections::BTreeMap;

use vortex_engine::session::{HARDWARE_HEADER, TOKEN_HEADER};
use vortex_engine::WEBSITE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRequest {
    pub url: String,
    pub headers: BTreeMap<&'static str, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiClient {
    pub origin: String,
    pub app_token: String,
    pub hardware_id: Option<String>,
}

impl ApiClient {
    pub fn recovered(app_token: impl Into<String>, hardware_id: Option<String>) -> Self {
        Self { origin: WEBSITE.into(), app_token: app_token.into(), hardware_id }
    }

    pub fn get(&self, path: &str) -> ApiRequest {
        let mut headers = BTreeMap::from([(TOKEN_HEADER, self.app_token.clone())]);
        if let Some(hardware_id) = &self.hardware_id {
            headers.insert(HARDWARE_HEADER, hardware_id.clone());
        }
        ApiRequest { url: format!("{}{}", self.origin.trim_end_matches('/'), path), headers }
    }

    pub fn game(&self, game_id: u64) -> ApiRequest {
        self.get(&format!("/api/games/{game_id}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_recovered_auth_headers() {
        let request = ApiClient::recovered("token", Some("machine".into())).get("/api/games/7");
        assert_eq!(request.url, "https://playvortex.io/api/games/7");
        assert_eq!(request.headers[TOKEN_HEADER], "token");
        assert_eq!(request.headers[HARDWARE_HEADER], "machine");
    }
}
