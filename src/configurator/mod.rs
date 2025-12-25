use url::Url;
mod dns;
mod inbound;
mod outbound;
mod route;
mod shared;

use outbound::*;
use serde::{Deserialize, Serialize};

pub trait Config {
    fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize)]
pub struct Configurator {
    outbound: Outbound,
}

impl Configurator {
    pub fn from(input: &str) -> Result<Self, String> {
        let url = Url::parse(input).map_err(|e| e.to_string())?;

        match url.scheme() {
            "vless" => {
                let cfg =
                    outbound::vless::VlessConfig::from_url(input).map_err(|e| e.to_string())?;
                Ok(Configurator {
                    outbound: Outbound::Vless(cfg),
                })
            }
            other => Err(format!("unsupported scheme: {other}")),
        }
    }
}
