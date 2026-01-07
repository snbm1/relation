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
            ..Default::default()
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
