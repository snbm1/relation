use anyhow::{Context, Result, bail};
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD},
};
use serde::{Deserialize, Serialize};
use url::Url;

use macros::auto_skip_none;

use crate::configurator::shared::{
    Network, dialfields::DialFields, multiplex::*, tls::*, transport::*,
};

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ShadowsocksConfig {
    tag: String,
    server: String,
    server_port: u16,
    method: String,
    password: String,

    network: Option<Network>,
    plugin: Option<String>,
    plugin_opts: Option<String>,

    #[serde(flatten)]
    dial: Option<DialFields>,
}

impl ShadowsocksConfig {
    pub fn new() -> Self {
        Self {
            tag: "outbound-shadowsocks".to_string(),
            ..Default::default()
        }
    }

    pub fn with_tag(tag: String) -> Self {
        Self {
            tag,
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
            bail!("Shadowsocks server is empty");
        }

        if self.server_port == 0 {
            bail!("Shadowsocks server port cannot be 0");
        }

        if self.method.trim().is_empty() {
            bail!("Shadowsocks method is empty");
        }

        if self.password.is_empty() {
            bail!("Shadowsocks password is empty");
        }

        if let Some(plugin) = &self.plugin {
            match plugin.as_str() {
                "obfs-local" | "v2ray-plugin" => {}

                plugin => {
                    bail!("Unsupported Shadowsocks SIP003 plugin '{plugin}'");
                }
            }
        }

        Ok(())
    }

    pub fn check(&mut self) -> bool {
        self.validate().is_ok()
    }

    pub fn from_url(input: &str) -> Result<Self> {
        let parsed = ParsedShadowsocksUrl::parse(input)?;
        parsed.build()
    }
}

#[derive(Default, Debug)]
struct ParsedShadowsocksUrl {
    server: String,
    server_port: u16,

    method: String,
    password: String,

    tag: String,

    network: Option<Network>,
    plugin: Option<String>,
    plugin_opts: Option<String>,
}

impl ParsedShadowsocksUrl {
    fn parse(input: &str) -> Result<Self> {
        if !input.starts_with("ss://") {
            bail!("Invalid Shadowsocks URL: expected ss://");
        }

        let without_scheme = &input["ss://".len()..];

        let (without_fragment, fragment) = match without_scheme.split_once('#') {
            Some((left, right)) => (left, Some(right)),
            None => (without_scheme, None),
        };
        let (authority, query) = match without_fragment.split_once('?') {
            Some((left, right)) => (left, Some(right)),
            None => (without_fragment, None),
        };

        let tag = match fragment {
            Some(value) if !value.is_empty() => decode_component(value, "tag")?,

            _ => "shadowsocks_outbound".to_string(),
        };

        let (method, password, server, server_port) = parse_ss_authority(authority)?;

        let mut result = Self {
            server,
            server_port,
            method,
            password,
            tag,
            ..Default::default()
        };

        if let Some(query) = query {
            result.parse_query(query)?;
        }

        result.validate()?;

        Ok(result)
    }

    fn parse_query(&mut self, query: &str) -> Result<()> {
        let fake = format!("https://localhost/?{query}");

        let url = Url::parse(&fake).context("Invalid Shadowsocks query parameters")?;

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "plugin" => {
                    self.parse_plugin(value.as_ref())?;
                }

                "network" => {
                    self.network = Some(parse_network(value.as_ref())?);
                }

                _ => {}
            }
        }

        Ok(())
    }

    fn parse_plugin(&mut self, value: &str) -> Result<()> {
        if value.is_empty() {
            return Ok(());
        }
        let mut parts = value.splitn(2, ';');

        let plugin = parts.next().unwrap_or_default().trim();

        if plugin.is_empty() {
            bail!("Empty Shadowsocks plugin name");
        }

        match plugin {
            "obfs-local" | "v2ray-plugin" => {}

            _ => {
                bail!("Unsupported Shadowsocks SIP003 plugin '{plugin}'");
            }
        }

        self.plugin = Some(plugin.to_string());

        if let Some(options) = parts.next() {
            if !options.is_empty() {
                self.plugin_opts = Some(options.to_string());
            }
        }

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.server.trim().is_empty() {
            bail!("Shadowsocks server is empty");
        }

        if self.server_port == 0 {
            bail!("Shadowsocks server port cannot be 0");
        }

        if self.method.trim().is_empty() {
            bail!("Shadowsocks method is empty");
        }

        if self.password.is_empty() {
            bail!("Shadowsocks password is empty");
        }

        Ok(())
    }

    fn build(self) -> Result<ShadowsocksConfig> {
        let config = ShadowsocksConfig {
            tag: self.tag,
            server: self.server,
            server_port: self.server_port,
            method: self.method,
            password: self.password,

            network: self.network,
            plugin: self.plugin,
            plugin_opts: self.plugin_opts,

            dial: None,
        };

        config.validate()?;

        Ok(config)
    }
}

