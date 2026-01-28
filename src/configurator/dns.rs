use serde::{Deserialize, Serialize};
pub mod dnsrule;
pub mod dnsruleaction;
pub mod dnsserver;

use dnsserver::*;

#[derive(Serialize, Deserialize, Default)]
pub struct DnsConfig {
    pub servers: Option<Vec<DnsServer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<String>>,
    #[serde(rename = "final")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_expire: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_capacity: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

impl DnsConfig {
    pub fn new() -> Self {
        DnsConfig {
            servers: Some(vec![]),
            ..Default::default()
        }
    }

    pub fn get_tag_by_type(&self, name: &str) -> Option<String> {
        self.servers
            .as_ref()
            .and_then(|s| s.iter().find(|x| x.server_type() == name))
            .map(|x| x.get_tag())
    }

    pub fn set_final_by_type(&mut self, name: &str) {
        self.default = self.get_tag_by_type(name);
    }

    pub fn add_server(&mut self, server: DnsServer) {
        match &mut self.servers {
            Some(servers) => servers.push(server),
            None => self.servers = Some(vec![server]),
        }
    }

    pub fn remove_server(&mut self, name: &str) {
        if let Some(servers) = self.servers.as_mut() {
            servers.retain(|x| x.server_type() != name);
        }
    }
}

// { "dns": {
//     "servers": [],
//     "rules": [],
//     "final": "",
//     "strategy": "",
//     "disable_cache": false,
//     "disable_expire": false,
//     "independent_cache": false,
//     "cache_capacity": 0,
//     "reverse_mapping": false,
//     "client_subnet": "",
//     "fakeip": {}
//   }
// }

// impl HttpUpdateConfig {
//     pub fn new() -> Self {
//         HttpUpdateConfig {
//             config_type: Some("httpupgrade".to_string()),
//             host: None,
//             path: None,
//             headers: None,
//         }
//     }
//
//     pub fn check(&self) -> bool {
//         !self.path.is_none()
//     }
// }
