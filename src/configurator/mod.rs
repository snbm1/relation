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

                let mut lp = shared::listenfields::ListenFields::new();
                lp.listen = Some("127.0.0.1".to_string());
                lp.listen_port = Some(12334);

                let mut mxd = inbound::mixed::MixedConfig::new();
                mxd.set_system_proxy = Some(true);
                mxd.listen = Some(lp);

                Ok(Configurator {
                    dns: DnsConfig::new(),
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
