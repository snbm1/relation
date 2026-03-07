use crate::configurator::shared::listenfields::ListenFields;
use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

use crate::configurator::shared;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Http_proxy {
    pub enabled: Option<bool>,
    server: Option<String>,
    server_port: Option<u16>,
    bypass_domain: Option<Vec<String>>,
    match_domain: Option<Vec<String>>,
}

impl Http_proxy {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TunConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub tag: Option<String>,
    pub interface_name: Option<String>,
    pub address: Vec<String>,
    pub mtu: Option<u16>,
    pub auto_route: Option<bool>,
    pub iprote2_table_index: Option<u16>,
    pub iproute_rule_index: Option<u16>,
    pub auto_redirect: Option<bool>,
    pub auto_redirect_input_mark: Option<String>,
    pub auto_redirect_output_mark: Option<String>,
    pub auto_redirect_reset_mark: Option<String>,
    pub auto_redirect_nfqueue: Option<u16>,
    pub auto_redirect_iproute2_fallback_rule_index: Option<u16>,
    pub exlude_mptcp: Option<bool>,
    pub loopback_address: Option<Vec<String>>,
    pub strict_route: Option<bool>,
    pub route_address: Option<Vec<String>>,
    pub route_exclude_address: Option<Vec<String>>,
    pub route_address_set: Option<Vec<String>>,
    pub route_exclude_address_set: Option<Vec<String>>,
    pub endpoint_independent_nat: Option<bool>,
    pub udp_timeout: Option<String>,
    pub stack: Option<String>,
    pub include_interface: Option<Vec<String>>,
    pub exclude_interface: Option<Vec<String>>,
    pub include_uid: Option<Vec<u16>>,
    pub include_uid_range: Option<Vec<String>>,
    pub exclude_uid: Option<Vec<u16>>,
    pub exclude_uid_range: Option<Vec<String>>,
    pub include_android_user: Option<Vec<u16>>,
    pub include_package: Option<Vec<String>>,
    pub exclude_package: Option<Vec<String>>,
    pub platform: Option<Http_proxy>,
    #[serde(flatten)]
    pub listen: Option<ListenFields>,
}

impl TunConfig {
    pub fn new() -> Self {
        Self {
            config_type: Some("tun".to_string()),
            tag: Some("inbound-tun".to_string()),
            address: vec![],
            ..Default::default()
        }
    }

    pub fn add_ip(mut self, address: String) -> Self {
        self.address.push(address);
        self
    }

    pub fn add_ip_list(mut self, address: Vec<String>) -> Self {
        for i in address {
            self.address.push(i);
        }
        self
    }

    pub fn set_mtu(mut self, mtu: u16) -> Self {
        self.mtu = Some(mtu);
        self
    }

    pub fn set_auto_route(mut self, route: bool) -> Self {
        self.auto_route = Some(route);
        self
    }

    pub fn set_strict_route(mut self, route: bool) -> Self {
        self.strict_route = Some(route);
        self
    }

    pub fn get_type(&self) -> String {
        self.config_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
