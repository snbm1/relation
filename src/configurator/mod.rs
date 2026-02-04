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

use crate::configurator::route::routerule::RouteRule;

#[derive(Serialize, Deserialize)]
pub struct Configurator {
    dns: DnsConfig,
    inbounds: InboundConfig,
    outbounds: OutboundConfig,
    route: RouteConfig,
}

impl Configurator {
    pub fn from(input: &str) -> Result<Self, String> {
        let mut dns_config = DnsConfig::new();
        dns_config
            .add_udp(Some("8.8.8.8".to_string()), None, None)
            .add_local(None);

        let mut inbound_config = InboundConfig::new();
        inbound_config.add_server(Inbound::Mixed(
            mixed::MixedConfig::with_addr(Some("122.0.0.1".to_string()), Some(12334))
                .set_system_proxy(true),
        ));
        inbound_config.add_direct(None);

        let mut outbound_config = OutboundConfig::new();
        outbound_config.add_server_from_url(input).add_direct();

        let mut route_config = RouteConfig::new();

        let mut route = route::routerule::DefaultRouteRule::new();
        route.inbound = Some(vec![
            route::routerule::DefaultRouteRule::get_inbound_tag_by_type(&inbound_config, "direct"),
        ]);
        route_config.add_rule(RouteRule::Default(route));

        let mut route = route::routerule::DefaultRouteRule::new();
        route.inbound = Some(vec![
            route::routerule::DefaultRouteRule::get_inbound_tag_by_type(&inbound_config, "direct"),
        ]);

        let mut route = route::routerule::DefaultRouteRule::new();
        route.inbound = Some(vec![
            route::routerule::DefaultRouteRule::get_inbound_tag_by_type(&inbound_config, "direct"),
        ]);

        Ok(Configurator {
            dns: dns_config,
            inbounds: inbound_config,
            outbounds: outbound_config,
            route: RouteConfig::new(),
        })
    }
}
