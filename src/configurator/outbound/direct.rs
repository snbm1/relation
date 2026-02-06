use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

use crate::configurator::shared::dialfields::DialFields;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DirectConfig {
    #[serde(rename = "type")]
    config_type: String,
    tag: String,
    #[serde(flatten)]
    dial: Option<DialFields>,
}

impl DirectConfig {
    pub fn new() -> Self {
        DirectConfig {
            config_type: "direct".to_string(),
            tag: "outbound-direct".to_string(),
            ..Default::default()
        }
    }

    fn check(&self) -> bool {
        !(self.config_type == "" || self.tag == "")
    }

    pub fn get_type(&self) -> String {
        self.config_type.clone()
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}
