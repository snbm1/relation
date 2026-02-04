use crate::configurator::dns::dnsruleaction::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rellib::auto_skip_none;

use crate::configurator::shared::Listable;
use crate::configurator::shared::ListableString;
use crate::configurator::shared::ListableU16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryType {
    Name(String),
    Code(u16),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterfaceAddress {
    Map(HashMap<String, Vec<String>>),
    List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsRule {
    Default(DnsDefaultRule),
    Logical(DnsLogicalRule),
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsDefaultRule {
    pub inbound: Option<ListableString>,
    pub ip_version: Option<u8>,
    pub query_type: Option<Listable<QueryType>>,
    pub network: Option<String>,
    pub auth_user: Option<ListableString>,
    pub protocol: Option<ListableString>,
    pub domain: Option<ListableString>,
    pub domain_suffix: Option<ListableString>,
    pub domain_keyword: Option<ListableString>,
    pub domain_regex: Option<ListableString>,
    pub source_ip_cidr: Option<ListableString>,
    pub source_ip_is_private: Option<bool>,
    pub source_port: Option<ListableU16>,
    pub source_port_range: Option<ListableString>,
    pub port: Option<ListableU16>,
    pub port_range: Option<ListableString>,
    pub process_name: Option<ListableString>,
    pub process_path: Option<ListableString>,
    pub process_path_regex: Option<ListableString>,
    pub package_name: Option<ListableString>,
    pub user: Option<ListableString>,
    pub user_id: Option<Listable<u32>>,
    pub clash_mode: Option<String>,
    pub network_type: Option<ListableString>,
    pub network_is_expensive: Option<bool>,
    pub network_is_constrained: Option<bool>,
    pub interface_address: Option<HashMap<String, Vec<String>>>,
    pub network_interface_address: Option<HashMap<String, Vec<String>>>,
    pub default_interface_address: Option<Vec<String>>,
    pub wifi_ssid: Option<ListableString>,
    pub wifi_bssid: Option<ListableString>,
    pub rule_set: Option<ListableString>,
    pub rule_set_ip_cidr_match_source: Option<bool>,
    pub invert: Option<bool>,
    #[serde(flatten)]
    pub action: Option<DnsRuleAction>,
    pub ip_cidr: Option<ListableString>,
    pub ip_is_private: Option<bool>,
    pub rule_set_ip_cidr_accept_empty: Option<bool>,
    pub ip_accept_any: Option<bool>,
}

impl DnsDefaultRule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: Option<String>,
    pub mode: Option<String>,
    pub rules: Option<Vec<DnsRule>>,
    pub invert: Option<bool>,
    #[serde(flatten)]
    pub action: Option<DnsRuleAction>,
}

impl DnsLogicalRule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
