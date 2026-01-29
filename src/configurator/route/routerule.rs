use serde::{Deserialize, Serialize};

use crate::configurator::dns::DnsConfig;
use crate::configurator::outbound::OutboundConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteRule {
    Default(DefaultRouteRule),
    Logical(LogicalRouteRule),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultRouteRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path_regex: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Vec<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_match_source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl DefaultRouteRule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn get_dns_tag_by_type(dns: &DnsConfig, dns_type: &str) -> String {
        dns.get_tag_by_type(dns_type).unwrap()
    }

    pub fn set_final_by_type(outbound: &OutboundConfig, outbound_type: &str) -> String {
        outbound.get_tag_by_type(outbound_type).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LogicalRouteRule {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RouteRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_disable_domain_unmapping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_connect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_fragment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_fragment_fallback_delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RejectAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HijackDnsAction {
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SniffAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniffer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResolveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
