use serde::{Deserialize, Serialize};
use url::Url;

use crate::configurator::Config;

#[derive(Serialize, Deserialize, Default, Debug)]
enum Flow {
    #[default]
    #[serde(rename = "xtls-rprx-vision")]
    xtls_rprx_vision,
}

#[derive(Serialize, Deserialize, Debug)]
enum Network {
    tcp,
    udp,
}

#[derive(Serialize, Deserialize, Default, Debug)]
enum PacketEncoding {
    none,
    packetaddr,
    #[default]
    xudp,
}

#[warn(dead_code)]
enum PossibleKeys {
    Server,
    ServerPort,
    UUID,
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
}
#[derive(Debug)]
enum PossibleValues {
    Bool(bool),
    U16(u16),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct RealityConfig {
    enable: bool,
    public_key: String,
    short_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct UtlsConfig {
    enable: bool,
    fingerprint: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct TlsConfig {
    enable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_sni: Option<bool>,
    server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    utls: Option<UtlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reality: Option<RealityConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VlessConfig {
    server: String,
    server_port: u16,
    uuid: String,
    flow: Flow,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<TlsConfig>,
    packet_encoding: PacketEncoding,
}

impl VlessConfig {
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
            PossibleKeys::UUID,
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
        let mut server: Option<String> = None;
        let mut server_port: Option<u16> = None;
        let mut uuid: Option<String> = None;
        let mut flow: Option<Flow> = None;
        let mut network: Option<Network> = None;

        let mut tls_server_name: Option<String> = None;
        let mut tls_utls: Option<String> = None;
        let mut tls_reality_pbk: Option<String> = None;
        let mut tls_reality_sid: Option<String> = None;

        for (key, val) in value {
            match key {
                PossibleKeys::Type => {
                    network = match val {
                        PossibleValues::String(x) => match x.as_str() {
                            "udp" => Some(Network::udp),
                            "tcp" => Some(Network::tcp),
                            _ => None,
                        },
                        _ => return Err("Invalid type".into()),
                    };
                }

                PossibleKeys::Flow => {
                    flow = Some(match val {
                        PossibleValues::String(x) => match x.as_str() {
                            "xtls-rprx-vision" => Flow::xtls_rprx_vision,
                            _ => Flow::xtls_rprx_vision,
                        },
                        _ => return Err("Invalid flow".into()),
                    });
                }

                PossibleKeys::Server => {
                    server = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid server".into()),
                    });
                }

                PossibleKeys::ServerPort => {
                    server_port = Some(match val {
                        PossibleValues::U16(x) => x,
                        _ => return Err("Invalid port".into()),
                    });
                }

                PossibleKeys::UUID => {
                    uuid = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid uuid".into()),
                    });
                }
                PossibleKeys::Sni => {
                    tls_server_name = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid tls server name".into()),
                    })
                }
                PossibleKeys::Fp => {
                    tls_utls = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid fingerprint name".into()),
                    })
                }
                PossibleKeys::Pbk => {
                    tls_reality_pbk = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid public key".into()),
                    })
                }
                PossibleKeys::Sid => {
                    tls_reality_sid = Some(match val {
                        PossibleValues::String(x) => x,
                        _ => return Err("Invalid short id".into()),
                    })
                }
                _ => {}
            }
        }
        if !(tls_server_name.is_none() || tls_reality_sid.is_none() || tls_reality_pbk.is_none()) {
            Ok(VlessConfig {
                server: server.ok_or("server missing")?,
                server_port: server_port.ok_or("port missing")?,
                uuid: uuid.ok_or("uuid missing")?,
                flow: flow.ok_or("flow missing")?,
                network,
                tls: Some(TlsConfig {
                    enable: true,
                    disable_sni: None,
                    server_name: tls_server_name.unwrap(),
                    utls: Some(UtlsConfig {
                        enable: true,
                        fingerprint: tls_utls.unwrap(),
                    }),
                    reality: Some(RealityConfig {
                        enable: true,
                        public_key: tls_reality_pbk.unwrap(),
                        short_id: tls_reality_sid.unwrap(),
                    }),
                }),
                packet_encoding: PacketEncoding::default(),
            })
        } else if tls_server_name.is_none()
            || tls_reality_sid.is_none()
            || tls_reality_pbk.is_none()
        {
            println!("Tls not full configurated");
            Ok(VlessConfig {
                server: server.ok_or("server missing")?,
                server_port: server_port.ok_or("port missing")?,
                uuid: uuid.ok_or("uuid missing")?,
                flow: flow.ok_or("flow missing")?,
                network,
                tls: None,
                packet_encoding: PacketEncoding::default(),
            })
        } else {
            Ok(VlessConfig {
                server: server.ok_or("server missing")?,
                server_port: server_port.ok_or("port missing")?,
                uuid: uuid.ok_or("uuid missing")?,
                flow: flow.ok_or("flow missing")?,
                network,
                tls: None,
                packet_encoding: PacketEncoding::default(),
            })
        }
    }

    fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }
}
