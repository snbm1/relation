use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::configurator::Configurator;

pub struct DataManager {
    data_dir: PathBuf,
    handler: Configurator
}

impl DataManager {
    /// ~/.local/share/<app_name>
    pub fn new(app_name: &str) -> Self {
        let proj_dirs = ProjectDirs::from("", "", app_name)
            .expect("[ERROR] Unable to determine project directories");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let config_dir = data_dir.join("config");
        let settings_file = data_dir.join("settings.toml");

        fs::create_dir_all(&config_dir).expect("[ERROR] Failed to create config directory");

        Self { 
            data_dir,
            handler: Configurator::new()
        }
    }

    pub fn configs_list(&self) -> Vec<String> {
        let mut result = Vec::new();

        let entries = std::fs::read_dir(&self.get_configs_path())
            .expect("[ERROR] Failed to read config directory");

        for entry in entries {
            let entry = entry.expect("[ERROR] Failed to read directory entry");
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "json" {
                if let Some(stem) = path.file_stem() {
                    result.push(stem.to_str().unwrap().to_string());
                }
            }
        }

        result.sort();
        result
    }

    pub fn add_config(&mut self) -> &mut Self {
        self.handler.save_to_file(self.get_configs_path()).unwrap();
        self
    }

    pub fn handler_mut(&mut self) -> &mut Configurator {
        &mut self.handler
    }

    pub fn get_data_path(&self) -> PathBuf {
        self.data_dir.clone()
    }

    pub fn get_configs_path(&self) -> PathBuf {
        self.data_dir.clone().join("config")
    }

    pub fn get_settings_path(&self) -> PathBuf {
        self.data_dir.clone().join("settings.toml")
    }
}
