use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

use crate::configurator::inbound::InboundConfig;
use crate::configurator::outbound::OutboundConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RouteRule {
    Default(DefaultRouteRule),
    Logical(LogicalRouteRule),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    pub action: Option<RuleAction>,
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

    pub fn route_action(outbound: String) -> Self {
        Self {
            action: Some(RuleAction::Route(RouteAction::new(outbound))),
            ..Default::default()
        }
    }

    pub fn route_action_by_type(outbound: &OutboundConfig, outbound_type: &str) -> Self {
        Self {
            action: Some(RuleAction::Route(RouteAction::new(
                outbound
                    .get_tag_by_type(&outbound_type)
                    .expect("[ERROR] cannot find that type"),
            ))),
            ..Default::default()
        }
    }

    pub fn reject_action() -> Self {
        Self {
            action: Some(RuleAction::Reject(RejectAction::new())),
            ..Default::default()
        }
    }

    pub fn sniff_action(timeout: &str) -> Self {
        Self {
            action: Some(RuleAction::Sniff(
                SniffAction::new().set_timeout(timeout.to_string()),
            )),
            ..Default::default()
        }
    }

    pub fn hijack_dns_action() -> Self {
        Self {
            action: Some(RuleAction::HijackDns(HijackDnsAction::new())),
            ..Default::default()
        }
    }

    pub fn add_inbound(mut self, inbound: Vec<&str>) -> Self {
        if let Some(value) = self.inbound.as_mut() {
            let _ = inbound.iter().map(|x| value.push(x.to_string()));
        } else {
            self.inbound = Some(inbound.iter().map(|x| x.to_string()).collect());
        }
        self
    }

    pub fn add_ip_cidr(mut self, ip: &str) -> Self {
        if let Some(value) = self.ip_cidr.as_mut() {
            value.push(ip.to_string());
        } else {
            self.ip_cidr = Some(vec![ip.to_string()]);
        }
        self
    }

    pub fn add_inbound_by_type(mut self, inbound: &InboundConfig, inbound_type: &str) -> Self {
        let tag = inbound
            .get_tag_by_type(inbound_type)
            .expect("[ERROR] cannot find that inbound type");

        self.inbound.get_or_insert_with(Vec::new).push(tag);

        self
    }

    pub fn add_ports(mut self, ports: Vec<u16>) -> Self {
        if let Some(value) = self.port.as_mut() {
            let _ = ports.iter().map(|x| value.push(*x));
        } else {
            self.port = Some(ports);
        }
        self
    }

    pub fn add_port(mut self, port: u16) -> Self {
        if let Some(value) = self.port.as_mut() {
            value.push(port);
        } else {
            self.port = Some(vec![port]);
        }
        self
    }

    pub fn add_protocol(mut self, protocol: &str) -> Self {
        if let Some(value) = self.protocol.as_mut() {
            value.push(protocol.to_string());
        } else {
            self.protocol = Some(vec![protocol.to_string()]);
        }
        self
    }

    pub fn add_ip_is_private(mut self, ip_is_private: bool) -> Self {
        self.ip_is_private = Some(ip_is_private);
        self
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LogicalRouteRule {
    #[serde(rename = "type")]
    pub rule_type: Option<String>,
    pub mode: Option<String>,
    pub rules: Vec<RouteRule>,
    pub invert: Option<bool>,
    #[serde(flatten)]
    pub action: RuleAction,
}

impl LogicalRouteRule {
    pub fn new() -> Self {
        Self {
            rule_type: Some("logical".to_string()),
            action: RuleAction::Reject(RejectAction::new()),
            rules: vec![],
            ..Default::default()
        }
    }

    pub fn or() -> Self {
        Self {
            rule_type: Some("logical".to_string()),
            action: RuleAction::Reject(RejectAction::new()),
            rules: vec![],
            mode: Some("or".to_string()),
            ..Default::default()
        }
    }

    pub fn set_mode(mut self, mode: String) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn add_rule(mut self, rule: DefaultRouteRule) -> Self {
        self.rules.push(RouteRule::Default(rule));
        self
    }

    pub fn set_sniff_action(mut self, timeout: String) -> Self {
        self.action = RuleAction::Sniff(SniffAction::new().set_timeout(timeout));
        self
    }

    pub fn set_hijack_dns_action(mut self) -> Self {
        self.action = RuleAction::HijackDns(HijackDnsAction::new());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RuleAction {
    Route(RouteAction),
    Reject(RejectAction),
    HijackDns(HijackDnsAction),
    Sniff(SniffAction),
    Resolve(ResolveAction),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RejectAction {
    pub action: String,
    pub method: String,
    pub no_drop: Option<bool>,
}

impl RejectAction {
    pub fn new() -> Self {
        Self {
            action: "reject".to_string(),
            method: "default".to_string(),
            ..Default::default()
        }
    }

    pub fn set_method(mut self, method: String) -> Self {
        self.method = method;
        self
    }

    pub fn set_no_drop(mut self, no_drop: bool) -> Self {
        self.no_drop = Some(no_drop);
        self
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HijackDnsAction {
    pub action: String,
}

impl HijackDnsAction {
    pub fn new() -> Self {
        Self {
            action: "hijack-dns".to_string(),
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SniffAction {
    pub action: String,
    pub sniffer: Option<Vec<String>>,
    pub timeout: Option<String>,
}

impl SniffAction {
    pub fn new() -> Self {
        Self {
            action: "sniff".to_string(),
            ..Default::default()
        }
    }
    pub fn set_timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
