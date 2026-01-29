pub mod direct;
pub mod vless;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::configurator::outbound::{direct::DirectConfig, vless::VlessConfig};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct OutboundConfig {
    pub servers: Vec<Outbound>,
}

impl OutboundConfig {
    pub fn new() -> Self {
        Self { servers: vec![] }
    }

    pub fn add_server(&mut self, server: Outbound) -> &mut Self {
        self.servers.push(server);
        self
    }

    pub fn add_server_from_url(&mut self, url: &str) -> &mut Self {
        match Url::parse(url).unwrap().scheme() {
            "vless" => {
                self.add_server(Outbound::Vless(
                    VlessConfig::from_url(url).expect("[ERROR] Cant parse Vless config from url."),
                ));
            }
            _ => {}
        }
        self
    }

    pub fn add_direct(&mut self) -> &mut Self {
        self.servers.push(Outbound::Direct(DirectConfig::new()));
        self
    }

    pub fn get_ref_by_tag(&self, tag: &str) -> Option<&Outbound> {
        self.servers.iter().find(|x| x.get_tag() == tag)
    }

    pub fn get_mut_by_tag(&mut self, tag: &str) -> Option<&mut Outbound> {
        self.servers.iter_mut().find(|x| x.get_tag() == tag)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Outbound {
    Direct(direct::DirectConfig),
    Vless(vless::VlessConfig),
}

impl Outbound {
    pub fn get_tag(&self) -> String {
        match self {
            Outbound::Direct(cfg) => cfg.get_tag(),
            Outbound::Vless(cfg) => cfg.get_tag(),
        }
    }
}
