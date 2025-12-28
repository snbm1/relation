use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::configurator::shared::Network;
use crate::configurator::shared::dialfields::DialFields;
use crate::configurator::shared::multiplex::*;
use crate::configurator::shared::tls::*;
use crate::configurator::shared::transport::*;

use crate::configurator::Config;

#[derive(Serialize, Deserialize, Default, Debug)]
enum Flow {
    #[serde(rename = "")]
    None,
    #[default]
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
}

#[derive(Serialize, Deserialize, Debug)]
enum TransportType {
    Grpc,
    #[serde(rename = "ws")]
    WebSocket,
}

#[derive(Serialize, Deserialize, Default, Debug)]
enum PacketEncoding {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "packetaddr")]
    Packetaddr,
    #[default]
    #[serde(rename = "xudp")]
    Xudp,
}

#[warn(dead_code)]
enum PossibleKeys {
    Server,
    ServerPort,
    Uuid,
    Security,
    Type,
    Header,
    Flow,
    Path,
    Host,
    Sni,
    Fp,
    Pbk,
    Sid,
    Mux,
    ServiceName,
}

#[warn(dead_code)]
enum PossibleValues {
    Bool(bool),
    U16(u16),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct VlessConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    server: Option<String>,
    server_port: Option<u16>,
    uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow: Option<Flow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplex: Option<MultiplexConfig>,
    packet_encoding: Option<PacketEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport: Option<TransportConfig>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    dial: Option<DialFields>,
}

impl VlessConfig {
    fn new() -> VlessConfig {
        VlessConfig {
            config_type: Some("vless".to_string()),
            tag: Some("vless-outbound".to_string()),
            server: None,
            server_port: None,
            uuid: None,
            flow: None,
            network: None,
            tls: None,
            multiplex: None,
            transport: None,
            packet_encoding: None,
            dial: None,
        }
    }

    fn check(&mut self) -> bool {
        match self.server.is_none() || self.server_port.is_none() || self.uuid.is_none() {
            true => false,
            false => {
                if self.flow.is_none() {
                    self.flow = Some(Flow::default());
                }
                if self.packet_encoding.is_none() {
                    self.packet_encoding = Some(PacketEncoding::default());
                }
                match self.tls.is_none() {
                    true => true,
                    false => self.tls.as_ref().unwrap().check(),
                }
            }
        }
    }

