use serde::{Deserialize, Serialize};

pub mod routerule;
pub mod ruleset;
use crate::configurator::outbound::OutboundConfig;
use crate::configurator::route::routerule::RouteRule;
use crate::configurator::route::ruleset::RuleSet;

#[derive(Debug, Serialize, Deserialize, Default)]
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
        Self {
            ..Default::default()
        }
    }

    pub fn add_rule(&mut self, rule: RouteRule) -> &mut Self {
        if let Some(rules) = self.rules.as_mut() {
            rules.push(rule);
        } else {
            self.rules = Some(vec![rule]);
        }
        self
    }

    pub fn auto_detect_interface(&mut self, value: bool) -> &mut Self {
        self.auto_detect_interface = Some(value);
        self
    }

    pub fn set_final_by_type(&mut self, outbound: &OutboundConfig, outbound_type: &str) -> &mut Self {
        self.default = Some(outbound.get_tag_by_type(outbound_type).unwrap());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainResolver {
    Tag(String),
    Options(ResolveOptionsNoAction),
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl ResolveOptionsNoAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
