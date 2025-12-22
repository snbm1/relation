use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransportConfig {
    None,
    WebSocket(WebSocketConfig),
    Grpc(GrpcConfig),
    Quic(QuicConfig),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GrpcConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit_without_stream: Option<bool>,
}

impl GrpcConfig {
    pub fn new() -> Self {
        GrpcConfig {
            config_type: Some("gprc".to_string()),
            service_name: None,
            idle_timeout: Some("15s".to_string()),
            ping_timeout: Some("15s".to_string()),
            permit_without_stream: Some(false),
        }
    }

    pub fn check(&self) -> bool {
        !self.service_name.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuicConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
}

impl QuicConfig {
    pub fn new() -> Self {
        QuicConfig {
            config_type: Some("quic".to_string()),
        }
    }

    pub fn check(&self) -> bool {
        !self.config_type.is_none()
    }
}
