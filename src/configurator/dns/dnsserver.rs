use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::configurator::shared::dialfields::DialFields;
use crate::configurator::shared::tls::TlsConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsServer {
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
pub struct DnsServerLocal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_go: Option<bool>,
}

impl DnsServerLocal {
    pub fn new() -> Self {
        Self {
            server_type: Some("local".to_string()),
            tag: None,
            prefer_go: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHosts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HostsPath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined: Option<HashMap<String, HostValue>>,
}

impl DnsServerHosts {
    pub fn new() -> Self {
        Self {
            server_type: Some("hosts".to_string()),
            tag: None,
            path: None,
            predefined: None,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

impl DnsServerTcp {
    pub fn new() -> Self {
        Self {
            server_type: Some("tcp".to_string()),
            tag: None,
            server: None,
            server_port: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerUdp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

impl DnsServerUdp {
    pub fn new() -> Self {
        Self {
            server_type: Some("udp".to_string()),
            tag: None,
            server: None,
            server_port: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerTls {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

impl DnsServerTls {
    pub fn new() -> Self {
        Self {
            server_type: Some("tls".to_string()),
            tag: None,
            server: None,
            server_port: None,
            tls: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerQuic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

impl DnsServerQuic {
    pub fn new() -> Self {
        Self {
            server_type: Some("quic".to_string()),
            tag: None,
            server: None,
            server_port: None,
            tls: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHttps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
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

impl DnsServerHttps {
    pub fn new() -> Self {
        Self {
            server_type: Some("https".to_string()),
            tag: None,
            server: None,
            server_port: None,
            path: None,
            headers: None,
            tls: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerHttp3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
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

impl DnsServerHttp3 {
    pub fn new() -> Self {
        Self {
            server_type: Some("h3".to_string()),
            tag: None,
            server: None,
            server_port: None,
            path: None,
            headers: None,
            tls: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerDhcp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<DialFields>,
}

impl DnsServerDhcp {
    pub fn new() -> Self {
        Self {
            server_type: Some("dhcp".to_string()),
            tag: None,
            interface: None,
            dial: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerFakeIp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_range: Option<String>,
}

impl DnsServerFakeIp {
    pub fn new() -> Self {
        Self {
            server_type: Some("fakeip".to_string()),
            tag: None,
            inet4_range: None,
            inet6_range: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerTailscale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerTailscale {
    pub fn new() -> Self {
        Self {
            server_type: Some("tailscale".to_string()),
            tag: None,
            endpoint: None,
            accept_default_resolvers: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsServerResolved {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerResolved {
    pub fn new() -> Self {
        Self {
            server_type: Some("resolved".to_string()),
            tag: None,
            service: None,
            accept_default_resolvers: None,
        }
    }
}
