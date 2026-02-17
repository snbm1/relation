use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default)]
pub struct LogConfig {
    disable: Option<bool>,
    level: Option<String>,
    output: Option<String>,
    timestamp: Option<bool>,
}

impl LogConfig {
    pub fn new() -> Self {
        Self {
            level: Some("info".to_string()),
            ..Default::default()
        }
    }
}
