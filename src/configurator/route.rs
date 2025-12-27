use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Listable<T> {
    One(T),
    Many(Vec<T>),
}

pub type ListableString = Listable<String>;
pub type ListableU16 = Listable<u16>;
pub type ListableU32 = Listable<u32>;

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
    pub default_network_type: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_fallback_network_type: Option<ListableString>,

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
pub struct DefaultRouteRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_geoip: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_is_private: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path_regex: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ListableU32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_expensive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_constrained: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_address: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_address: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface_address: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_by: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<ListableString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ipcidr_match_source: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_match_source: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,

    #[serde(rename = "action")]
    pub actions: Listable<RuleAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogicalRouteRule {
    #[serde(rename = "type")]
    pub rule_type: String,

    pub mode: String,

    pub rules: Vec<RouteRule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,

    #[serde(rename = "action")]
    pub actions: Listable<RuleAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum RuleAction {
    #[serde(rename = "route")]
    Route(RouteAction),

    #[serde(rename = "reject")]
    Reject(RejectAction),

    #[serde(rename = "hijack-dns")]
    HijackDns(HijackDnsAction),

    #[serde(rename = "route-options")]
    RouteOptions(RouteOptionsAction),

    #[serde(rename = "sniff")]
    Sniff(SniffAction),

    #[serde(rename = "resolve")]
    Resolve(ResolveAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteAction {
    pub outbound: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_port: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_network_type: Option<ListableString>,
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
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_drop: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HijackDnsAction {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteOptionsAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_port: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_network_type: Option<ListableString>,
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
pub struct SniffAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniffer: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveAction {
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
pub enum RuleSet {
    Inline(RuleSetInline),
    Local(RuleSetLocal),
    Remote(RuleSetRemote),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetInline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub tag: String,
    pub rules: Vec<HeadlessRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetLocal {
    pub r#type: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSetRemote {
    pub r#type: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadlessRule {
    Default(HeadlessDefaultRule),
    Logical(HeadlessLogicalRule),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadlessDefaultRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_geoip: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path_regex: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ListableU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_expensive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_constrained: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_address: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_address: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface_address: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_by: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ipcidr_match_source: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_match_source: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadlessLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub mode: String,
    pub rules: Vec<HeadlessRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
}
