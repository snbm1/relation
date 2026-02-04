use crate::configurator::shared::listenfields::ListenFields;
use serde::{Deserialize, Serialize};
use rellib::auto_skip_none;

use crate::configurator::shared;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MixedConfig {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub tag: Option<String>,
    #[serde(flatten)]
    pub listen: Option<ListenFields>,
    pub set_system_proxy: Option<bool>,
}

impl MixedConfig {
    pub fn new() -> Self {
        MixedConfig {
            config_type: Some("mixed".to_string()),
            tag: Some("inbound-mixed".to_string()),
            ..Default::default()
        }
    }

    pub fn with_listen(addr: ListenFields) -> Self {
        Self {
            config_type: Some("mixed".to_string()),
            tag: Some("inbound-mixed".to_string()),
            listen: Some(addr),
            set_system_proxy: None,
        }
    }

    pub fn with_addr(addr: Option<String>, port: Option<u16>) -> Self {
        Self {
            config_type: Some("mixed".to_string()),
            tag: Some("inbound-mixed".to_string()),
            listen: Some(shared::listenfields::ListenFields::with_listen(addr, port)),
            ..Default::default()
        }
    }

    pub fn set_system_proxy(mut self, value: bool) -> Self {
        self.set_system_proxy = Some(value);
        self
    }

    pub fn check(&self) -> bool {
        !(self.listen.is_none())
    }

    pub fn get_type(&self) -> String {
        self.config_type.clone().expect("[ERROR] No type")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone().expect("[ERROR] No tag")
    }
}
