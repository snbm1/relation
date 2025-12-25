use crate::configurator::shared::listenfields::ListenFields;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MixedConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    listen: Option<ListenFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_system_proxy: Option<bool>,
}

impl MixedConfig {
    fn new() -> Self {
        MixedConfig {
            config_type: Some("direct".to_string()),
            tag: Some("direct-outbound".to_string()),
            listen: None,
            set_system_proxy: None,
        }
    }

    fn check(&self) -> bool {
        !(self.config_type.is_none() || self.tag.is_none())
    }
}
