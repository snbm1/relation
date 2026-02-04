use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use rellib::auto_skip_none;

use crate::configurator::shared::ListableString;
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

impl DnsServer {
    pub fn get_type(&self) -> String {
        match self {
            DnsServer::Local(s) => s.get_type(),
            DnsServer::Hosts(s) => s.get_type(),
            DnsServer::Tcp(s) => s.get_type(),
            DnsServer::Udp(s) => s.get_type(),
            DnsServer::Tls(s) => s.get_type(),
            DnsServer::Quic(s) => s.get_type(),
            DnsServer::Https(s) => s.get_type(),
            DnsServer::Http3(s) => s.get_type(),
            DnsServer::Dhcp(s) => s.get_type(),
            DnsServer::FakeIp(s) => s.get_type(),
            DnsServer::Tailscale(s) => s.get_type(),
            DnsServer::Resolved(s) => s.get_type(),
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

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerLocal {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub prefer_go: Option<bool>,
}

impl DnsServerLocal {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-local".to_string()),
            server_type: Some("local".to_string()),
            prefer_go: None,
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("local".to_string()),
            prefer_go: None,
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerHosts {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub path: Option<ListableString>,
    pub predefined: Option<HashMap<String, ListableString>>,
}

impl DnsServerHosts {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-hosts".to_string()),
            server_type: Some("hosts".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("hosts".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerTcp {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTcp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tcp".to_string()),
            server_type: Some("tcp".to_string()),
            ..Default::default()
        }
    }

    pub fn with_server(server: Option<String>, server_port: Option<u16>) -> Self {
        Self {
            tag: Some("dns-tcp".to_string()),
            server_type: Some("tcp".to_string()),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tcp".to_string()),
            ..Default::default()
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = Some(name);
        self
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerUdp {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerUdp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-udp".to_string()),
            server_type: Some("udp".to_string()),
            ..Default::default()
        }
    }

    pub fn with_server(server: Option<String>, server_port: Option<u16>) -> Self {
        Self {
            tag: Some("dns-udp".to_string()),
            server_type: Some("udp".to_string()),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("udp".to_string()),
            ..Default::default()
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = Some(name);
        self
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerTls {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTls {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tls".to_string()),
            server_type: Some("tls".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tls".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerQuic {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerQuic {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-quic".to_string()),
            server_type: Some("quic".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("quic".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerHttps {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerHttps {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-https".to_string()),
            server_type: Some("https".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("https".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerHttp3 {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub server: Option<String>,
    pub server_port: Option<u16>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerHttp3 {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-h3".to_string()),
            server_type: Some("h3".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("h3".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerDhcp {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub interface: Option<String>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerDhcp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-dhcp".to_string()),
            server_type: Some("dhcp".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("dhcp".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerFakeIp {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub inet4_range: Option<String>,
    pub inet6_range: Option<String>,
}

impl DnsServerFakeIp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-fakeip".to_string()),
            server_type: Some("fakeip".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("fakeip".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerTailscale {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub endpoint: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerTailscale {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tailscale".to_string()),
            server_type: Some("tailscale".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("tailscale".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerResolved {
    pub tag: Option<String>,
    #[serde(rename = "type")]
    pub server_type: Option<String>,
    pub service: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerResolved {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-resolved".to_string()),
            server_type: Some("resolved".to_string()),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            server_type: Some("resolved".to_string()),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        self.server_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
