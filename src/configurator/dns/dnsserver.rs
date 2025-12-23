use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::configurator::tls::TlsConfig;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DialFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_resolver: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsServer {
    Legacy(DnsServerLegacy),
    Local(DnsServerLocal),
    Hosts(DnsServerHosts),
    Tcp(DnsServerTcp),
    Udp(DnsServerUdp),
    Tls(DnsServerTls),
    Quic(DnsServerQuic),
    Https(DnsServerHttps),
    Http3(DnsServerHttp3),
    Dhcp(DnsServerDhcp),
    FakeIp(DnsServerFakeIp),
    Tailscale(DnsServerTailscale),
    Resolved(DnsServerResolved),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerLegacy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerLocal {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_go: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHosts {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HostsPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined: Option<HashMap<String, HostValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HostsPath {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HostValue {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerTcp {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerUdp {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerTls {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerQuic {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHttps {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHttp3 {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerDhcp {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerFakeIp {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerTailscale {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_default_resolvers: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerResolved {
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub service: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_default_resolvers: Option<bool>,
}
