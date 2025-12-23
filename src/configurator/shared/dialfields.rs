use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DialFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_resolver: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,
}
