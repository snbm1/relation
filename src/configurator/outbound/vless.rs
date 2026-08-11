use std::collections::HashMap;

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use url::Url;

use macros::auto_skip_none;

use crate::configurator::shared::{
    Network, dialfields::DialFields, multiplex::*, tls::*, transport::*,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    #[default]
    #[serde(rename = "")]
    None,
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketEncoding {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "packetaddr")]
    Packetaddr,
    #[default]
    #[serde(rename = "xudp")]
    Xudp,
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VlessConfig {
    tag: String,
    server: String,
    server_port: u16,
    uuid: String,
    flow: Option<Flow>,
    network: Option<Network>,
    tls: Option<TlsConfig>,
    multiplex: Option<MultiplexConfig>,
    packet_encoding: Option<PacketEncoding>,
    transport: Option<TransportConfig>,

    #[serde(flatten)]
    dial: Option<DialFields>,
}

#[derive(Default, Debug)]
struct ParsedVlessUrl {
    server: String,
    server_port: u16,
    uuid: String,
    tag: String,

    transport_type: Option<TransportKind>,
    security: SecurityKind,
    flow: Option<Flow>,
    packet_encoding: Option<PacketEncoding>,

    sni: Option<String>,
    fingerprint: Option<String>,
    reality_public_key: Option<String>,
    reality_short_id: Option<String>,

    mux: Option<u16>,

