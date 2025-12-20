use std::error;

use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

#[derive(Serialize, Deserialize, Default)]
enum Flow {
    #[default]
    #[serde(rename = "xtls-rprx-vision")]
    xtls_rprx_vision,
}

#[derive(Serialize, Deserialize, Default)]
enum Network {
    #[default]
    Tcp,
    Udp,
}

#[derive(Serialize, Deserialize, Default)]
enum PacketEncoding {
    none,
    packetaddr,
    #[default]
    xudp,
}
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
enum PossibleValues {
    _bool(bool),
    _u16(u16),
    _string(String),
}

#[derive(Serialize, Deserialize, Default)]
struct TlsConfig {
    enable: bool,
    disable_sni: bool,
    server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    utls: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reality: Option<(String, String)>,
    packet_encoding: PacketEncoding,
}

#[derive(Serialize, Deserialize)]
struct VlessConfig {
    server: String,
    server_port: u16,
    uuid: String,
    flow: Flow,
    network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls: Option<TlsConfig>,
}

impl VlessConfig {
    fn new(value: Vec<(PossibleKeys, PossibleValues)>) -> Result<Self, Box<dyn error::Error>> {
        let mut server: Option<String> = None;
        let mut server_port: Option<u16> = None;
        let mut uuid: Option<String> = None;
        let mut flow: Option<Flow> = None;
        let mut network: Option<Network> = None;
        let mut tls: Option<TlsConfig> = None;

        for (key, val) in value {
            match key {
                PossibleKeys::Type => {
                    network = Some(match val {
                        PossibleValues::_string(x) => match x.as_str() {
                            "udp" => Network::Udp,
                            "tcp" => Network::Tcp,
                            _ => return Err("Invalid network".into()),
                        },
                        _ => return Err("Invalid type".into()),
                    });
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

                _ => {}
            }
        }

        Ok(VlessConfig {
            server: server.ok_or("server missing")?,
            server_port: server_port.ok_or("port missing")?,
            uuid: uuid.ok_or("uuid missing")?,
            flow: flow.ok_or("flow missing")?,
            network: network.ok_or("network missing")?,
            tls: None,
        })
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

    let mut config: VlessConfig;
}
