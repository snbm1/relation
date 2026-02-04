use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use rellib::auto_skip_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TransportConfig {
    None,
    Tcp,
    WebSocket(WebSocketConfig),
    Grpc(GrpcConfig),
    Quic(QuicConfig),
    Http(HttpConfig),
    HttpUpdate(HttpUpdateConfig),
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WebSocketConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub max_early_data: Option<u16>,
    pub early_data_header_name: Option<String>,
}

impl WebSocketConfig {
    pub fn new() -> Self {
        WebSocketConfig {
            config_type: Some("ws".to_string()),
            ..Default::default()
        }
    }

    pub fn check(&self) -> bool {
        !(self.path.is_none() || self.headers.is_none())
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GrpcConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub service_name: Option<String>,
    pub idle_timeout: Option<String>,
    pub ping_timeout: Option<String>,
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

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
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

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HttpConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub host: Option<Vec<String>>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub idle_timeout: Option<String>,
    pub ping_timeout: Option<String>,
}

impl HttpConfig {
    pub fn new() -> Self {
        HttpConfig {
            config_type: Some("http".to_string()),
            ..Default::default()
        }
    }

    pub fn check(&self) -> bool {
        !self.path.is_none()
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HttpUpdateConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub host: Option<Vec<String>>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl HttpUpdateConfig {
    pub fn new() -> Self {
        HttpUpdateConfig {
            config_type: Some("httpupgrade".to_string()),
            ..Default::default()
        }
    }

    pub fn check(&self) -> bool {
        !self.path.is_none()
    }
}
