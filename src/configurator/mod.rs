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

    pub fn from(input: &str) -> Result<Self, String> {
        let mut dns_config = DnsConfig::new();
        dns_config
            .add_udp(Some("8.8.8.8".to_string()), None, None)
            .add_local(None);

        let mut inbound_config = InboundConfig::new();
        inbound_config
            .add_server(Inbound::Mixed(
                mixed::MixedConfig::with_addr(Some("127.0.0.1".to_string()), Some(12334))
                    .set_system_proxy(true),
            ))
            .add_direct(None);

        let mut outbound_config = OutboundConfig::new();
        outbound_config.add_server_from_url(input).add_direct();

        let mut route_config = RouteConfig::new();
        route_config
            .auto_detect_interface(true)
            .set_final_by_type(&outbound_config, "vless")
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&outbound_config, "direct".to_string())
                    .add_inbound(vec!["dns-direct".to_string()]),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&outbound_config, "direct".to_string())
                    .add_port(vec![53]),
            )
            .add_default_rule(
                DefaultRouteRule::route_action_by_type(&outbound_config, "direct".to_string())
                    .add_ip_is_private(true),
            );

        Ok(Configurator {
            dns: dns_config,
            inbounds: inbound_config,
            outbounds: outbound_config,
            route: route_config,
        })
    }

    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
}
