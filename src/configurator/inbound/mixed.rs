use crate::configurator::shared::listenfields::ListenFields;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MixedConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<ListenFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_system_proxy: Option<bool>,
}

impl MixedConfig {
    pub fn new() -> Self {
        MixedConfig {
            config_type: Some("mixed".to_string()),
            tag: Some("mixed-inbound".to_string()),
            listen: None,
            set_system_proxy: None,
        }
    }

    pub fn check(&self) -> bool {
        !(self.config_type.is_none() || self.tag.is_none())
    }
}
