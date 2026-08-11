pub mod direct;
pub mod shadowsocks;
pub mod socks;
pub mod subcription;
pub mod vless;
pub mod vmess;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use url::Url;

use direct::DirectConfig;
use shadowsocks::ShadowsocksConfig;
use socks::SocksConfig;
use subcription::Subscription;
use vless::VlessConfig;
use vmess::VmessConfig;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct OutboundConfig {
    pub servers: Vec<Outbound>,
}

impl OutboundConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_server(&mut self, server: Outbound) -> &mut Self {
        self.servers.push(server);
        self
    }

    pub fn add_server_from_url(&mut self, input: &str) -> Result<&mut Self> {
        self.servers.push(Outbound::from_url(input)?);
        Ok(self)
    }

    pub fn add_direct(&mut self) -> &mut Self {
        self.servers.push(Outbound::Direct(DirectConfig::new()));

        self
    }

    pub fn get_ref_by_tag(&self, tag: &str) -> Option<&Outbound> {
        self.servers.iter().find(|outbound| outbound.tag() == tag)
    }

    pub fn get_mut_by_tag(&mut self, tag: &str) -> Option<&mut Outbound> {
        self.servers
            .iter_mut()
            .find(|outbound| outbound.tag() == tag)
    }

    pub fn get_ref_by_type(&self, kind: &str) -> Option<&Outbound> {
        self.servers.iter().find(|outbound| outbound.kind() == kind)
    }

    pub fn get_mut_by_type(&mut self, kind: &str) -> Option<&mut Outbound> {
        self.servers
            .iter_mut()
            .find(|outbound| outbound.kind() == kind)
    }

    pub fn get_tags_except_direct(&self) -> Vec<String> {
        self.servers
            .iter()
            .filter(|outbound| outbound.kind() != "direct")
            .map(|outbound| outbound.tag().to_owned())
            .collect()
    }

    pub fn get_types_except_direct(&self) -> Vec<&'static str> {
        self.servers
            .iter()
            .filter(|outbound| outbound.kind() != "direct")
            .map(Outbound::kind)
            .collect()
    }

    pub fn get_tag_by_type(&self, outbound_type: &str) -> Option<String> {
        self.servers
            .iter()
            .find(|outbound| outbound.kind() == outbound_type)
            .map(|outbound| outbound.tag().to_owned())
    }

    pub fn get_server_addr_by_type(&self, kind: &str) -> Option<&str> {
        self.get_ref_by_type(kind).and_then(Outbound::server)
    }

    pub fn clean(&mut self) -> &mut Self {
        self.servers.clear();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Outbound {
    #[serde(rename = "direct")]
    Direct(DirectConfig),

    #[serde(rename = "vless")]
    Vless(VlessConfig),

    #[serde(rename = "socks")]
    Socks(SocksConfig),

    #[serde(rename = "shadowsocks")]
    Shadowsocks(shadowsocks::ShadowsocksConfig),

    #[serde(rename = "vmess")]
    Vmess(VmessConfig),
}

impl Outbound {
    pub fn from_url(input: &str) -> Result<Self> {
        let url = Url::parse(input).context("Invalid outbound URL")?;

        match url.scheme() {
            "vless" => Ok(Self::Vless(
                VlessConfig::from_url(input).context("Cannot parse VLESS outbound")?,
            )),

            "socks" | "socks4" | "socks4a" | "socks5" | "socks5h" => Ok(Self::Socks(
                SocksConfig::from_url(input).context("Cannot parse SOCKS outbound")?,
            )),

            "http" | "https" => Ok(Subscription::load(input)?.pop().unwrap()),

            "ss" => Ok(Self::Shadowsocks(
                ShadowsocksConfig::from_url(input).context("Cannot parse Shadowsocks outbound")?,
            )),

            "vmess" => Ok(Self::Vmess(
                VmessConfig::from_url(input).context("Cannot parse VMess outbound")?,
            )),

            scheme => {
                bail!("Unsupported outbound scheme '{scheme}'")
            }
        }
    }

    pub fn tag(&self) -> &str {
        match self {
            Self::Direct(cfg) => cfg.tag(),
            Self::Vless(cfg) => cfg.tag(),
            Self::Socks(cfg) => cfg.tag(),
            Self::Shadowsocks(cfg) => cfg.tag(),
            Self::Vmess(cfg) => cfg.tag(),
        }
    }

    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Direct(_) => "direct",
            Self::Vless(_) => "vless",
            Self::Socks(_) => "socks",
            Self::Shadowsocks(_) => "shadowsocks",
            Self::Vmess(_) => "vmess",
        }
    }

    pub fn server(&self) -> Option<&str> {
        match self {
            Self::Direct(_) => None,
            Self::Vless(cfg) => Some(cfg.server()),
            Self::Socks(cfg) => Some(cfg.server()),
            Self::Shadowsocks(cfg) => Some(cfg.server()),
            Self::Vmess(cfg) => Some(cfg.server()),
        }
    }
}
