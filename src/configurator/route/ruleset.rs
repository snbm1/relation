use serde::{Deserialize, Serialize};

use crate::configurator::shared::Network;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumOrStr {
    Num(u16),
    Str(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuleSet {
    Inline(RuleSetInline),
    Local(RuleSetLocal),
    Remote(RuleSetRemote),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetInline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HeadlessRule>>,
}

impl RuleSetInline {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetLocal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl RuleSetLocal {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetRemote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<String>,
}

impl RuleSetRemote {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadlessRule {
    Default(HeadlessDefaultRule),
    Logical(HeadlessLogicalRule),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeadlessDefaultRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<Vec<NumOrStr>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<Network>>,
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
    pub ip_cidr: Option<Vec<String>>,
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
    pub default_interface_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
}

impl HeadlessDefaultRule {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeadlessLogicalRule {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HeadlessRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
}

impl HeadlessLogicalRule {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
