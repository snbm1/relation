use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use macros::auto_skip_none;

use crate::configurator::shared::ListableString;
use crate::configurator::shared::dialfields::DialFields;
use crate::configurator::shared::tls::TlsConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DnsServer {
    #[serde(rename = "local")]
    Local(DnsServerLocal),
    #[serde(rename = "hosts")]
    Hosts(DnsServerHosts),
    #[serde(rename = "tcp")]
    Tcp(DnsServerTcp),
    #[serde(rename = "udp")]
    Udp(DnsServerUdp),
    #[serde(rename = "tls")]
    Tls(DnsServerTls),
    #[serde(rename = "quic")]
    Quic(DnsServerQuic),
    #[serde(rename = "https")]
    Https(DnsServerHttps),
    #[serde(rename = "h3")]
    Http3(DnsServerHttp3),
    #[serde(rename = "dhcp")]
    Dhcp(DnsServerDhcp),
    #[serde(rename = "fakeip")]
    FakeIp(DnsServerFakeIp),
    #[serde(rename = "tailscale")]
    Tailscale(DnsServerTailscale),
    #[serde(rename = "resolved")]
    Resolved(DnsServerResolved),
}

impl DnsServer {
    pub fn get_type(&self) -> &'static str {
        match self {
            DnsServer::Local(_) => "local",
            DnsServer::Hosts(_) => "hosts",
            DnsServer::Tcp(_) => "tcp",
            DnsServer::Udp(_) => "udp",
            DnsServer::Tls(_) => "tls",
            DnsServer::Quic(_) => "quic",
            DnsServer::Https(_) => "https",
            DnsServer::Http3(_) => "h3",
            DnsServer::Dhcp(_) => "dhcp",
            DnsServer::FakeIp(_) => "fakeip",
            DnsServer::Tailscale(_) => "tailscale",
            DnsServer::Resolved(_) => "resolved",
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
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerLocal {
    pub tag: String,
    pub prefer_go: Option<bool>,
}

impl DnsServerLocal {
    pub fn new() -> Self {
        Self {
            tag: "dns-local".to_string(),
            prefer_go: None,
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: name,
            prefer_go: None,
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerHosts {
    pub tag: String,
    pub path: Option<ListableString>,
    pub predefined: Option<HashMap<String, ListableString>>,
}

impl DnsServerHosts {
    pub fn new() -> Self {
        Self {
            tag: "dns-hosts".to_string(),
            ..Default::default()
        }
    }

    pub fn with_tag(name: String) -> Self {
        Self {
            tag: name,
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerTcp {
    pub tag: String,
    pub server: String,
    pub server_port: Option<u16>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTcp {
    pub fn new() -> Self {
        Self {
            tag: "dns-udp".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-udp".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = name;
        self
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerUdp {
    pub tag: String,
    pub server: String,
    pub server_port: Option<u16>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerUdp {
    pub fn new() -> Self {
        Self {
            tag: "dns-udp".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-udp".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn change_tag(mut self, name: String) -> Self {
        self.tag = name;
        self
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerTls {
    pub tag: String,
    pub server: String,
    pub server_port: Option<u16>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerTls {
    pub fn new() -> Self {
        Self {
            tag: "dns-tls".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-tls".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerQuic {
    pub tag: String,
    pub server: String,
    pub server_port: Option<u16>,
    pub tls: Option<TlsConfig>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerQuic {
    pub fn new() -> Self {
        Self {
            tag: "dns-quic".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-quic".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerHttps {
    pub tag: String,
    pub server: String,
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
            tag: "dns-https".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-https".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerHttp3 {
    pub tag: String,
    pub server: String,
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
            tag: "dns-h3".to_string(),
            server: "8.8.8.8".to_string(),
            ..Default::default()
        }
    }

    pub fn with_server(server: String, server_port: Option<u16>) -> Self {
        Self {
            tag: "dns-h3".to_string(),
            server,
            server_port,
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerDhcp {
    pub tag: String,
    pub interface: Option<String>,
    #[serde(flatten)]
    pub dial: Option<DialFields>,
}

impl DnsServerDhcp {
    pub fn new() -> Self {
        Self {
            tag: "dns-dhcp".to_string(),
            ..Default::default()
        }
    }

    pub fn with_interface(interface: &str) -> Self {
        Self {
            tag: "dns-dhcp".to_string(),
            interface: Some(interface.to_string()),
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerFakeIp {
    pub tag: String,
    pub address: String,
    pub inet4_range: Option<String>,
    pub inet6_range: Option<String>,
}

// TODO find out how to set up
impl DnsServerFakeIp {
    pub fn new() -> Self {
        Self {
            tag: "dns-fakeip".to_string(),
            ..Default::default()
        }
    }

    pub fn add_ip4(mut self, address: String) -> Self {
        self.inet4_range = Some(address);
        self
    }

    pub fn add_ip6(mut self, address: String) -> Self {
        self.inet6_range = Some(address);
        self
    }

    /// (String, String) -> (ipv4, ipv6)
    pub fn add_ips(mut self, addresses: (String, String)) -> Self {
        self.inet4_range = Some(addresses.0);
        self.inet6_range = Some(addresses.1);
        self
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerTailscale {
    pub tag: String,
    pub address: String,
    pub endpoint: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerTailscale {
    pub fn new() -> Self {
        Self {
            tag: "dns-tailscale".to_string(),
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}

#[auto_skip_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DnsServerResolved {
    pub tag: String,
    pub service: Option<String>,
    pub accept_default_resolvers: Option<bool>,
}

impl DnsServerResolved {
    pub fn new() -> Self {
        Self {
            tag: "dns-resolved".to_string(),
            ..Default::default()
        }
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}
