use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DialFields {
    pub detour: Option<String>,
    pub domain_resolver: Option<serde_json::Value>,
    pub bind_interface: Option<String>,
    pub inet4_bind_address: Option<String>,
    pub inet6_bind_address: Option<String>,
    pub routing_mark: Option<u32>,
    pub reuse_addr: Option<bool>,
    pub connect_timeout: Option<String>,
    pub tcp_fast_open: Option<bool>,
    pub tcp_multi_path: Option<bool>,
    pub udp_fragment: Option<bool>,
}

impl DialFields {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
