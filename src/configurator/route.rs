use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

pub mod routerule;
pub mod ruleset;
use crate::configurator::dns::DnsConfig;
use crate::configurator::outbound::OutboundConfig;
use crate::configurator::route::routerule::{DefaultRouteRule, LogicalRouteRule, RouteRule};
use crate::configurator::route::ruleset::RuleSet;

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RouteConfig {
    pub rules: Vec<RouteRule>,
    pub rule_set: Option<Vec<RuleSet>>,
    #[serde(rename = "final")]
    pub default: Option<String>,
    pub auto_detect_interface: Option<bool>,
    pub override_android_vpn: Option<bool>,
    pub default_interface: Option<String>,
    pub default_mark: Option<u32>,
    pub default_domain_resolver: Option<String>,
    pub default_network_strategy: Option<String>,
    pub default_network_type: Option<Vec<String>>,
    pub default_fallback_network_type: Option<Vec<String>>,
    pub default_fallback_delay: Option<String>,
}

impl RouteConfig {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            ..Default::default()
        }
    }

    pub fn add_default_rule(&mut self, rule: DefaultRouteRule) -> &mut Self {
        self.rules.push(RouteRule::Default(rule));
        self
    }

    pub fn add_logical_rule(&mut self, rule: LogicalRouteRule) -> &mut Self {
        self.rules.push(RouteRule::Logical(rule));
        self
    }

    pub fn auto_detect_interface(&mut self, value: bool) -> &mut Self {
        self.auto_detect_interface = Some(value);
        self
    }

    pub fn set_final_by_type(
        &mut self,
        outbound: &OutboundConfig,
        outbound_type: &str,
    ) -> &mut Self {
        self.default = outbound.get_tag_by_type(outbound_type);
        self
    }

    pub fn set_default_domain_resolver(&mut self, dns: &DnsConfig, dns_type: &str) -> &mut Self {
        self.default_domain_resolver = dns.get_tag_by_type(dns_type);
        self
    }

    pub fn get_list(&self) -> Vec<RouteRule> {
        self.rules.clone()
    }

    pub fn clean(&mut self) -> &mut Self {
        *self = Self::new();
        self
    }
}
