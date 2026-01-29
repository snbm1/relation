use serde::{Deserialize, Serialize};

use crate::configurator::shared::dialfields::DialFields;

#[derive(Serialize, Deserialize, Default, Debug)]
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
    pub fn new() -> Self {
        DirectConfig {
            config_type: Some("direct".to_string()),
            tag: Some("outbound-direct".to_string()),
            ..Default::default()
        }
    }

    fn check(&self) -> bool {
        !(self.config_type.is_none() || self.tag.is_none())
    }

    pub fn get_type(&self) -> String {
        self.config_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
