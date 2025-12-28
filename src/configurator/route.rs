use serde::{Deserialize, Serialize};

mod rule_set;
use crate::configurator::route::rule_set::RuleSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RouteRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<Vec<RuleSet>>,
    #[serde(rename = "final")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_detect_interface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_android_vpn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_domain_resolver: Option<DomainResolver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_network_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_network_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fallback_network_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fallback_delay: Option<String>,
}

impl RouteConfig {
    pub fn new() -> Self {
        RouteConfig {
            rules: None,
            rule_set: None,
            default: None,
            auto_detect_interface: None,
            override_android_vpn: None,
            default_interface: None,
            default_mark: None,
            default_domain_resolver: None,
            default_network_strategy: None,
            default_network_type: None,
            default_fallback_network_type: None,
            default_fallback_delay: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainResolver {
    Tag(String),
    Options(ResolveOptionsNoAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveOptionsNoAction {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteRule {
    Default(DefaultRouteRule),
    Logical(LogicalRouteRule),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumOrStr {
    Num(u16),
    Str(String),
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleAction {
    #[serde(rename = "route")]
    RouteOptions(RouteAction),
    #[serde(rename = "reject")]
    Reject(RejectAction),
    #[serde(rename = "hijack-dns")]
    HijackDns(HijackDnsAction),
    #[serde(rename = "sniff")]
    Sniff(SniffAction),
    #[serde(rename = "resolve")]
    Resolve(ResolveAction),
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RejectAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_drop: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HijackDnsAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SniffAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniffer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
