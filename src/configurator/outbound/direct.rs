use serde::{Deserialize, Serialize};

use crate::configurator::shared::dialfields::DialFields;

#[derive(Serialize, Deserialize)]
pub struct DirectConfig {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    config_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    dial: Option<DialFields>,
}

impl DirectConfig {
    fn new() -> Self {
        DirectConfig {
            config_type: Some("direct".to_string()),
            tag: Some("direct-outbound".to_string()),
            dial: None,
        }
    }

    fn check(&self) -> bool {
        !(self.config_type.is_none() || self.tag.is_none())
    }
}
