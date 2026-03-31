use crate::configurator::shared::listenfields::ListenFields;
use macros::auto_skip_none;
use serde::{Deserialize, Serialize};

use crate::configurator::shared;
use crate::configurator::shared::Network;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DirectConfig {
    pub tag: Option<String>,
    #[serde(flatten)]
    pub listen: Option<ListenFields>,
    pub network: Option<Network>,
    pub override_address: Option<String>,
    pub override_port: Option<u16>,
}

impl DirectConfig {
    pub fn new() -> Self {
        DirectConfig {
            tag: Some("inbound-direct".to_string()),
            ..Default::default()
        }
    }

    pub fn with_listen(addr: ListenFields) -> Self {
        Self {
            tag: Some("inbound-direct".to_string()),
            listen: Some(addr),
            ..Default::default()
        }
    }

    pub fn with_addr(addr: Option<String>, port: Option<u16>) -> Self {
        Self {
            tag: Some("inbound-direct".to_string()),
            listen: Some(shared::listenfields::ListenFields::with_listen(addr, port)),
            ..Default::default()
        }
    }

    pub fn get_address_port(&self) -> Option<u16> {
        if let Some(x) = &self.listen {
            return x.listen_port.clone();
        }
        None
    }

    pub fn check(&self) -> bool {
        !(self.tag.is_none() || self.override_address.is_none() || self.override_port.is_none())
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
