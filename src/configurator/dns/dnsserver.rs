use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::configurator::shared::ListableString;
use crate::configurator::shared::dialfields::DialFields;
use crate::configurator::shared::tls::TlsConfig;

pub trait DnsServerVariansTrait {
    fn new() -> Self;
    fn with_name(name: String) -> Self;
    fn server_type(&self) -> String;
    fn get_tag(&self) -> String;
}

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

impl DnsServer {
    pub fn server_type(&self) -> String {
        match self {
            DnsServer::Local(s) => s.server_type(),
            DnsServer::Hosts(s) => s.server_type(),
            DnsServer::Tcp(s) => s.server_type(),
            DnsServer::Udp(s) => s.server_type(),
            DnsServer::Tls(s) => s.server_type(),
            DnsServer::Quic(s) => s.server_type(),
            DnsServer::Https(s) => s.server_type(),
            DnsServer::Http3(s) => s.server_type(),
            DnsServer::Dhcp(s) => s.server_type(),
            DnsServer::FakeIp(s) => s.server_type(),
            DnsServer::Tailscale(s) => s.server_type(),
            DnsServer::Resolved(s) => s.server_type(),
        }
    }
    pub fn get_tag(&self) -> String {
        match self {
            DnsServer::Local(s) => s.get_tag(),
            DnsServer::Hosts(s) => s.get_tag(),
            DnsServer::Tcp(s) => s.get_tag(),
            DnsServer::Udp(s) => s.get_tag(),
            DnsServer::Tls(s) => s.get_tag(),
            DnsServer::Quic(s) => s.get_tag(),
            DnsServer::Https(s) => s.get_tag(),
            DnsServer::Http3(s) => s.get_tag(),
            DnsServer::Dhcp(s) => s.get_tag(),
            DnsServer::FakeIp(s) => s.get_tag(),
            DnsServer::Tailscale(s) => s.get_tag(),
            DnsServer::Resolved(s) => s.get_tag(),
        }
    }
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

impl DnsServerVariansTrait for DnsServerLocal {
    fn new() -> Self {
        Self {
            tag: Some("dns-local".to_string()),
            server_type: Some("local".to_string()),
            prefer_go: None,
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("local".to_string()),
            prefer_go: None,
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerHosts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<ListableString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined: Option<HashMap<String, ListableString>>,
}

impl DnsServerVariansTrait for DnsServerHosts {
    fn new() -> Self {
        Self {
            tag: Some("dns-hosts".to_string()),
            server_type: Some("hosts".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("hosts".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerTcp {
    fn new() -> Self {
        Self {
            tag: Some("dns-tcp".to_string()),
            server_type: Some("tcp".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tcp".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerUdp {
    fn new() -> Self {
        Self {
            tag: Some("dns-udp".to_string()),
            server_type: Some("udp".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("udp".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerTls {
    fn new() -> Self {
        Self {
            tag: Some("dns-tls".to_string()),
            server_type: Some("tls".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tls".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerQuic {
    fn new() -> Self {
        Self {
            tag: Some("dns-quic".to_string()),
            server_type: Some("quic".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("quic".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerHttps {
    fn new() -> Self {
        Self {
            tag: Some("dns-https".to_string()),
            server_type: Some("https".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("https".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerHttp3 {
    fn new() -> Self {
        Self {
            tag: Some("dns-h3".to_string()),
            server_type: Some("h3".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("h3".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerDhcp {
    fn new() -> Self {
        Self {
            tag: Some("dns-dhcp".to_string()),
            server_type: Some("dhcp".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("dhcp".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerFakeIp {
    fn new() -> Self {
        Self {
            tag: Some("dns-fakeip".to_string()),
            server_type: Some("fakeip".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("fakeip".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerTailscale {
    fn new() -> Self {
        Self {
            tag: Some("dns-tailscale".to_string()),
            server_type: Some("tailscale".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tailscale".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl DnsServerVariansTrait for DnsServerResolved {
    fn new() -> Self {
        Self {
            tag: Some("dns-resolved".to_string()),
            server_type: Some("resolved".to_string()),
            ..Default::default()
        }
    }

    fn with_name(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("resolved".to_string()),
            ..Default::default()
        }
    }

    fn server_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