    fn parser(input: &str) -> Vec<(PossibleKeys, PossibleValues)> {
        let parsed_input = Url::parse(input).unwrap();
        let mut values: Vec<(PossibleKeys, PossibleValues)> = Vec::new();
        values.push((
            PossibleKeys::Server,
            PossibleValues::String(String::from(parsed_input.host_str().unwrap())),
        ));
        values.push((
            PossibleKeys::ServerPort,
            PossibleValues::U16(parsed_input.port().unwrap_or(443)),
        ));
        values.push((
            PossibleKeys::Uuid,
            PossibleValues::String(String::from(parsed_input.username())),
        ));
        for i in parsed_input
            .query()
            .unwrap()
            .split("&")
            .map(|x| x.split("="))
        {
            let j = i.collect::<Vec<_>>();
            let ln = j.len();
            if ln > 1 {
                match j[0] {
                    "type" => values.push((
                        PossibleKeys::Type,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "security" => values.push((
                        PossibleKeys::Security,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "flow" => values.push((
                        PossibleKeys::Flow,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "sni" => values.push((
                        PossibleKeys::Sni,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "fp" => {
                        values.push((PossibleKeys::Fp, PossibleValues::String(String::from(j[1]))))
                    }
                    "pbk" => values.push((
                        PossibleKeys::Pbk,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "sid" => values.push((
                        PossibleKeys::Sid,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "mux" => values.push((
                        PossibleKeys::Mux,
                        PossibleValues::U16(j[1].parse().unwrap()),
                    )),
                    "path" => values.push((
                        PossibleKeys::Path,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "host" => values.push((
                        PossibleKeys::Host,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    "serviceName" => values.push((
                        PossibleKeys::ServiceName,
                        PossibleValues::String(String::from(j[1])),
                    )),
                    _ => {}
                }
            }
        }
        values
    }
}

impl Config for VlessConfig {
    fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let value = VlessConfig::parser(url);

        let mut cfg = VlessConfig::new();

        let mut tls = TlsConfig::new();
        let mut utls = UtlsConfig::new();
        let mut rlt = RealityConfig::new();
        let mut mtx = MultiplexConfig::new();
        let mut tfg = TransportConfig::None;

        for (key, val) in value {
            match key {
                PossibleKeys::Type => {
                    match val {
                        PossibleValues::String(x) => match x.as_str() {
                            "tcp" => tfg = TransportConfig::Tcp,
                            "ws" => tfg = TransportConfig::WebSocket(WebSocketConfig::new()),
                            "grpc" => tfg = TransportConfig::Grpc(GrpcConfig::new()),
                            "quic" => tfg = TransportConfig::Quic(QuicConfig::new()),
                            "http" => tfg = TransportConfig::Http(HttpConfig::new()),
                            "httpupdate" => {
                                tfg = TransportConfig::HttpUpdate(HttpUpdateConfig::new())
                            }
                            _ => {}
                        },
                        _ => return Err("Invalid type type".into()),
                    };
                }

                PossibleKeys::Flow => {
                    match val {
                        PossibleValues::String(x) => match x.as_str() {
                            "xtls-rprx-vision" => cfg.flow = Some(Flow::XtlsRprxVision),
                            _ => cfg.flow = Some(Flow::None),
                        },
                        _ => return Err("Invalid flow type".into()),
                    };
                }

                PossibleKeys::Server => {
                    match val {
                        PossibleValues::String(x) => cfg.server = Some(x),
                        _ => return Err("Invalid server type".into()),
                    };
                }

                PossibleKeys::ServerPort => {
                    match val {
                        PossibleValues::U16(x) => cfg.server_port = Some(x),
                        _ => return Err("Invalid port type".into()),
                    };
                }

                PossibleKeys::Uuid => {
                    match val {
                        PossibleValues::String(x) => cfg.uuid = Some(x),
                        _ => return Err("Invalid uuid type".into()),
                    };
                }

                PossibleKeys::Sni => match val {
                    PossibleValues::String(x) => {
                        tls.enable = Some(true);
                        tls.server_name = Some(x);
                    }
                    _ => return Err("Invalid tls server name".into()),
                },

                PossibleKeys::Fp => match val {
                    PossibleValues::String(x) => {
                        tls.enable = Some(true);
                        utls.enable = Some(true);
                        utls.fingerprint = Some(x);
                    }
                    _ => return Err("Invalid fingerprint name".into()),
                },

                PossibleKeys::Security => match val {
                    PossibleValues::String(x) => match x.as_str() {
                        "reality" => rlt.enable = Some(true),
                        "tls" => tls.enable = Some(true),
                        _ => eprintln!("{} not supported", x),
                    },
                    _ => return Err("Invalid Security type".into()),
                },

                PossibleKeys::Pbk => match val {
                    PossibleValues::String(x) => {
                        tls.enable = Some(true);
                        rlt.enable = Some(true);
                        rlt.public_key = Some(x);
                    }
                    _ => return Err("Invalid public key".into()),
                },

                PossibleKeys::Sid => match val {
                    PossibleValues::String(x) => {
                        tls.enable = Some(true);
                        rlt.enable = Some(true);
                        rlt.short_id = Some(x);
                    }
                    _ => return Err("Invalid short id".into()),
                },

                PossibleKeys::Mux => match val {
                    PossibleValues::U16(x) => {
                        mtx.enable = Some(true);
                        mtx.protocol = Some("h2mux".to_string());
                        mtx.max_streams = Some(x);
                    }
                    _ => return Err("Invalid multiplex type".into()),
                },
                // urlencoding::decode(encoded).unwrap();
                PossibleKeys::Path => match val {
                    PossibleValues::String(x) => match tfg {
                        TransportConfig::None => eprintln!("[Warning] Path before transport"),
                        TransportConfig::WebSocket(ref mut z) => z.path = Some(x),
                        TransportConfig::Http(ref mut z) => z.path = Some(x),
                        TransportConfig::HttpUpdate(ref mut z) => z.path = Some(x),
                        _ => {}
                    },
                    _ => return Err("Invalid Path type".into()),
                },

                PossibleKeys::Host => match val {
                    PossibleValues::String(x) => match tfg {
                        TransportConfig::None => eprintln!("[Warning] Host before transport"),
                        TransportConfig::WebSocket(ref mut z) => match z.headers {
                            None => z.headers = Some(HashMap::from([("Host".to_string(), x)])),
                            Some(_) => {
                                let _ = z.headers.as_mut().unwrap().insert("Host".to_string(), x);
                            }
                        },
                        TransportConfig::Http(ref mut z) => match z.host {
                            None => z.host = Some(vec![x]),
                            Some(_) => {
                                z.host.as_mut().unwrap().push(x);
                            }
                        },
                        TransportConfig::HttpUpdate(ref mut z) => match z.host {
                            None => z.host = Some(vec![x]),
                            Some(_) => {
                                z.host.as_mut().unwrap().push(x);
                            }
                        },
                        TransportConfig::Tcp => {}
                        _ => {}
                    },
                    _ => return Err("Invalid Host type".into()),
                },

                PossibleKeys::ServiceName => match val {
                    PossibleValues::String(x) => match tfg {
                        TransportConfig::None => return Err("Host before type".into()),
                        TransportConfig::Grpc(ref mut z) => z.service_name = Some(x),
                        _ => {}
                    },
                    _ => return Err("Invalid ServiceName type".into()),
                },
                _ => {}
            }
        }

        match cfg.check() {
            false => Err("Not configurated required fields".into()),
            true => {
                if mtx.check() {
                    cfg.multiplex = Some(mtx);
                }
                match tls.check() {
                    false => {}
                    true => {
                        if utls.check() {
                            tls.utls = Some(utls);
                        }
                        if let Some(x) = rlt.enable {
                            if x && rlt.check() {
                                tls.reality = Some(rlt)
                            } else if x && !rlt.check() {
                                return Err("Security = reality, but it doesnt configurated".into());
                            }
                        }
                        cfg.tls = Some(tls);
                    }
                }
                match tfg {
                    TransportConfig::None => {}
                    TransportConfig::WebSocket(x) => {
                        if x.check() {
                            if let Some(ref mut z) = cfg.tls {
                                z.insecure = Some(true);
                            }
                            cfg.flow = Some(Flow::None);
                            cfg.transport = Some(TransportConfig::WebSocket(x));
                        }
                    }
                    TransportConfig::Grpc(x) => {
                        if x.check() {
                            if let Some(ref mut z) = cfg.tls {
                                z.insecure = Some(true);
                            }
                            cfg.flow = Some(Flow::None);
                            cfg.transport = Some(TransportConfig::Grpc(x));
                        }
                    }
                    TransportConfig::Quic(x) => {
                        if x.check() {
                            if let Some(ref mut z) = cfg.tls {
                                z.insecure = Some(true);
                            }
                            cfg.flow = Some(Flow::None);
                            cfg.transport = Some(TransportConfig::Quic(x));
                        }
                    }
                    TransportConfig::Http(x) => {
                        if x.check() {
                            if let Some(ref mut z) = cfg.tls {
                                z.insecure = Some(true)
                            }
                            cfg.flow = Some(Flow::None);
                            cfg.transport = Some(TransportConfig::Http(x))
                        }
                    }
                    TransportConfig::HttpUpdate(x) => {
                        if x.check() {
                            if let Some(ref mut z) = cfg.tls {
                                z.insecure = Some(true)
                            }
                            cfg.flow = Some(Flow::None);
                            cfg.transport = Some(TransportConfig::HttpUpdate(x))
                        }
                    }
                    TransportConfig::Tcp => {}
                }
                Ok(cfg)
            }
        }
    }
}
