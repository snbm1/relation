pub mod direct;
pub mod mixed;
pub mod tun;

use serde::{Deserialize, Serialize};

use crate::configurator::{
    inbound::{direct::DirectConfig, mixed::MixedConfig, tun::TunConfig},
    shared::listenfields::ListenFields,
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

    pub fn add_tun(
        &mut self,
        address: Vec<String>,
        auto_route: bool,
        strict_route: bool,
        mtu: u16,
    ) -> &mut Self {
        self.servers.push(Inbound::Tun(
            TunConfig::new()
                .add_ip_list(address)
                .set_mtu(mtu)
                .set_auto_route(auto_route)
                .set_strict_route(strict_route),
        ));
        self
    }

    pub fn get_ref_by_tag(&self, tag: &str) -> Option<&Inbound> {
        self.servers.iter().find(|x| x.get_tag() == tag)
    }

    pub fn get_mut_by_tag(&mut self, tag: &str) -> Option<&mut Inbound> {
        self.servers.iter_mut().find(|x| x.get_tag() == tag)
    }

    pub fn get_tag_by_type(&self, name: &str) -> Option<String> {
        self.servers
            .iter()
            .find(|x| x.get_type() == name)
            .map(|x| x.get_tag())
    }

    pub fn get_vec_ref(&self) -> &Vec<Inbound> {
        &self.servers
    }

    pub fn clean(&mut self) -> &mut Self {
        *self = Self::new();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Inbound {
    Direct(direct::DirectConfig),
    Mixed(mixed::MixedConfig),
    Tun(tun::TunConfig),
}

impl Inbound {
    pub fn get_tag(&self) -> String {
        match self {
            Inbound::Direct(cfg) => cfg.get_tag(),
            Inbound::Mixed(cfg) => cfg.get_tag(),
            Inbound::Tun(cfg) => cfg.get_tag(),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Inbound::Direct(cfg) => cfg.get_type(),
            Inbound::Mixed(cfg) => cfg.get_type(),
            Inbound::Tun(cfg) => cfg.get_type(),
        }
    }
    pub fn get_system_proxy_status(&self) -> Option<(String, u16, bool)> {
        match self {
            Inbound::Direct(cfg) => None,
            Inbound::Mixed(cfg) => {
                if cfg.is_system_proxy() {
                    let socks_status = if cfg.get_type() == "socks" {
                        true
                    } else {
                        false
                    };
                    Some((
                        cfg.get_address().unwrap(),
                        cfg.get_address_port().unwrap(),
                        socks_status,
                    ))
                } else {
                    None
                }
            }
            Inbound::Tun(cfg) => None,
        }
    }
}
