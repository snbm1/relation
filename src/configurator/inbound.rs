pub mod direct;
pub mod mixed;

use serde::{Deserialize, Serialize};

use crate::configurator::{
    inbound::{direct::DirectConfig, mixed::MixedConfig},
    shared::{listenfields::ListenFields},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InboundConfig {
    pub servers: Vec<Inbound>,
}

impl InboundConfig {
    pub fn new() -> Self {
        Self { servers: vec![] }
    }

    pub fn add_server(&mut self, server: Inbound) -> &mut Self {
        self.servers.push(server);
        self
    }

    pub fn add_direct(&mut self, listen: Option<ListenFields>) -> &mut Self {
        if let Some(value) = listen {
            self.servers
                .push(Inbound::Direct(DirectConfig::with_listen(value)));
        } else {
            self.servers.push(Inbound::Direct(DirectConfig::new()));
        }
        self
    }

    pub fn add_mixed(&mut self, listen: Option<ListenFields>) -> &mut Self {
        if let Some(value) = listen {
            self.servers
                .push(Inbound::Mixed(MixedConfig::with_listen(value)));
        } else {
            self.servers.push(Inbound::Mixed(MixedConfig::new()));
        }
        self
    }

    pub fn get_ref_by_tag(&self, tag: &str) -> Option<&Inbound> {
        self.servers.iter().find(|x| x.get_tag() == tag)
    }

    pub fn get_mut_by_tag(&mut self, tag: &str) -> Option<&mut Inbound> {
        self.servers.iter_mut().find(|x| x.get_tag() == tag)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Inbound {
    Direct(direct::DirectConfig),
    Mixed(mixed::MixedConfig),
}

impl Inbound {
    pub fn get_tag(&self) -> String {
        match self {
            Inbound::Direct(cfg) => cfg.get_tag(),
            Inbound::Mixed(cfg) => cfg.get_tag(),
        }
    }
}
