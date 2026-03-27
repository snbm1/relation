use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

use crate::configurator::shared::dialfields::DialFields;

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DirectConfig {
    tag: String,
    #[serde(flatten)]
    dial: Option<DialFields>,
}

impl DirectConfig {
    pub fn new() -> Self {
        DirectConfig {
            tag: "outbound-direct".to_string(),
            ..Default::default()
        }
    }

    fn check(&self) -> bool {
        !(self.tag == "")
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }
}
