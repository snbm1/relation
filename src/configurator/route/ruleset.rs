use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

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

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetInline {
    pub r#type: Option<String>,
    pub tag: Option<String>,
    pub rules: Option<Vec<HeadlessRule>>,
}

impl RuleSetInline {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetLocal {
    pub r#type: Option<String>,
    pub tag: Option<String>,
    pub format: Option<String>,
    pub path: Option<String>,
}

impl RuleSetLocal {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RuleSetRemote {
    pub r#type: Option<String>,
    pub tag: Option<String>,
    pub format: Option<String>,
    pub url: Option<String>,
    pub download_detour: Option<String>,
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

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeadlessDefaultRule {
    pub query_type: Option<Vec<NumOrStr>>,
    pub network: Option<Vec<Network>>,
    pub domain: Option<Vec<String>>,
    pub domain_suffix: Option<Vec<String>>,
    pub domain_keyword: Option<Vec<String>>,
    pub domain_regex: Option<Vec<String>>,
    pub source_ip_cidr: Option<Vec<String>>,
    pub ip_cidr: Option<Vec<String>>,
    pub source_port: Option<Vec<u16>>,
    pub source_port_range: Option<Vec<String>>,
    pub port: Option<Vec<u16>>,
    pub port_range: Option<Vec<String>>,
    pub process_name: Option<Vec<String>>,
    pub process_path: Option<Vec<String>>,
    pub process_path_regex: Option<Vec<String>>,
    pub package_name: Option<Vec<String>>,
    pub default_interface_address: Option<Vec<String>>,
    pub invert: Option<bool>,
}

impl HeadlessDefaultRule {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HeadlessLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: Option<String>,
    pub mode: Option<String>,
    pub rules: Option<Vec<HeadlessRule>>,
    pub invert: Option<bool>,
}

impl HeadlessLogicalRule {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
