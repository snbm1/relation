use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransportConfig {
    None,
    WebSocket(WebSocketConfig),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
}

impl WebSocketConfig {
    pub fn new() -> Self {
        WebSocketConfig {
            config_type: Some("ws".to_string()),
            path: None,
            headers: None,
            max_early_data: None,
            early_data_header_name: None,
        }
    }

    pub fn check(&self) -> bool {
        !(self.path.is_none() || self.headers.is_none())
    }
}

