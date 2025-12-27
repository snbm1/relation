use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsRuleAction {
    Route(DnsRouteAction),
    RouteOptions(DnsRouteOptionsAction),
    Reject(DnsRejectAction),
    Predefined(DnsPredefinedAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRouteAction {
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

impl DnsRouteAction {
    pub fn new() -> Self {
        Self {
            server: None,
            strategy: None,
            disable_cache: None,
            rewrite_ttl: None,
            client_subnet: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRouteOptionsAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

impl DnsRouteOptionsAction {
    pub fn new() -> Self {
        Self {
            disable_cache: None,
            rewrite_ttl: None,
            client_subnet: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRejectAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_drop: Option<bool>,
}

impl DnsRejectAction {
    pub fn new() -> Self {
        Self {
            method: None,
            no_drop: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsPredefinedAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<String>>,
}

impl DnsPredefinedAction {
    pub fn new() -> Self {
        Self {
            rcode: None,
            answer: None,
            ns: None,
            extra: None,
        }
    }
}
