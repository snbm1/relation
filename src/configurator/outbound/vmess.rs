use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use macros::auto_skip_none;

use crate::configurator::shared::Network;
use crate::configurator::shared::dialfields::DialFields;
use crate::configurator::shared::multiplex::*;
use crate::configurator::shared::tls::*;
use crate::configurator::shared::transport::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketEncoding {
    #[default]
    #[serde(rename = "")]
    None,

    #[serde(rename = "packetaddr")]
    Packetaddr,

    #[serde(rename = "xudp")]
    Xudp,
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VmessConfig {
    tag: String,
    server: String,
    server_port: u16,
    uuid: String,

    security: String,
    alter_id: u32,

    network: Option<Network>,
    tls: Option<TlsConfig>,
    packet_encoding: Option<PacketEncoding>,
    multiplex: Option<MultiplexConfig>,
    transport: Option<TransportConfig>,

    #[serde(flatten)]
    dial: Option<DialFields>,
}

impl VmessConfig {
    pub fn new() -> Self {
        Self {
            tag: "outbound-vmess".to_string(),
            security: "auto".to_string(),
            ..Default::default()
        }
    }

    pub fn with_tag(tag: String) -> Self {
        Self {
            tag,
            security: "auto".to_string(),
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
            bail!("VMess server is empty");
        }

        if self.server_port == 0 {
            bail!("VMess server port cannot be 0");
        }

        validate_uuid(&self.uuid)?;

        match self.security.as_str() {
            "auto" | "none" | "zero" | "aes-128-gcm" | "chacha20-poly1305" | "aes-128-ctr" => {}

            security => {
                bail!("Unsupported VMess security method '{security}'");
            }
        }

        if let Some(tls) = &self.tls {
            if !tls.check() {
                bail!("Invalid VMess TLS configuration");
            }
        }

        Ok(())
    }

    pub fn check(&mut self) -> bool {
        self.validate().is_ok()
    }

    pub fn from_url(input: &str) -> Result<Self> {
        let parsed = ParsedVmessUrl::parse(input)?;
        parsed.build()
    }
}

#[derive(Debug, Default)]
struct ParsedVmessUrl {
    tag: String,

    server: String,
    server_port: u16,

    uuid: String,
    alter_id: u32,
    security: String,

    transport: TransportKind,

    path: Option<String>,
    host: Option<String>,
    service_name: Option<String>,

    tls_enabled: bool,
    server_name: Option<String>,
    fingerprint: Option<String>,

    packet_encoding: Option<PacketEncoding>,

    mux: Option<u16>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum TransportKind {
    #[default]
    Tcp,
    WebSocket,
    Grpc,
    Http,
    HttpUpgrade,
    Quic,
}

impl ParsedVmessUrl {
    fn parse(input: &str) -> Result<Self> {
        let encoded = input
            .strip_prefix("vmess://")
            .context("Invalid VMess URL: expected vmess://")?;

        if encoded.trim().is_empty() {
            bail!("VMess URL contains an empty payload");
        }

        let decoded =
            decode_base64_flexible(encoded).context("Cannot decode VMess Base64 payload")?;

        let json = String::from_utf8(decoded).context("VMess payload is not valid UTF-8")?;

        let share: VmessShare =
            serde_json::from_str(&json).context("VMess payload is not valid JSON")?;

        share.into_parsed()
    }

    fn build(self) -> Result<VmessConfig> {
        let tls = build_tls(self.tls_enabled, self.server_name, self.fingerprint)?;

        let transport = build_transport(self.transport, self.path, self.host, self.service_name)?;

        let multiplex = build_multiplex(self.mux)?;

        let config = VmessConfig {
            tag: self.tag,
            server: self.server,
            server_port: self.server_port,
            uuid: self.uuid,

            security: self.security,
            alter_id: self.alter_id,

            network: None,
            tls,
            packet_encoding: self.packet_encoding,
            multiplex,
            transport,

            dial: None,
        };

        config.validate()?;

        Ok(config)
    }
}

#[derive(Deserialize, Debug, Default)]
struct VmessShare {
    #[serde(default)]
    v: FlexibleString,

    #[serde(default)]
    ps: FlexibleString,

    #[serde(default)]
    add: FlexibleString,

    #[serde(default)]
    port: FlexibleString,

    #[serde(default)]
    id: FlexibleString,

    #[serde(default)]
    aid: FlexibleString,

    #[serde(default)]
    scy: FlexibleString,

    #[serde(default)]
    net: FlexibleString,

    #[serde(default, rename = "type")]
    header_type: FlexibleString,

    #[serde(default)]
    host: FlexibleString,

    #[serde(default)]
    path: FlexibleString,

    #[serde(default)]
    tls: FlexibleString,

    #[serde(default)]
    sni: FlexibleString,

    #[serde(default)]
    fp: FlexibleString,

    #[serde(default)]
    alpn: FlexibleString,

    #[serde(default, rename = "serviceName")]
    service_name: FlexibleString,

    #[serde(default, rename = "packetEncoding")]
    packet_encoding: FlexibleString,

    #[serde(default)]
    mux: FlexibleString,
}

impl VmessShare {
    fn into_parsed(self) -> Result<ParsedVmessUrl> {
        let server = self.add.into_string();

        if server.trim().is_empty() {
            bail!("VMess JSON does not contain 'add'");
        }

        let port = self.port.into_string();

        if port.trim().is_empty() {
            bail!("VMess JSON does not contain 'port'");
        }

        let server_port = port
            .parse::<u16>()
            .with_context(|| format!("Invalid VMess port '{port}'"))?;

        if server_port == 0 {
            bail!("VMess server port cannot be 0");
        }

        let uuid = self.id.into_string();

        validate_uuid(&uuid)?;

        let aid = self.aid.into_string();

        let alter_id = if aid.trim().is_empty() {
            0
        } else {
            aid.parse::<u32>()
                .with_context(|| format!("Invalid VMess alterId '{aid}'"))?
        };

        let security = normalize_security(self.scy.as_str())?;

        let net = self.net.into_string();

        let transport = parse_transport(&net)?;

        let tag = {
            let value = self.ps.into_string();

            if value.trim().is_empty() {
                "outbound-vmess".to_string()
            } else {
                value
            }
        };

        let host = non_empty(self.host.into_string());

        let path = non_empty(self.path.into_string());

        let explicit_service_name = non_empty(self.service_name.into_string());
        let service_name = if transport == TransportKind::Grpc {
            explicit_service_name.or_else(|| path.clone())
        } else {
            explicit_service_name
        };

        let tls_value = self.tls.into_string().to_ascii_lowercase();

        let tls_enabled = matches!(tls_value.as_str(), "tls" | "1" | "true");
        if !tls_value.is_empty() && tls_value != "none" && !tls_enabled {
            bail!("Unsupported VMess TLS mode '{tls_value}'");
        }

        let server_name = non_empty(self.sni.into_string());

        let fingerprint = non_empty(self.fp.into_string());

        let packet_encoding = {
            let value = self.packet_encoding.into_string();

            if value.is_empty() {
                None
            } else {
                Some(parse_packet_encoding(&value)?)
            }
        };

        let mux = {
            let value = self.mux.into_string();

            if value.is_empty() {
                None
            } else {
                let streams = value
                    .parse::<u16>()
                    .with_context(|| format!("Invalid VMess mux value '{value}'"))?;

                if streams == 0 {
                    bail!("VMess mux value must be greater than 0");
                }

                Some(streams)
            }
        };
        let _header_type = self.header_type;
        let _alpn = self.alpn;

        Ok(ParsedVmessUrl {
            tag,

            server,
            server_port,

            uuid,
            alter_id,
            security,

            transport,

            path,
            host,
            service_name,

            tls_enabled,
            server_name,
            fingerprint,

            packet_encoding,
            mux,
        })
    }
}

fn normalize_security(input: &str) -> Result<String> {
    let value = input.trim().to_ascii_lowercase();
    match value.as_str() {
        "" => Ok("auto".to_string()),
        "auto" | "none" | "zero" | "aes-128-gcm" | "chacha20-poly1305" | "aes-128-ctr" => Ok(value),
        _ => {
            bail!("Unsupported VMess security method '{input}'")
        }
    }
}

fn parse_transport(input: &str) -> Result<TransportKind> {
    match input.trim().to_ascii_lowercase().as_str() {
        "" | "tcp" => Ok(TransportKind::Tcp),
        "ws" | "websocket" => Ok(TransportKind::WebSocket),
        "grpc" => Ok(TransportKind::Grpc),
        "h2" | "http" => Ok(TransportKind::Http),
        "httpupgrade" => Ok(TransportKind::HttpUpgrade),
        "quic" => Ok(TransportKind::Quic),
        value => {
            bail!("Unsupported VMess transport '{value}'")
        }
    }
}

fn build_tls(
    enabled: bool,
    server_name: Option<String>,
    fingerprint: Option<String>,
) -> Result<Option<TlsConfig>> {
    if !enabled {
        return Ok(None);
    }

    let mut tls = TlsConfig::new();

    tls.enabled = Some(true);

    if let Some(server_name) = server_name {
        tls.server_name = Some(server_name);
    }

    if let Some(fingerprint) = fingerprint {
        let mut utls = UtlsConfig::new();

        utls.enabled = Some(true);
        utls.fingerprint = Some(fingerprint);

        if !utls.check() {
            bail!("Invalid VMess uTLS configuration");
        }

        tls.utls = Some(utls);
    }
    if !tls.check() {
        bail!("Invalid VMess TLS configuration");
    }

    Ok(Some(tls))
}

fn build_transport(
    kind: TransportKind,
    path: Option<String>,
    host: Option<String>,
    service_name: Option<String>,
) -> Result<Option<TransportConfig>> {
    match kind {
        TransportKind::Tcp => Ok(None),

        TransportKind::WebSocket => {
            let mut ws = WebSocketConfig::new();

            ws.path = path;

            if let Some(host) = host {
                ws.headers = Some(HashMap::from([("Host".to_string(), host)]));
            }

            if !ws.check() {
                bail!("Invalid VMess WebSocket transport");
            }

            Ok(Some(TransportConfig::WebSocket(ws)))
        }

        TransportKind::Grpc => {
            let mut grpc = GrpcConfig::new();

            grpc.service_name = service_name;

            if !grpc.check() {
                bail!("Invalid VMess gRPC transport");
            }

            Ok(Some(TransportConfig::Grpc(grpc)))
        }

        TransportKind::Http => {
            let mut http = HttpConfig::new();

            http.path = path;

            if let Some(host) = host {
                let hosts = host
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect::<Vec<_>>();

                if !hosts.is_empty() {
                    http.host = Some(hosts);
                }
            }

            if !http.check() {
                bail!("Invalid VMess HTTP transport");
            }

            Ok(Some(TransportConfig::Http(http)))
        }

        TransportKind::HttpUpgrade => {
            let mut upgrade = HttpUpgradeConfig::new();

            upgrade.path = path;

            if let Some(host) = host {
                upgrade.host = Some(vec![host]);
            }

            if !upgrade.check() {
                bail!("Invalid VMess HTTPUpgrade transport");
            }

            Ok(Some(TransportConfig::HttpUpgrade(upgrade)))
        }

        TransportKind::Quic => {
            let quic = QuicConfig::new();

            if !quic.check() {
                bail!("Invalid VMess QUIC transport");
            }

            Ok(Some(TransportConfig::Quic(quic)))
        }
    }
}

fn build_multiplex(max_streams: Option<u16>) -> Result<Option<MultiplexConfig>> {
    let Some(max_streams) = max_streams else {
        return Ok(None);
    };

    let mut multiplex = MultiplexConfig::new();

    multiplex.enable = Some(true);
    multiplex.protocol = Some("h2mux".to_string());
    multiplex.max_streams = Some(max_streams);

    if !multiplex.check() {
        bail!("Invalid VMess multiplex configuration");
    }

    Ok(Some(multiplex))
}

fn parse_packet_encoding(input: &str) -> Result<PacketEncoding> {
    match input.trim().to_ascii_lowercase().as_str() {
        "" | "none" => Ok(PacketEncoding::None),

        "packetaddr" => Ok(PacketEncoding::Packetaddr),

        "xudp" => Ok(PacketEncoding::Xudp),

        value => {
            bail!("Unsupported VMess packet encoding '{value}'")
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FlexibleString(String);

impl FlexibleString {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn into_string(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for FlexibleString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let value = match value {
            Value::Null => String::new(),

            Value::String(value) => value,

            Value::Number(value) => value.to_string(),

            Value::Bool(value) => value.to_string(),

            other => {
                return Err(serde::de::Error::custom(format!(
                    "expected string/number/bool, got {other}"
                )));
            }
        };

        Ok(Self(value))
    }
}

fn non_empty(input: String) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else {
        Some(input)
    }
}

fn decode_base64_flexible(input: &str) -> Result<Vec<u8>> {
    let normalized: String = input.chars().filter(|ch| !ch.is_whitespace()).collect();

    if normalized.is_empty() {
        bail!("Empty VMess Base64 payload");
    }

    if let Ok(value) = STANDARD.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = STANDARD_NO_PAD.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = URL_SAFE.decode(&normalized) {
        return Ok(value);
    }

    if let Ok(value) = URL_SAFE_NO_PAD.decode(&normalized) {
        return Ok(value);
    }

    bail!("Invalid VMess Base64 payload")
}

fn validate_uuid(value: &str) -> Result<()> {
    if value.len() != 36 {
        bail!("Invalid VMess UUID '{value}': expected 36 characters");
    }

    for (index, byte) in value.bytes().enumerate() {
        match index {
            8 | 13 | 18 | 23 => {
                if byte != b'-' {
                    bail!(
                        "Invalid VMess UUID '{value}': \
                         expected '-' at position {}",
                        index + 1
                    );
                }
            }

            _ => {
                if !byte.is_ascii_hexdigit() {
                    bail!(
                        "Invalid VMess UUID '{value}': \
                         invalid character at position {}",
                        index + 1
                    );
                }
            }
        }
    }
    Ok(())
}
