use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

use crate::configurator::inbound::InboundConfig;
use crate::configurator::outbound::OutboundConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteRule {
    Default(DefaultRouteRule),
    Logical(LogicalRouteRule),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultRouteRule {
    pub inbound: Option<Vec<String>>,
    pub ip_version: Option<u8>,
    pub auth_user: Option<Vec<String>>,
    pub protocol: Option<Vec<String>>,
    pub client: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub domain: Option<Vec<String>>,
    pub domain_suffix: Option<Vec<String>>,
    pub domain_keyword: Option<Vec<String>>,
    pub domain_regex: Option<Vec<String>>,
    pub source_ip_cidr: Option<Vec<String>>,
    pub ip_is_private: Option<bool>,
    pub ip_cidr: Option<Vec<String>>,
    pub source_ip_is_private: Option<bool>,
    pub source_port: Option<Vec<u16>>,
    pub source_port_range: Option<Vec<String>>,
    pub port: Option<Vec<u16>>,
    pub port_range: Option<Vec<String>>,
    pub process_name: Option<Vec<String>>,
    pub process_path: Option<Vec<String>>,
    pub process_path_regex: Option<Vec<String>>,
    pub package_name: Option<Vec<String>>,
    pub user: Option<Vec<String>>,
    pub user_id: Option<Vec<u16>>,
    pub clash_mode: Option<String>,
    pub rule_set: Option<Vec<String>>,
    pub rule_set_ip_cidr_match_source: Option<Vec<String>>,
    pub invert: Option<bool>,
    pub action: Option<String>,
}

impl DefaultRouteRule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_inbound_tag_by_type(inbound: &InboundConfig, inbound_type: &str) -> String {
        inbound.get_tag_by_type(inbound_type).unwrap()
    }

    pub fn set_final_by_type(outbound: &OutboundConfig, outbound_type: &str) -> String {
        outbound.get_tag_by_type(outbound_type).unwrap()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LogicalRouteRule {
    #[serde(rename = "type")]
    pub rule_type: Option<String>,
    pub mode: Option<String>,
    pub rules: Option<Vec<RouteRule>>,
    pub invert: Option<bool>,
    #[serde(rename = "action")]
    #[serde(flatten)]
    pub action: Option<RuleAction>,
}

impl LogicalRouteRule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleAction {
    RouteOptions(RouteAction),
    Reject(RejectAction),
    HijackDns(HijackDnsAction),
    Sniff(SniffAction),
    Resolve(ResolveAction),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteAction {
    pub action: Option<String>,
    pub outbound: Option<String>,
    pub override_address: Option<String>,
    pub override_port: Option<u16>,
    pub network_strategy: Option<String>,
    pub fallback_delay: Option<String>,
    pub udp_disable_domain_unmapping: Option<bool>,
    pub udp_connect: Option<bool>,
    pub udp_timeout: Option<String>,
    pub tls_fragment: Option<bool>,
    pub tls_fragment_fallback_delay: Option<String>,
    pub tls_record_fragment: Option<String>,
}

impl RouteAction {
    pub fn new() -> Self {
        Self {
            action: Some("route".to_string()),
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RejectAction {
    pub action: Option<String>,
    pub method: Option<String>,
    pub no_drop: Option<bool>,
}

impl RejectAction {
    pub fn new() -> Self {
        Self {
            action: Some("reject".to_string()),
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HijackDnsAction {
    pub action: Option<String>,
}

impl HijackDnsAction {
    pub fn new() -> Self {
        Self {
            action: Some("hijack-dns".to_string()),
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SniffAction {
    pub action: Option<String>,
    pub sniffer: Option<Vec<String>>,
    pub timeout: Option<String>,
}

impl SniffAction {
    pub fn new() -> Self {
        Self {
            action: Some("sniff".to_string()),
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResolveAction {
    pub action: Option<String>,
    pub server: Option<String>,
    pub strategy: Option<String>,
    pub disable_cache: Option<bool>,
    pub rewrite_ttl: Option<u32>,
    pub client_subnet: Option<String>,
}

impl ResolveAction {
    pub fn new() -> Self {
        Self {
            action: Some("resolve".to_string()),
            ..Default::default()
        }
    }
}
