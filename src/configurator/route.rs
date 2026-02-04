use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

pub mod routerule;
pub mod ruleset;
use crate::configurator::outbound::OutboundConfig;
use crate::configurator::route::routerule::{DefaultRouteRule, LogicalRouteRule, RouteRule};
use crate::configurator::route::ruleset::RuleSet;

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RouteConfig {
    pub rules: Option<Vec<RouteRule>>,
    pub rule_set: Option<Vec<RuleSet>>,
    #[serde(rename = "final")]
    pub default: Option<String>,
    pub auto_detect_interface: Option<bool>,
    pub override_android_vpn: Option<bool>,
    pub default_interface: Option<String>,
    pub default_mark: Option<u32>,
    pub default_domain_resolver: Option<DomainResolver>,
    pub default_network_strategy: Option<String>,
    pub default_network_type: Option<Vec<String>>,
    pub default_fallback_network_type: Option<Vec<String>>,
    pub default_fallback_delay: Option<String>,
}

impl RouteConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_default_rule(&mut self, rule: DefaultRouteRule) -> &mut Self {
        if let Some(rules) = self.rules.as_mut() {
            rules.push(RouteRule::Default(rule));
        } else {
            self.rules = Some(vec![RouteRule::Default(rule)]);
        }
        self
    }

    pub fn add_logical_rule(&mut self, rule: LogicalRouteRule) -> &mut Self {
        if let Some(rules) = self.rules.as_mut() {
            rules.push(RouteRule::Logical(rule));
        } else {
            self.rules = Some(vec![RouteRule::Logical(rule)]);
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

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResolveOptionsNoAction {
    pub server: Option<String>,
    pub strategy: Option<String>,
    pub disable_cache: Option<bool>,
    pub rewrite_ttl: Option<u32>,
    pub client_subnet: Option<String>,
}

impl ResolveOptionsNoAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
