use macros::auto_skip_none;
use serde::{Deserialize, Serialize};

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExperimentalConfig {
    pub clash_api: Option<ClashApiConfig>,
}

impl ExperimentalConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[auto_skip_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClashApiConfig {
    pub external_controller: Option<String>,
}

impl ClashApiConfig {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