    path: Option<String>,
    hosts: Vec<String>,
    service_name: Option<String>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum SecurityKind {
    #[default]
    None,
    Tls,
    Reality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransportKind {
    Tcp,
    WebSocket,
    Grpc,
    Quic,
    Http,
    HttpUpgrade,
}

impl VlessConfig {
    pub fn new() -> Self {
        Self {
            tag: "outbound-vless".to_string(),
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

    fn apply_defaults(&mut self) {
        self.flow.get_or_insert_default();
        self.packet_encoding.get_or_insert_default();
    }

    pub fn validate(&self) -> Result<()> {
        if self.server.trim().is_empty() {
            bail!("VLESS server is empty");
        }

        if self.server_port == 0 {
            bail!("VLESS server port cannot be 0");
        }

        if self.uuid.trim().is_empty() {
            bail!("VLESS UUID is empty");
        }

        validate_uuid(&self.uuid)?;

        if let Some(tls) = &self.tls {
            if !tls.check() {
                bail!("Invalid VLESS TLS configuration");
            }
        }

        Ok(())
    }

    pub fn check(&mut self) -> bool {
        self.apply_defaults();
        self.validate().is_ok()
    }

    pub fn from_url(input: &str) -> Result<Self> {
        let parsed = ParsedVlessUrl::parse(input)?;
        parsed.build()
    }
}

impl ParsedVlessUrl {
    fn parse(input: &str) -> Result<Self> {
        let url = Url::parse(input).context("Invalid VLESS URL")?;

        if url.scheme() != "vless" {
            bail!("Invalid URL scheme '{}': expected 'vless'", url.scheme());
        }

        let server = url
            .host_str()
            .context("VLESS URL does not contain a server")?
            .to_owned();

        if server.trim().is_empty() {
            bail!("VLESS URL contains an empty server");
        }

        let server_port = url.port().unwrap_or(443);

        if server_port == 0 {
            bail!("VLESS server port cannot be 0");
        }

        let uuid = decode_component(url.username(), "UUID")?;

        if uuid.trim().is_empty() {
            bail!("VLESS URL does not contain a UUID");
        }

        validate_uuid(&uuid)?;

        if url.password().is_some() {
            bail!("VLESS URL must not contain a password");
        }

        let tag = match url.fragment() {
            Some(fragment) if !fragment.is_empty() => decode_component(fragment, "fragment/tag")?,
            _ => "vless_outbound".to_string(),
        };

        let mut result = ParsedVlessUrl {
            server,
            server_port,
            uuid,
            tag,
            ..Default::default()
        };

        for (key, value) in url.query_pairs() {
            let key = key.as_ref();
            let value = value.into_owned();
            result.parse_query_parameter(key, value)?;
        }

        result.validate_parameters()?;

        Ok(result)
    }

    fn parse_query_parameter(&mut self, key: &str, value: String) -> Result<()> {
        match key {
            "type" => {
                self.transport_type = Some(parse_transport_type(&value)?);
            }
            "security" => {
                self.security = parse_security(&value)?;
            }
            "flow" => {
                self.flow = Some(parse_flow(&value)?);
            }
            "sni" => {
                self.sni = non_empty(value);
            }
            "fp" => {
                self.fingerprint = non_empty(value);
            }
            "pbk" => {
                self.reality_public_key = non_empty(value);
            }
            "sid" => {
                self.reality_short_id = non_empty(value);
            }
            "mux" => {
                if value.is_empty() {
                    bail!("mux parameter cannot be empty");
                }
                let max_streams = value
                    .parse::<u16>()
                    .with_context(|| format!("Invalid mux value '{value}'"))?;
                if max_streams == 0 {
                    bail!("mux must be greater than 0");
                }
                self.mux = Some(max_streams);
            }
            "path" => {
                self.path = Some(value);
            }
            "host" => {
                if !value.is_empty() {
                    self.hosts.push(value);
                }
            }
            "serviceName" | "service_name" => {
                self.service_name = Some(value);
            }
            "packetEncoding" | "packet_encoding" => {
                self.packet_encoding = Some(parse_packet_encoding(&value)?);
            }
            _ => {}
        }

        Ok(())
    }

    fn validate_parameters(&self) -> Result<()> {
        if self.reality_public_key.is_some() || self.reality_short_id.is_some() {
            if self.security != SecurityKind::Reality {
                bail!("REALITY parameters 'pbk'/'sid' require security=reality");
            }
        }

        if self.security == SecurityKind::Reality {
            if self.reality_public_key.is_none() {
                bail!("security=reality requires a non-empty 'pbk' parameter");
            }
        }

        if self.flow == Some(Flow::XtlsRprxVision) {
            match self.transport_type.unwrap_or(TransportKind::Tcp) {
                TransportKind::Tcp => {}
                other => {
                    bail!(
                        "flow=xtls-rprx-vision cannot be combined with transport {:?}",
                        other
                    );
                }
            }
        }

        Ok(())
    }

    fn build(self) -> Result<VlessConfig> {
        let ParsedVlessUrl {
            server,
            server_port,
            uuid,
            tag,

            transport_type,
            security,
            flow,
            packet_encoding,

            sni,
            fingerprint,
            reality_public_key,
            reality_short_id,

            mux,

            path,
            hosts,
            service_name,
        } = self;

        let tls = build_tls(
            security,
            sni,
            fingerprint,
            reality_public_key,
            reality_short_id,
        )?;

        let multiplex = build_multiplex(mux)?;

        let transport = build_transport(
            transport_type.unwrap_or(TransportKind::Tcp),
            path,
            hosts,
            service_name,
        )?;

        let mut config = VlessConfig {
            tag,
            server,
            server_port,
            uuid,
            flow,
            network: None,
            tls,
            multiplex,
            packet_encoding,
            transport,
            dial: None,
        };

        config.apply_defaults();
        config.validate()?;

        Ok(config)
    }
}

fn parse_transport_type(value: &str) -> Result<TransportKind> {
    match value.to_ascii_lowercase().as_str() {
        "" | "tcp" => Ok(TransportKind::Tcp),

        "ws" | "websocket" => Ok(TransportKind::WebSocket),

        "grpc" => Ok(TransportKind::Grpc),

        "quic" => Ok(TransportKind::Quic),

        "http" | "h2" => Ok(TransportKind::Http),

        // Correct sing-box/Xray spelling.
        "httpupgrade" => Ok(TransportKind::HttpUpgrade),

        _ => bail!("Unsupported VLESS transport type '{value}'"),
    }
}

fn parse_security(value: &str) -> Result<SecurityKind> {
    match value.to_ascii_lowercase().as_str() {
        "" | "none" => Ok(SecurityKind::None),
        "tls" => Ok(SecurityKind::Tls),
        "reality" => Ok(SecurityKind::Reality),

        _ => bail!("Unsupported VLESS security type '{value}'"),
    }
}

fn parse_flow(value: &str) -> Result<Flow> {
    match value {
        "" | "none" => Ok(Flow::None),
        "xtls-rprx-vision" => Ok(Flow::XtlsRprxVision),

        _ => bail!("Unsupported VLESS flow '{value}'"),
    }
}

fn parse_packet_encoding(value: &str) -> Result<PacketEncoding> {
    match value.to_ascii_lowercase().as_str() {
        "" | "none" => Ok(PacketEncoding::None),
        "packetaddr" => Ok(PacketEncoding::Packetaddr),
        "xudp" => Ok(PacketEncoding::Xudp),

        _ => bail!("Unsupported packet encoding '{value}'"),
    }
}

fn build_tls(
    security: SecurityKind,
    sni: Option<String>,
    fingerprint: Option<String>,
    reality_public_key: Option<String>,
    reality_short_id: Option<String>,
) -> Result<Option<TlsConfig>> {
    if security == SecurityKind::None {
        return Ok(None);
    }

    let mut tls = TlsConfig::new();
    tls.enabled = Some(true);

    if let Some(sni) = sni {
        tls.server_name = Some(sni);
    }
    if let Some(fingerprint) = fingerprint {
        let mut utls = UtlsConfig::new();

        utls.enabled = Some(true);
        utls.fingerprint = Some(fingerprint);

        if !utls.check() {
            bail!("Invalid uTLS configuration");
        }

        tls.utls = Some(utls);
    }

    if security == SecurityKind::Reality {
        let mut reality = RealityConfig::new();

        reality.enabled = Some(true);
        reality.public_key = reality_public_key;
        reality.short_id = reality_short_id;

        if !reality.check() {
            bail!("Invalid REALITY configuration");
        }

        tls.reality = Some(reality);
    }

    if !tls.check() {
        bail!("Invalid TLS configuration");
    }

    Ok(Some(tls))
}

fn build_transport(
    kind: TransportKind,
    path: Option<String>,
    hosts: Vec<String>,
    service_name: Option<String>,
) -> Result<Option<TransportConfig>> {
    match kind {
        TransportKind::Tcp => Ok(None),

        TransportKind::WebSocket => {
            let mut ws = WebSocketConfig::new();

            ws.path = path;

            if let Some(host) = hosts.last() {
                ws.headers = Some(HashMap::from([("Host".to_string(), host.clone())]));
            }

            if !ws.check() {
                bail!("Invalid WebSocket transport configuration");
            }

            Ok(Some(TransportConfig::WebSocket(ws)))
        }

        TransportKind::Grpc => {
            let mut grpc = GrpcConfig::new();

            grpc.service_name = service_name;

            if !grpc.check() {
                bail!("Invalid gRPC transport configuration");
            }

            Ok(Some(TransportConfig::Grpc(grpc)))
        }

        TransportKind::Quic => {
            let quic = QuicConfig::new();

            if !quic.check() {
                bail!("Invalid QUIC transport configuration");
            }

            Ok(Some(TransportConfig::Quic(quic)))
        }

        TransportKind::Http => {
            let mut http = HttpConfig::new();

            http.path = path;

            if !hosts.is_empty() {
                http.host = Some(hosts);
            }

            if !http.check() {
                bail!("Invalid HTTP transport configuration");
            }

            Ok(Some(TransportConfig::Http(http)))
        }

        TransportKind::HttpUpgrade => {
            let mut upgrade = HttpUpgradeConfig::new();

            upgrade.path = path;

            if !hosts.is_empty() {
                upgrade.host = Some(hosts);
            }

            if !upgrade.check() {
                bail!("Invalid HTTPUpgrade transport configuration");
            }

            Ok(Some(TransportConfig::HttpUpgrade(upgrade)))
        }
    }
}

fn non_empty(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

fn decode_component(value: &str, name: &str) -> Result<String> {
    urlencoding::decode(value)
        .with_context(|| format!("Invalid percent encoding in VLESS {name}"))
        .map(|value| value.into_owned())
}

fn validate_uuid(value: &str) -> Result<()> {
    if value.len() != 36 {
        bail!("Invalid VLESS UUID '{value}': expected 36 characters");
    }

    for (index, byte) in value.bytes().enumerate() {
        match index {
            8 | 13 | 18 | 23 => {
                if byte != b'-' {
                    bail!(
                        "Invalid VLESS UUID '{value}': expected '-' at position {}",
                        index + 1
                    );
                }
            }

            _ => {
                if !byte.is_ascii_hexdigit() {
                    bail!(
                        "Invalid VLESS UUID '{value}': invalid character at position {}",
                        index + 1
                    );
                }
            }
        }
    }

    Ok(())
}
