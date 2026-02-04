use crate::configurator::shared::listenfields::ListenFields;
use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

use crate::configurator::shared::Network;
use crate::configurator::shared;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DirectConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
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
            config_type: Some("direct".to_string()),
            tag: Some("inbound-direct".to_string()),
            ..Default::default()
        }
    }

    pub fn with_listen(addr: ListenFields) -> Self {
        Self {
            config_type: Some("direct".to_string()),
            tag: Some("inbound-direct".to_string()),
            listen: Some(addr),
            ..Default::default()
        }
    }

    pub fn with_addr(addr: Option<String>, port: Option<u16>) -> Self {
        Self {
            config_type: Some("direct".to_string()),
            tag: Some("inbound-direct".to_string()),
            listen: Some(shared::listenfields::ListenFields::with_listen(addr, port)),
            ..Default::default()
        }
    }

    pub fn check(&self) -> bool {
        !(self.config_type.is_none()
            || self.tag.is_none()
            || self.override_address.is_none()
            || self.override_port.is_none())
    }

    pub fn get_type(&self) -> String {
        self.config_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
