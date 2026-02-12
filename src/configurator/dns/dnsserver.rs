use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;

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
    pub address: String,
    pub prefer_go: Option<bool>,
}

impl DnsServerLocal {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-local".to_string()),
            address: "local".to_string(),
            prefer_go: None,
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            address: "local".to_string(),
            prefer_go: None,
        }
    }

    pub fn get_type(&self) -> String {
        "local".to_string()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerHosts {
    pub tag: Option<String>,
    pub address: String,
    pub path: Option<ListableString>,
    pub predefined: Option<HashMap<String, ListableString>>,
}

impl DnsServerHosts {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-hosts".to_string()),
            address: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: Some(name),
            address: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        "hosts".to_string()
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
    pub address: String,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTcp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tcp".to_string()),
            address: "tcp://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-tcp".to_string()),
                address: format!("tcp://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-tcp".to_string()),
                address: format!("tcp://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = Some(name);
        self
    }

    pub fn get_type(&self) -> String {
        "tcp".to_string()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerUdp {
    pub tag: Option<String>,
    pub address: String,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerUdp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-udp".to_string()),
            address: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-udp".to_string()),
                address: format!("{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-udp".to_string()),
                address: format!("{}",server),
                ..Default::default()
            }
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = Some(name);
        self
    }

    pub fn get_type(&self) -> String {
        "udp".to_string()
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
    pub address: String,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTls {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tls".to_string()),
            address: "tls://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-tls".to_string()),
                address: format!("tls://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-tls".to_string()),
                address: format!("tls://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn get_type(&self) -> String {
        "tls".to_string()
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
    pub address: String,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerQuic {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-quic".to_string()),
            address: "quic://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-quic".to_string()),
                address: format!("quic://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-quic".to_string()),
                address: format!("quic://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn get_type(&self) -> String {
        "quic".to_string()
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
    pub address: String,
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
            address: "https://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-https".to_string()),
                address: format!("https://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-https".to_string()),
                address: format!("https://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn get_type(&self) -> String {
        "https".to_string()
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
    pub address: String,
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
            address: "h3://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-h3".to_string()),
                address: format!("h3://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-h3".to_string()),
                address: format!("h3://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn get_type(&self) -> String {
        "h3".to_string()
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
    pub address: String,
    pub interface: Option<String>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerDhcp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-dhcp".to_string()),
            address: "dhcp://8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        if let Some(y) = server_port {
            Self {
                tag: Some("dns-dhcp".to_string()),
                address: format!("dhcp://{}:{}",server,y),
                ..Default::default()
            }
        } else {
            Self {
                tag: Some("dns-dhcp".to_string()),
                address: format!("dhcp://{}",server),
                ..Default::default()
            }
        }
    }

    pub fn get_type(&self) -> String {
        "dhcp".to_string()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerFakeIp {
    pub tag: Option<String>,
    pub address: String,
    pub inet4_range: Option<String>,
    pub inet6_range: Option<String>,
}

impl DnsServerFakeIp {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-fakeip".to_string()),
            // TODO find out how to set up
            address: "fakeip".to_string(),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        "fakeip".to_string()
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
    pub address: String,
    pub endpoint: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerTailscale {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-tailscale".to_string()),
            address: "tailscale".to_string(),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        "tailscale".to_string()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnsServerResolved {
    pub tag: Option<String>,
    pub address: String,
    pub service: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerResolved {
    pub fn new() -> Self {
        Self {
            tag: Some("dns-resolved".to_string()),
            address: "resolved".to_string(),
            ..Default::default()
        }
    }

    pub fn get_type(&self) -> String {
        "resolved".to_string()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
