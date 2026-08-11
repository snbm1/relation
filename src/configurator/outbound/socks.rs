use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use url::Url;

use macros::auto_skip_none;

use crate::configurator::shared::Network;
use crate::configurator::shared::dialfields::DialFields;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SocksConfig {
    tag: String,
    server: String,
    server_port: u16,
    version: String,
    username: Option<String>,
    password: Option<String>,
    network: Option<Network>,

    #[serde(flatten)]
    dial: Option<DialFields>,
}

impl SocksConfig {
    pub fn new() -> Self {
        Self {
            tag: "outbound-socks".to_string(),
            version: "5".to_string(),
            ..Default::default()
        }
    }

    pub fn with_tag(tag: String) -> Self {
        Self {
            tag,
            version: "5".to_string(),
            ..Default::default()
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }

    pub fn get_server_ip(&self) -> String {
        self.server.clone()
    }

    pub fn validate(&self) -> Result<()> {
        if self.server.trim().is_empty() {
            bail!("SOCKS server is empty");
        }

        if self.server_port == 0 {
            bail!("SOCKS server port cannot be 0");
        }

        match self.version.as_str() {
            "4" | "4a" | "5" => {}
            version => {
                bail!("Unsupported SOCKS version '{version}'");
            }
        }

        Ok(())
    }

    pub fn check(&mut self) -> bool {
        self.validate().is_ok()
    }

    pub fn from_url(input: &str) -> Result<Self> {
        let url = Url::parse(input).context("Invalid SOCKS URL")?;

        let version = match url.scheme().to_ascii_lowercase().as_str() {
            "socks" | "socks5" | "socks5h" => "5",
            "socks4" => "4",
            "socks4a" => "4a",

            scheme => {
                bail!("Unsupported SOCKS URL scheme '{scheme}'");
            }
        }
        .to_string();

        let server = url
            .host_str()
            .context("SOCKS URL does not contain a server")?
            .to_owned();

        if server.trim().is_empty() {
            bail!("SOCKS URL contains an empty server");
        }

        let server_port = url.port().unwrap_or(1080);

        if server_port == 0 {
            bail!("SOCKS server port cannot be 0");
        }

        let username = if url.username().is_empty() {
            None
        } else {
            Some(decode_component(url.username(), "username")?)
        };

        let password = match url.password() {
            Some(password) => Some(decode_component(password, "password")?),
            None => None,
        };

        let tag = match url.fragment() {
            Some(fragment) if !fragment.is_empty() => decode_component(fragment, "tag")?,
            _ => "socks_outbound".to_string(),
        };

        let mut network = None;

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "network" => {
                    network = Some(parse_network(value.as_ref())?);
                }

                // Compatibility: ignore parameters emitted by other clients
                // that do not map to this config.
                _ => {}
            }
        }

        let config = Self {
            tag,
            server,
            server_port,
            version,
            username,
            password,
            network,
            dial: None,
        };

        config.validate()?;

        Ok(config)
    }
}

fn decode_component(value: &str, name: &str) -> Result<String> {
    urlencoding::decode(value)
        .with_context(|| format!("Invalid percent encoding in SOCKS {name}"))
        .map(|value| value.into_owned())
}

fn parse_network(value: &str) -> Result<Network> {
    match value.to_ascii_lowercase().as_str() {
        "tcp" => Ok(Network::Tcp),
        "udp" => Ok(Network::Udp),

        value => bail!("Unsupported network '{value}'"),
    }
}
