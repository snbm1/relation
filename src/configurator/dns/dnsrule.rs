use crate::configurator::dns::dnsruleaction::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Listable<T> {
    One(T),
    Many(Vec<T>),
}

pub type ListableString = Listable<String>;
pub type ListableU16 = Listable<u16>;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    and,
    or,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsDefaultRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<Listable<QueryType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<ListableU16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path_regex: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Listable<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_expensive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_is_constrained: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_address: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_address: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_match_source: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DnsRuleAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_ip_cidr_accept_empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_accept_any: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: String,
    pub mode: String,
    pub rules: Vec<DnsRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(flatten)]
    pub action: DnsRuleAction,
}
