use rellib::auto_skip_none;

use serde::{Deserialize, Serialize};

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListenFields {
    pub listen: Option<String>,
    pub listen_port: Option<u16>,
    pub bind_interface: Option<String>,
    pub routing_mark: Option<u16>,
    pub reuse_addr: Option<bool>,
    pub netns: Option<String>,
    pub tcp_fast_open: Option<bool>,
    pub tcp_multi_path: Option<bool>,
    pub disable_tcp_keep_alive: Option<bool>,
    pub tcp_keep_alive: Option<String>,
    pub tcp_keep_alive_interval: Option<String>,
    pub udp_fragment: Option<bool>,
    pub udp_timeout: Option<String>,
    pub detour: Option<String>,
}

impl ListenFields {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn with_listen(addr: Option<String>, port: Option<u16>) -> Self {
        Self {
            listen: addr,
            listen_port: port,
            ..Default::default()
        }
    }

    pub fn with_addr(mut self, addr: String) -> Self {
        self.listen = Some(addr);
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.listen_port = Some(port);
        self
    }
}
