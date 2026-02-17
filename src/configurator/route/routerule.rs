use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

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
    #[serde(flatten)]
    pub action: RuleAction,
}

impl DefaultRouteRule {
    pub fn new() -> Self {
        Self {
            action: RuleAction::Route(RouteAction::new("".to_string())),
            ..Default::default()
        }
    }

    pub fn get_inbound_tag_by_type(inbound: &InboundConfig, inbound_type: &str) -> String {
        inbound.get_tag_by_type(inbound_type).unwrap()
    }

    pub fn set_final_by_type(outbound: &OutboundConfig, outbound_type: &str) -> String {
        outbound.get_tag_by_type(outbound_type).unwrap()
    }

    pub fn route_action(outbound: String) -> Self {
        Self {
            action: RuleAction::Route(RouteAction::new(outbound)),
            ..Default::default()
        }
    }

    pub fn route_action_by_type(outbound: &OutboundConfig, outbound_type: String) -> Self {
        Self {
            action: RuleAction::Route(RouteAction::new(
                outbound
                    .get_tag_by_type(&outbound_type)
                    .expect("[ERROR] cannot find that type"),
            )),
            ..Default::default()
        }
    }

    pub fn reject_action() -> Self {
        Self {
            action: RuleAction::Reject(RejectAction::new()),
            ..Default::default()
        }
    }

    pub fn add_inbound(mut self, inbound: Vec<String>) -> Self {
        if let Some(value) = self.inbound.as_mut() {
            let _ = inbound.iter().map(|x| value.push(x.clone()));
        } else {
            self.inbound = Some(inbound);
        }
        self
    }

    pub fn add_port(mut self, port: Vec<u16>) -> Self {
        if let Some(value) = self.port.as_mut() {
            let _ = port.iter().map(|x| value.push(*x));
        } else {
            self.port = Some(port);
        }
        self
    }

    pub fn add_ip_is_private(mut self, ip_is_private: bool) -> Self {
        self.ip_is_private = Some(ip_is_private);
        self
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
    Route(RouteAction),
    Reject(RejectAction),
    HijackDns(HijackDnsAction),
    Sniff(SniffAction),
    Resolve(ResolveAction),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteAction {
    pub outbound: String,
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
    pub fn new(outbound: String) -> Self {
        Self {
            outbound,
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RejectAction {
    pub method: String,
    pub no_drop: Option<bool>,
}

impl RejectAction {
    pub fn new() -> Self {
        Self {
            method: "default".to_string(),
            ..Default::default()
        }
    }

    pub fn with_method(method: String) -> Self {
        Self {
            method,
            ..Default::default()
        }
    }

    pub fn set_no_drop(mut self, no_drop: bool) -> Self {
        self.no_drop = Some(no_drop);
        self
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HijackDnsAction {}

impl HijackDnsAction {
    pub fn new() -> Self {
        Self {}
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SniffAction {
    pub sniffer: Vec<String>,
    pub timeout: Option<String>,
}

impl SniffAction {
    pub fn new() -> Self {
        Self {
            sniffer: vec![],
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResolveAction {
    pub server: String,
    pub strategy: String,
    pub disable_cache: Option<bool>,
    pub rewrite_ttl: Option<u32>,
    pub client_subnet: Option<String>,
}

impl ResolveAction {
    pub fn new() -> Self {
        Self {
            server: "".to_string(),
            strategy: "".to_string(),
            ..Default::default()
        }
    }
}

impl Default for RuleAction {
    fn default() -> Self {
        RuleAction::Route(RouteAction::new("".to_string()))
    }
}
