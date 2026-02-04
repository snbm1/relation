use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsRuleAction {
    Route(DnsRouteAction),
    RouteOptions(DnsRouteOptionsAction),
    Reject(DnsRejectAction),
    Predefined(DnsPredefinedAction),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsRouteAction {
    pub server: Option<String>,
    pub strategy: Option<String>,
    pub disable_cache: Option<bool>,
    pub rewrite_ttl: Option<u32>,
    pub client_subnet: Option<String>,
}

impl DnsRouteAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsRouteOptionsAction {
    pub disable_cache: Option<bool>,
    pub rewrite_ttl: Option<u32>,
    pub client_subnet: Option<String>,
}

impl DnsRouteOptionsAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsRejectAction {
    pub method: Option<String>,
    pub no_drop: Option<bool>,
}

impl DnsRejectAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsPredefinedAction {
    pub rcode: Option<String>,
    pub answer: Option<Vec<String>>,
    pub ns: Option<Vec<String>>,
    pub extra: Option<Vec<String>>,
}

impl DnsPredefinedAction {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
