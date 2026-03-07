use std::path::PathBuf;

use rellib::auto_skip_none;
use serde::{Deserialize, Serialize};

#[auto_skip_none]
#[derive(Serialize, Deserialize, Default)]
pub struct LogConfig {
    disable: Option<bool>,
    level: String,
    output: Option<String>,
    timestamp: Option<bool>,
}

impl LogConfig {
    pub fn new() -> Self {
        Self {
            level: "info".to_string(),
            ..Default::default()
        }
    }

    pub fn set_output(&mut self, file: PathBuf) -> &mut Self {
        self.output = Some(file.to_str().unwrap().to_string());
        self
    }

    pub fn set_level(&mut self, level: String) -> &mut Self {
        self.level = level;
        self
    }

    pub fn clean(&mut self) -> &mut Self {
        *self = Self::new();
        self
    }
}
