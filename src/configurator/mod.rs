pub mod dns;
pub mod experimental;
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

use crate::configurator::dns::dnsserver::*;
use crate::configurator::experimental::ExperimentalConfig;
use crate::configurator::inbound::tun::TunConfig;
use crate::configurator::log::LogConfig;
use crate::configurator::route::routerule::DefaultRouteRule;
use crate::configurator::route::routerule::LogicalRouteRule;

use anyhow::{Context, Result, anyhow};
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
    experimental: ExperimentalConfig,
}

impl Configurator {
    pub fn new() -> Self {
        Self {
            log: LogConfig::new(),
            dns: DnsConfig::new(),
            inbounds: InboundConfig::new(),
            outbounds: OutboundConfig::new(),
            route: RouteConfig::new(),
            experimental: ExperimentalConfig::new(),
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
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct")
                    .add_inbound(vec!["dns-direct"]),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct").add_port(53),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct")
                    .add_ip_is_private(true),
            );
        self
    }

    pub fn default_tun(&mut self) -> &mut Self {
        self.dns
            .add_udp("8.8.8.8".to_string(), None, None)
            .add_local(None);

        self.inbounds
            .add_server(Inbound::Tun(
                TunConfig::new()
                    .set_auto_route(true)
                    .set_auto_redirect(true)
                    .set_strict_route(true)
                    .set_stack("system".to_string())
                    .set_mtu(1500)
                    .add_ip("198.18.0.1/30"),
            ))
            .add_direct(None);

        self.outbounds.add_direct();

        self.route
            .auto_detect_interface(true)
            .add_default_rule(DefaultRouteRule::sniff_action("1s"))
            .add_logical_rule(
                LogicalRouteRule::or()
                    .set_hijack_dns_action()
                    .add_rule(DefaultRouteRule::new().add_port(53))
                    .add_rule(DefaultRouteRule::new().add_protocol("dns")),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct")
                    .add_ip_is_private(true),
            )
            .set_default_domain_resolver_by_type(&self.dns, "local");
        self
    }

    pub fn set_outbound_from_url(&mut self, url: &str) -> Result<&mut Self> {
        self.outbounds.add_server_from_url(url)?;
        if self.get_inbounds_types().contains(&"tun".to_string()) {
            self.route.add_default_rule(
                DefaultRouteRule::route_action_by_type(&self.outbounds, "direct")
                    .add_ip_cidr(&self.outbounds.get_server_addr_by_type("vless")),
            );
        }
        self.route.set_final_by_type(
            &self.outbounds,
            &self.outbounds.get_types_except_direct().first().unwrap(),
        );
        Ok(self)
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

    pub fn get_inbounds_types(&self) -> Vec<String> {
        let mut res = vec![];
        for i in self.inbounds.get_vec_ref() {
            res.push(i.get_type());
        }
        res
    }

    pub fn get_dns_list(&self) -> Vec<DnsServer> {
        self.dns.get_list()
    }

    /// Set route rules in format: <ACTION>:<TYPE>:<VALUE>
    /// ACTIONS:
    /// "r"      -> Reject
    /// "h"      -> Hijack-dns
    /// "s"      -> Shiff
    /// "<NAME>" -> Route outbound with NAME type (for example "vless")
    ///
    /// TYPES:                 VALUE type:
    /// "ib" -> inbound type   `str`
    /// "pt" -> port           `u16`
    ///
    /// SPECIFIC:
    /// "s":<VALUE>            `str`
    /// for example "1s" NOT just "1"
    pub fn add_route_rules(&mut self, rules: Vec<String>) -> Result<&mut Self> {
        for i in rules {
            let mut rh;
            let ri: Vec<&str> = i.split(":").collect();
            let mut value_flag = false;

            match *ri.first().context("Incorrect route rules manage input")? {
                "r" => {
                    rh = Some(DefaultRouteRule::reject_action());
                    value_flag = true;
                }
                "h" => rh = Some(DefaultRouteRule::hijack_dns_action()),
                "s" => rh = Some(DefaultRouteRule::sniff_action("1s")),
                x => {
                    rh = Some(DefaultRouteRule::route_action_by_type(&self.outbounds, x));
                    value_flag = true;
                }
            }
            if value_flag {
                match *ri.get(1).context("Incorrect route rules manage input")? {
                    "ib" => {
                        rh = Some(rh.unwrap().add_inbound_by_type(&self.inbounds, ri[2]));
                    }
                    "pt" => {
                        rh = Some(rh.unwrap().add_port(ri[2].parse().unwrap()));
                    }
                    _ => rh = None,
                }
            } else if ri[0] == "s" && ri.len() == 2 {
                rh = Some(DefaultRouteRule::sniff_action(ri[1]))
            }
            if let Some(value) = rh {
                self.route.add_default_rule(value);
            }
        }
        Ok(self)
    }

    /// Set route rules in format: <ACTION>:<VALUE1>:<VALUE2>
    ///
    ///If action contains one value you need only:
    ///     <ACTION>:<VALUE>
    ///
    /// ACTION:             VALUES:
    /// "r"    -> Remove    `usize`         -> remove by index <VALUE1>
    /// "m"    -> Move      `usize`:`usize`   -> move from <VALUE1> to <VALUE2>
    pub fn manage_route_rules(&mut self, rules: Vec<String>) -> Result<&mut Self> {
        for i in rules {
            let ri: Vec<&str> = i.split(":").collect();
            match *ri.first().context("Incorrect route rules manage input")? {
                "r" => {
                    let _ = self.route.remove_rule(
                        ri.get(1)
                            .context("Incorrect route rules manage input")?
                            .parse()?,
                    );
                }
                "m" => {
                    self.route.move_rule(
                        ri.get(1)
                            .context("Incorrect route rules manage input")?
                            .parse()?,
                        ri.get(2)
                            .context("Incorrect route rules manage input")?
                            .parse()?,
                    );
                }
                _ => {}
            }
        }

        Ok(self)
    }

    pub fn set_dns_servers(&mut self, dns: Vec<String>) -> Result<&mut Self> {
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
                df_port = Some(df[2].parse::<u16>()?);
                df_addr = df[1];
            }

            match df_type {
                "tcp" => {
                    dh = DnsServer::Tcp(DnsServerTcp::with_server(df_addr.to_string(), df_port))
                }
                "udp" => {
                    dh = DnsServer::Udp(DnsServerUdp::with_server(df_addr.to_string(), df_port))
                }
                _ => return Err(anyhow!("Cant parse that type of dns yet")),
            }
            self.dns.add_server(dh);
        }
        Ok(self)
    }

    pub fn set_log(&mut self, level: String, output: Option<PathBuf>) -> &mut Self {
        self.log.set_level(level);
        if let Some(value) = output {
            self.log.set_output(value);
        }
        self
    }

    pub fn set_tun(&mut self) -> &mut Self {
        self.inbounds
            .clean()
            .add_server(Inbound::Tun(
                TunConfig::new()
                    .set_auto_route(true)
                    .set_auto_redirect(true)
                    .set_strict_route(true)
                    .set_stack("system".to_string())
                    .set_mtu(1500)
                    .add_ip("198.18.0.1/30"),
            ))
            .add_direct(None);
        self
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn get_outbound_tag(&self) -> Result<String> {
        Ok(self
            .outbounds
            .get_tags_except_direct()
            .first()
            .context("No outbounds")?
            .clone())
    }

    pub fn save_to_file(&self, dir: PathBuf, file_name: String) -> Result<String> {
        let file_path = dir.join(format!("{file_name}.json"));

        let file = File::create(&file_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)?;

        Ok(self.outbounds.get_tags_except_direct()[0].clone())
    }

    pub fn load_from_file(&mut self, path: PathBuf) -> Result<&mut Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let configurator: Self = serde_json::from_reader(reader)?;

        *self = configurator;
        Ok(self)
    }

    pub fn clean(&mut self) -> &mut Self {
        self.log.clean();
        self.dns.clean();
        self.inbounds.clean();
        self.outbounds.clean();
        self.route.clean();
        self
    }
}