fn parse_ss_authority(input: &str) -> Result<(String, String, String, u16)> {
    if let Some((userinfo, host_port)) = input.rsplit_once('@') {
        return parse_sip002_authority(userinfo, host_port);
    }
    parse_legacy_authority(input)
}

fn parse_sip002_authority(
    encoded_userinfo: &str,
    host_port: &str,
) -> Result<(String, String, String, u16)> {
    if encoded_userinfo.is_empty() {
        bail!("Missing Shadowsocks credentials");
    }

    let decoded =
        decode_base64_string(encoded_userinfo).context("Cannot decode Shadowsocks credentials")?;

    let (method, password) = parse_method_password(&decoded)?;

    let (server, port) = parse_host_port(host_port)?;

    Ok((method, password, server, port))
}

fn parse_legacy_authority(encoded: &str) -> Result<(String, String, String, u16)> {
    if encoded.is_empty() {
        bail!("Missing Shadowsocks URL payload");
    }

    let decoded = decode_base64_string(encoded).context("Cannot decode legacy Shadowsocks URL")?;
    let (userinfo, host_port) = decoded
        .rsplit_once('@')
        .context("Invalid legacy Shadowsocks URL: missing '@'")?;

    let (method, password) = parse_method_password(userinfo)?;

    let (server, port) = parse_host_port(host_port)?;

    Ok((method, password, server, port))
}

fn parse_method_password(input: &str) -> Result<(String, String)> {
    let (method, password) = input
        .split_once(':')
        .context("Invalid Shadowsocks credentials: expected method:password")?;

    if method.trim().is_empty() {
        bail!("Shadowsocks method is empty");
    }

    if password.is_empty() {
        bail!("Shadowsocks password is empty");
    }

    Ok((method.to_string(), password.to_string()))
}

fn parse_host_port(input: &str) -> Result<(String, u16)> {
    let fake_url = format!("ss://dummy@{input}");

    let url = Url::parse(&fake_url).context("Invalid Shadowsocks server address")?;

    let server = url
        .host_str()
        .context("Shadowsocks URL does not contain a server")?
        .to_owned();

    if server.trim().is_empty() {
        bail!("Shadowsocks server is empty");
    }

    let port = url
        .port()
        .context("Shadowsocks URL does not contain a port")?;

    if port == 0 {
        bail!("Shadowsocks server port cannot be 0");
    }

    Ok((server, port))
}

fn decode_base64_string(input: &str) -> Result<String> {
    let bytes = decode_base64_flexible(input)?;

    String::from_utf8(bytes).context("Decoded Shadowsocks Base64 is not valid UTF-8")
}

fn decode_base64_flexible(input: &str) -> Result<Vec<u8>> {
    let decoded_input =
        urlencoding::decode(input).context("Invalid percent encoding in Shadowsocks Base64")?;

    let normalized: String = decoded_input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();

    if normalized.is_empty() {
        bail!("Empty Shadowsocks Base64 payload");
    }

    if let Ok(value) = URL_SAFE_NO_PAD.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = URL_SAFE.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = STANDARD_NO_PAD.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = STANDARD.decode(&normalized) {
        return Ok(value);
    }

    bail!("Invalid Shadowsocks Base64 payload")
}

fn decode_component(input: &str, name: &str) -> Result<String> {
    urlencoding::decode(input)
        .with_context(|| format!("Invalid percent encoding in Shadowsocks {name}"))
        .map(|value| value.into_owned())
}

fn parse_network(value: &str) -> Result<Network> {
    match value.to_ascii_lowercase().as_str() {
        "tcp" => Ok(Network::Tcp),
        "udp" => Ok(Network::Udp),

        value => {
            bail!("Unsupported Shadowsocks network '{value}'")
        }
    }
}
