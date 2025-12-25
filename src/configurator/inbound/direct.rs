use crate::configurator::shared::listenfields::ListenFields;
use serde::{Deserialize, Serialize};

use crate::configurator::shared::Network;

#[derive(Serialize, Deserialize)]
pub struct DirectConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    listen: Option<ListenFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_port: Option<u16>,
}

impl DirectConfig {
    fn new() -> Self {
        DirectConfig {
            config_type: Some("direct".to_string()),
            tag: Some("direct-inbound".to_string()),
            listen: None,
            network: None,
            override_address: None,
            override_port: None,
        }
    }

    fn check(&self) -> bool {
        !(self.config_type.is_none()
            || self.tag.is_none()
            || self.override_address.is_none()
            || self.override_port.is_none())
    }
}
