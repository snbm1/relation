use core::str;
use std::error;

use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

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
#[derive(Debug)]
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
    _bool(bool),
    _u16(u16),
    _string(String),
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
struct VlessConfig {
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
    fn new(value: Vec<(PossibleKeys, PossibleValues)>) -> Result<Self, Box<dyn error::Error>> {
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
                        PossibleValues::_string(x) => match x.as_str() {
                            "udp" => Some(Network::udp),
                            "tcp" => Some(Network::tcp),
                            _ => None,
                        },
                        _ => return Err("Invalid type".into()),
                    };
                }

                PossibleKeys::Flow => {
                    flow = Some(match val {
                        PossibleValues::_string(x) => match x.as_str() {
                            "xtls-rprx-vision" => Flow::xtls_rprx_vision,
                            _ => Flow::xtls_rprx_vision,
                        },
                        _ => return Err("Invalid flow".into()),
                    });
                }

                PossibleKeys::Server => {
                    server = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid server".into()),
                    });
                }

                PossibleKeys::ServerPort => {
                    server_port = Some(match val {
                        PossibleValues::_u16(x) => x,
                        _ => return Err("Invalid port".into()),
                    });
                }

                PossibleKeys::UUID => {
                    uuid = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid uuid".into()),
                    });
                }
                PossibleKeys::Sni => {
                    tls_server_name = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid tls server name".into()),
                    })
                }
                PossibleKeys::Fp => {
                    tls_utls = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid fingerprint name".into()),
                    })
                }
                PossibleKeys::Pbk => {
                    tls_reality_pbk = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid public key".into()),
                    })
                }
                PossibleKeys::Sid => {
                    tls_reality_sid = Some(match val {
                        PossibleValues::_string(x) => x,
                        _ => return Err("Invalid short id".into()),
                    })
                }
                _ => {}
            }
        }
        if (tls_server_name != None && tls_reality_sid != None && tls_reality_pbk != None) {
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
        } else if (tls_server_name != None || tls_reality_sid != None || tls_reality_pbk != None) {
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
}

pub fn vless2json(input: String) {
    let parsed_input = Url::parse(&input).unwrap();
    println!("{}", parsed_input.scheme());
    println!("{}", parsed_input.username());
    println!("{}", parsed_input.host().unwrap());
    println!("{}", parsed_input.port().unwrap());
    for i in parsed_input.query().unwrap().split("&") {
        println!("  {}", i);
    }
    println!("{}", parsed_input.fragment().unwrap());

    let mut values: Vec<(PossibleKeys, PossibleValues)> = Vec::new();
    values.push((
        PossibleKeys::Server,
        PossibleValues::_string(String::from(parsed_input.host_str().unwrap())),
    ));
    values.push((
        PossibleKeys::ServerPort,
        PossibleValues::_u16(parsed_input.port().unwrap_or(443)),
    ));
    values.push((
        PossibleKeys::UUID,
        PossibleValues::_string(String::from(parsed_input.username())),
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
                    PossibleValues::_string(String::from(j[1])),
                )),
                "flow" => values.push((
                    PossibleKeys::Flow,
                    PossibleValues::_string(String::from(j[1])),
                )),
                "sni" => values.push((
                    PossibleKeys::Sni,
                    PossibleValues::_string(String::from(j[1])),
                )),
                "fp" => values.push((
                    PossibleKeys::Fp,
                    PossibleValues::_string(String::from(j[1])),
                )),
                "pbk" => values.push((
                    PossibleKeys::Pbk,
                    PossibleValues::_string(String::from(j[1])),
                )),
                "sid" => values.push((
                    PossibleKeys::Sid,
                    PossibleValues::_string(String::from(j[1])),
                )),
                _ => {}
            }
        }
    }
    println!("{:?}", values);
    let config = VlessConfig::new(values).unwrap();
    let j = serde_json::to_string(&config).unwrap();
    println!("{}", j);
}
