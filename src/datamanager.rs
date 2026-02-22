use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;
use toml;

use crate::bridge;
use crate::configurator::Configurator;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    current: Option<String>,
    unable_system_proxy: Option<bool>,
}

impl Settings {
    pub fn new(setting_file: PathBuf) -> Result<Self, Box<dyn Error>> {
        // Try to read the file, if it doesn't exist, create default settings and save
        match fs::read_to_string(&setting_file) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let settings = Self {
                    current: None,
                    unable_system_proxy: None,
                };

                settings.save(setting_file)?;
                Ok(settings)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn save(&self, setting_file: PathBuf) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = setting_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp_file = setting_file.with_extension("tmp");
        let toml_string = toml::to_string_pretty(self)?;

        fs::write(&tmp_file, toml_string)?;
        fs::rename(tmp_file, setting_file)?;

        Ok(())
    }

    pub fn read(&mut self, setting_file: PathBuf) -> Result<(), Box<dyn Error>> {
        match fs::read_to_string(&setting_file) {
            Ok(content) => {
                *self = toml::from_str(&content)?;
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub struct App {
    data_dir: PathBuf,
    configs: Vec<String>,
    cfg_handler: Configurator,
    stg_handler: Settings,
}

impl App {
    pub fn new(app_name: &str) -> Self {
        let proj_dirs = ProjectDirs::from("", "", app_name)
            .expect("[ERROR] Unable to determine project directories");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let config_dir = data_dir.join("config");
        let settings_file = data_dir.join("settings.toml");

        fs::create_dir_all(&config_dir).expect("[ERROR] Failed to create config directory");

        let mut mng = Self {
            data_dir,
            configs: vec![],
            cfg_handler: Configurator::new(),
            stg_handler: Settings::new(settings_file).unwrap(),
        };

        mng.configs = mng.read_configs();
        mng
    }

    pub fn read_configs(&mut self) -> Vec<String> {
        let mut result = Vec::new();

        let entries = std::fs::read_dir(self.get_configs_path())
            .expect("[ERROR] Failed to read config directory");

        for entry in entries {
            let entry = entry.expect("[ERROR] Failed to read directory entry");
            let path = entry.path();

            if path.is_file() && path.extension().unwrap_or_default() == "json" {
                if let Some(stem) = path.file_stem() {
                    result.push(stem.to_str().unwrap().to_string());
                }
            }
        }

        result.sort();
        result
    }

    pub fn remove_config(&mut self, name: &str) {
        let file_path = self.get_configs_path().join(format!("{name}.json"));

        if !file_path.exists() {
            panic!(
                "[ERROR] Config file '{}' does not exist",
                file_path.display()
            );
        }

        fs::remove_file(&file_path).expect("[ERROR] Failed to remove config file");

        self.configs.retain(|x| x != name);
    }

    pub fn remove_config_by_number(&mut self, number: usize) {
        let file_path = self
            .get_configs_path()
            .join(format!("{}.json", self.configs[number]));

        if !file_path.exists() {
            panic!(
                "[ERROR] Config file '{}' does not exist",
                file_path.display()
            );
        }

        fs::remove_file(&file_path).expect("[ERROR] Failed to remove config file");

        self.configs.remove(number);
    }

    pub fn add_config(&mut self) -> &mut Self {
        self.configs.push(
            self.cfg_handler
                .save_to_file(self.get_configs_path())
                .unwrap(),
        );
        self.configs.sort();
        self
    }

    pub fn run_app(&mut self, tag: Option<String>, number: Option<u16>, unable_system_proxy: bool) {
        self.stg_handler.read(self.get_settings_path());

        let file_path;
        if let Some(n) = tag {
            file_path = self.get_configs_path().join(format!("{}.json", n));
            self.stg_handler.current = Some(n);
        } else if let Some(n) = number {
            file_path = self
                .get_configs_path()
                .join(format!("{}.json", self.get_list()[n as usize - 1]));
            self.stg_handler.current = Some(self.get_list()[n as usize - 1].clone());
        } else if let Some(n) = self.stg_handler.current.clone() {
            file_path = self.get_configs_path().join(format!("{}.json", n));
        } else {
            file_path = self
                .get_configs_path()
                .join(format!("{}.json", self.get_list()[0]));
            self.stg_handler.current = Some(self.get_list()[0].clone());
        }

        bridge::start_safe(file_path.to_str().unwrap(), 0);
        if unable_system_proxy {
            self.stg_handler.unable_system_proxy = Some(unable_system_proxy);
        }

        if !unable_system_proxy {
            if self.handler_ref().get_list_of_system_proxies().len() > 1 {
                panic!("[ERROR] A more than 1 system proxy");
            } else if let Some((host, port, support_socks)) =
                self.handler_ref().get_list_of_system_proxies().first()
            {
                bridge::enable_system_proxy_safe(host, *port as i64, *support_socks);
            }
        }
        let _ = self.stg_handler.save(self.get_settings_path());
        println!("[INFO] Run Relation");
    }

    pub fn stop_app(&mut self) {
        if !self.stg_handler.unable_system_proxy.unwrap_or(true) {
            bridge::disable_system_proxy_safe();
        }
        bridge::stop_safe();
        let _ = self.stg_handler.save(self.get_settings_path());
        println!("[INFO] Shutdown Relation");
    }

    pub fn handler_ref(&self) -> &Configurator {
        &self.cfg_handler
    }

    pub fn handler_mut(&mut self) -> &mut Configurator {
        &mut self.cfg_handler
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

    pub fn get_list(&self) -> Vec<String> {
        self.configs.clone()
    }
}
