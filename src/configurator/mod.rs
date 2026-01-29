use url::Url;
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

pub trait Config {
    fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct Configurator {
    dns: DnsConfig,
    inbounds: Vec<Inbound>,
    outbounds: Vec<Outbound>,
    route: RouteConfig,
}

impl Configurator {
    pub fn from(input: &str) -> Result<Self, String> {
        let url = Url::parse(input).map_err(|e| e.to_string())?;

        match url.scheme() {
            "vless" => {
                let cfg =
                    outbound::vless::VlessConfig::from_url(input).map_err(|e| e.to_string())?;

                let lp = shared::listenfields::ListenFields::with_listen(
                    Some("127.0.0.1".to_string()),
                    Some(12334),
                );

                let mxd = mixed::MixedConfig::with_listen(lp).ssp();

                let mut dns_config = DnsConfig::new();
                dns_config.add_udp(Some("8.8.8.8".to_string()), None, None).add_local(None);

                Ok(Configurator {
                    dns: dns_config,
                    inbounds: vec![
                        Inbound::Mixed(mxd),
                        Inbound::Direct(inbound::direct::DirectConfig::new()),
                    ],
                    outbounds: vec![Outbound::Vless(cfg)],
                    route: RouteConfig::new(),
                })
            }
            other => Err(format!("unsupported scheme: {other}")),
        }
    }
}
