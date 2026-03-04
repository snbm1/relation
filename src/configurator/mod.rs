pub mod dns;
pub mod inbound;
pub mod log;
pub mod outbound;
pub mod route;
pub mod shared;

use dns::*;
use inbound::*;
use outbound::*;
use route::*;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::configurator::dns::dnsserver::*;
use crate::configurator::log::LogConfig;
use crate::configurator::route::routerule::DefaultRouteRule;

use core::panic;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Configurator {
    log: LogConfig,
    dns: DnsConfig,
    inbounds: InboundConfig,
    outbounds: OutboundConfig,
    route: RouteConfig,
}

impl Configurator {
    pub fn new() -> Self {
        Self {
            log: LogConfig::new(),
            dns: DnsConfig::new(),
            inbounds: InboundConfig::new(),
            outbounds: OutboundConfig::new(),
            route: RouteConfig::new(),
        }
    }

    pub fn default(&mut self) -> &mut Self {
        self.dns
            .add_udp("8.8.8.8".to_string(), None, None)
            .add_local(None);

        self.inbounds
            .add_server(Inbound::Mixed(
                mixed::MixedConfig::with_addr(Some("127.0.0.1".to_string()), Some(12334))
                    .set_system_proxy(true),
            ))
            .add_direct(None);

        self.outbounds.add_direct();

        self.route
            .auto_detect_interface(true)
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct".to_string())
                    .add_inbound(vec!["dns-direct".to_string()]),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct".to_string())
                    .add_port(vec![53]),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct".to_string())
                    .add_ip_is_private(true),
            );
        self
    }

    pub fn set_outbound_from_url(&mut self, url: &str) -> &mut Self {
        self.outbounds.add_server_from_url(url);
        self.route.set_final_by_type(
            &self.outbounds,
            &self.outbounds.get_types_except_direct()[0],
        );
        self
    }

    pub fn get_list_of_system_proxies(&self) -> Vec<(String, u16, bool)> {
        let mut res = vec![];
        for i in self.inbounds.get_vec_ref() {
            if i.get_system_proxy_status().is_some() {
                res.push(i.get_system_proxy_status().unwrap());
            }
        }
        res
    }

    pub fn get_dns_list(&self) -> Vec<DnsServer> {
        self.dns.get_list()
    }

    pub fn set_route_rules(&mut self, rules: Vec<String>) -> &mut Self {
        let _ = self.route.clean();
        for i in rules {
            let mut rh;
            let ri: Vec<&str> = i.split(":").collect();

            if ri.len() < 2 {
                panic!("[ERROR] Invalid route rules input. Not enough input or incorrect")
            }
            match ri[0] {
                "ib" => {
                    match ri[2] {
                        "r" => rh = Some(DefaultRouteRule::reject_action()),
                        x => {
                            rh = Some(DefaultRouteRule::route_action_by_type(
                                &self.outbounds,
                                x.to_string(),
                            ))
                        }
                    }
                    rh = Some(
                        rh.unwrap()
                            .add_inbound_by_type(&self.inbounds, ri[1].to_string()),
                    );
                }
                "pt" => {
                    match ri[2] {
                        "r" => rh = Some(DefaultRouteRule::reject_action()),
                        x => {
                            rh = Some(DefaultRouteRule::route_action_by_type(
                                &self.outbounds,
                                x.to_string(),
                            ))
                        }
                    }
                    rh = Some(rh.unwrap().add_port(vec![ri[1].parse().unwrap()]));
                }
                _ => rh = None,
            }
            if let Some(value) = rh {
                self.route.add_default_rule(value);
            }
        }
        self
    }

    pub fn set_dns_servers(&mut self, dns: Vec<String>) -> &mut Self {
        let _ = self.dns.clean();
        for i in dns {
            let dh;
            let df: Vec<&str> = i.split(":").collect();
            let df_port;
            let df_type;
            let df_addr;

            if df.len() < 3 {
                df_port = None;
                if df.len() < 2 {
                    df_type = "udp";
                    df_addr = df[0];
                } else {
                    df_type = df[0];
                    df_addr = df[1];
                }
            } else {
                df_type = df[0];
                df_port = Some(df[2].parse::<u16>().unwrap());
                df_addr = df[1];
            }

            match df_type {
                "tcp" => {
                    dh = DnsServer::Tcp(DnsServerTcp::with_server(df_addr.to_string(), df_port))
                }
                "udp" => {
                    dh = DnsServer::Udp(DnsServerUdp::with_server(df_addr.to_string(), df_port))
                }
                _ => panic!("[ERROR] Cant parse that type of dns yet"),
            }
            self.dns.add_server(dh);
        }
        self
    }

    pub fn set_log(&mut self, level: String, output: Option<PathBuf>) -> &mut Self {
        self.log.set_level(level);
        if let Some(value) = output {
            self.log.set_output(value);
        }
        self
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn save_to_file(&self, dir: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let tag = &self.outbounds.get_tags_except_direct()[0];

        let file_path = dir.join(format!("{tag}.json"));

        let file = File::create(&file_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)?;

        Ok(self.outbounds.get_tags_except_direct()[0].clone())
    }

    pub fn load_from_file(
        &mut self,
        path: PathBuf,
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let configurator: Self = serde_json::from_reader(reader)?;

        *self = configurator;
        Ok(self)
    }
}
