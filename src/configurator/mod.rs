mod dns;
mod inbound;
mod outbound;
mod route;
mod shared;

use dns::*;
use inbound::*;
use outbound::*;
use route::*;
use serde::{Deserialize, Serialize};

use crate::configurator::route::routerule::DefaultRouteRule;

use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
pub struct Configurator {
    dns: DnsConfig,
    inbounds: InboundConfig,
    outbounds: OutboundConfig,
    route: RouteConfig,
}

impl Configurator {
    pub fn new() -> Self {
        Self {
            dns: DnsConfig::new(),
            inbounds: InboundConfig::new(),
            outbounds: OutboundConfig::new(),
            route: RouteConfig::new(),
        }
    }

    pub fn default(&mut self) -> &mut Self {
        self.dns
            .add_udp(Some("8.8.8.8".to_string()), None, None)
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

        self.route.set_final_by_type(&self.outbounds, "vless");
        self
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("config.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open("config.json")?;
        let reader = BufReader::new(file);
        let configurator = serde_json::from_reader(reader)?;
        Ok(configurator)
    }
}
