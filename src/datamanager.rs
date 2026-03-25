use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::ascii::escape_default;
use std::collections::VecDeque;
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
    pub fn new(setting_file: PathBuf) -> Result<Self> {
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
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn save(&self, setting_file: PathBuf) -> Result<()> {
        if let Some(parent) = setting_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp_file = setting_file.with_extension("tmp");
        let toml_string = toml::to_string_pretty(self)?;

        fs::write(&tmp_file, toml_string)?;
        fs::rename(tmp_file, setting_file)?;

        Ok(())
    }

    pub fn read(&mut self, setting_file: PathBuf) -> Result<()> {
        match fs::read_to_string(&setting_file) {
            Ok(content) => {
                *self = toml::from_str(&content)?;
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

pub struct Logger {
    logs: VecDeque<String>,
    new_logs: Vec<String>,
    last_pos: usize,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::with_capacity(128),
            new_logs: Vec::with_capacity(64),
            last_pos: 0,
        }
    }

    pub fn read(&mut self, path: PathBuf) {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };

        if self.last_pos > content.len() {
            self.last_pos = 0;
        }

        let new_content = &content[self.last_pos..];
        self.last_pos = content.len();

        self.new_logs.clear();

        for line in new_content.lines() {
            let line = line.to_string();
            self.push_log(line.clone());
            self.new_logs.push(line);
        }
    }

    fn push_log(&mut self, line: String) -> &mut Self {
        if self.logs.len() == 128 {
            self.logs.pop_front();
        }
        self.logs.push_back(line);
        self
    }

    pub fn get_new_logs(&mut self) -> Vec<String> {
        std::mem::take(&mut self.new_logs)
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.iter().cloned().collect()
    }

    pub fn clean(&mut self) -> Vec<String> {
        self.new_logs.clear();
        self.last_pos = 0;
        std::mem::take(&mut self.logs).into_iter().collect()
    }
}

#[derive(Copy, Debug, Clone)]
pub enum InboundMod {
    Http(u16),
    Socks5(u16),
    Mixed(u16),
    Tun,
    None,
}

pub struct Infor {
    pub config_name: String,
    pub inbound_mod: InboundMod,
}

impl Infor {
    pub fn new() -> Self {
        Self {
            config_name: "".to_string(),
            inbound_mod: InboundMod::None,
        }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.config_name = name.to_string();
        self
    }

    pub fn set_inbound(&mut self, inbound: InboundMod) -> &mut Self {
        self.inbound_mod = inbound;
        self
    }

    pub fn get_name(&self) -> String {
        self.config_name.clone()
    }

    pub fn get_inbound(&self) -> InboundMod {
        self.inbound_mod
    }
}

pub struct App {
    data_dir: PathBuf,
    configs: Vec<String>,
    inf_handler: Infor,
    cfg_handler: Configurator,
    stg_handler: Settings,
    log_handler: Logger,
}

impl App {
    pub fn new(app_name: &str) -> Result<Self> {
        let proj_dirs =
            ProjectDirs::from("", "", app_name).expect("Unable to determine project directories");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let config_dir = data_dir.join("config");
        let settings = Settings::new(data_dir.join("settings.toml"))?;

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        let mut mng = Self {
            data_dir,
            configs: vec![],
            inf_handler: Infor::new(),
            cfg_handler: Configurator::new(),
            stg_handler: settings,
            log_handler: Logger::new(),
        };

        mng.configs = mng.read_configs()?;
        Ok(mng)
    }

    pub fn read_configs(&mut self) -> Result<Vec<String>> {
        let mut result = Vec::new();

        let entries = std::fs::read_dir(self.get_configs_path())
            .context("Failed to read config directory")?;

        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_file() && path.extension().unwrap_or_default() == "json" {
                if let Some(stem) = path.file_stem() {
                    result.push(stem.to_str().unwrap().to_string());
                }
            }
        }

        result.sort();
        Ok(result)
    }

    pub fn remove_config(&mut self, name: &str) -> Result<()> {
        let file_path = self.get_configs_path().join(format!("{name}.json"));

        if !file_path.exists() {
            return Err(anyhow!(
                "Config file '{}' does not exist",
                file_path.display()
            ));
        }

        fs::remove_file(&file_path).context("Failed to remove config file")?;

        self.configs.retain(|x| x != name);

        Ok(())
    }

    pub fn remove_config_by_number(&mut self, number: usize) -> Result<()> {
        let file_path = self.get_configs_path().join(format!(
            "{}.json",
            self.configs.get(number).context("Config doesnt exist")?
        ));

        if !file_path.exists() {
            return Err(anyhow!(
                "Config file '{}' does not exist",
                file_path.display()
            ));
        }

        fs::remove_file(&file_path).context("Failed to remove config file")?;

        self.configs.remove(number);
        Ok(())
    }

    pub fn add_config(&mut self, name: Option<String>) -> Result<&mut Self> {
        self.set_log_file();
        let saved_name = self.save_config(name)?;
        self.inf_handler.set_name(&saved_name).set_inbound(
            *self
                .cfg_handler
                .get_inbounds_ports()
                .first()
                .context("No inbound exists")?,
        );
        self.configs.push(saved_name);
        self.configs.sort();
        Ok(self)
    }

    pub fn run_app(
        &mut self,
        tag: Option<&str>,
        number: Option<u16>,
        unable_system_proxy: bool,
    ) -> Result<()> {
        let _ = self.stg_handler.read(self.get_settings_path());

        let file_path;
        if let Some(n) = tag {
            file_path = self.get_configs_path().join(format!("{}.json", n));
            self.stg_handler.current = Some(n.to_string().clone());
            self.set_handler_config_by_name(n);
            self.inf_handler.set_name(n).set_inbound(
                *self
                    .cfg_handler
                    .get_inbounds_ports()
                    .first()
                    .context("No inbound exists")?,
            );
        } else if let Some(n) = number {
            file_path = self.get_configs_path().join(format!(
                "{}.json",
                self.get_list()
                    .get(n as usize - 1)
                    .context("No exists config with that number")?
            ));
            self.stg_handler.current = Some(
                self.get_list()
                    .get(n as usize - 1)
                    .context("No exists config with that number")?
                    .clone(),
            );
            self.set_handler_config_by_number(n)?;
        } else if let Some(n) = self.stg_handler.current.clone() {
            file_path = self.get_configs_path().join(format!("{}.json", n));
            self.set_handler_config_by_name(&n)?;
        } else {
            file_path = self.get_configs_path().join(format!(
                "{}.json",
                self.get_list().first().context("Configs doesnt exist")?
            ));
            self.stg_handler.current = Some(self.get_list().first().unwrap().clone());
            self.set_handler_config_by_name(self.get_list().first().unwrap())?;
        }

        bridge::start_safe(file_path.to_str().unwrap(), 0);
        if unable_system_proxy {
            self.stg_handler.unable_system_proxy = Some(unable_system_proxy);
        }

        if !unable_system_proxy {
            if self.handler_ref().get_list_of_system_proxies().len() > 1 {
                return Err(anyhow!("More than 1 system proxy"));
            } else if let Some((host, port, support_socks)) =
                self.handler_ref().get_list_of_system_proxies().first()
            {
                bridge::enable_system_proxy_safe(host, *port as i64, *support_socks);
            }
        }
        let _ = self.stg_handler.save(self.get_settings_path());
        Ok(())
    }

    pub fn stop_app(&mut self) -> Result<()> {
        if !self.stg_handler.unable_system_proxy.unwrap_or(true) {
            bridge::disable_system_proxy_safe();
        }
        bridge::stop_safe();
        let _ = self.stg_handler.save(self.get_settings_path());
        self.log_handler.clean();
        self.remove_log_file()?;

        Ok(())
    }

    pub fn rename_config(&mut self, new_name: String) -> Result<()> {
        self.remove_config(
            &self
                .get_selected_config()
                .context("Config doesnt selected")?,
        )?;
        self.add_config(Some(new_name.clone()))?;
        self.inf_handler.set_name(&new_name.to_string());
        Ok(())
    }

    pub fn set_handler_config_by_name(&mut self, name: &str) -> Result<()> {
        self.cfg_handler
            .load_from_file(self.get_configs_path().join(format!("{}.json", name)))?;

        self.inf_handler.set_name(name).set_inbound(
            self.cfg_handler
                .get_inbounds_ports()
                .first()
                .context("")?
                .clone(),
        );
        Ok(())
    }

    pub fn set_handler_config_by_number(&mut self, number: u16) -> Result<()> {
        self.cfg_handler
            .load_from_file(self.get_configs_path().join(format!(
                "{}.json",
                self.configs.get(number as usize).context("Config doesnt exist")?
            )))?;

        self.inf_handler
            .set_name(&self.configs.get(number as usize).unwrap())
            .set_inbound(
                self.cfg_handler
                    .get_inbounds_ports()
                    .first()
                    .context("")?
                    .clone(),
            );
        Ok(())
    }

    pub fn save_config(&mut self, name: Option<String>) -> Result<String> {
        let free_name = match name {
            Some(value) => {
                if self.exist_config(&value) > 0 {
                    format!("[{}] {}", self.exist_config(&value), value)
                } else {
                    value
                }
            }
            None => {
                if self.exist_config(
                    &self
                        .cfg_handler
                        .get_outbound_tag()
                        .context("Not defined outbound tag")?,
                ) > 0
                {
                    format!(
                        "[{}] {}",
                        self.exist_config(&self.cfg_handler.get_outbound_tag().unwrap()),
                        self.cfg_handler.get_outbound_tag().unwrap()
                    )
                } else {
                    self.cfg_handler.get_outbound_tag().unwrap()
                }
            }
        };

        self.cfg_handler
            .save_to_file(self.get_configs_path(), &free_name)?;
        Ok(free_name)
    }

    pub fn set_log_file(&mut self) -> &mut Self {
        self.cfg_handler.set_log(
            "info".to_string(),
            Some(self.get_data_path().join("box.log")),
        );
        self
    }

    pub fn remove_log_file(&mut self) -> Result<&mut Self> {
        fs::remove_file(self.get_data_path().join("box.log"))
            .context("Failed to remove config file")?;
        Ok(self)
    }

    pub fn read_logs(&mut self) -> &mut Self {
        let path = self.get_data_path().join("box.log");

        self.log_handler.read(path);

        self
    }

    pub fn get_new_logs(&mut self) -> Vec<String> {
        self.log_handler.get_new_logs()
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.log_handler.get_logs()
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

    pub fn get_len(&self) -> usize {
        self.configs.len()
    }

    pub fn get_current_config(&self) -> Option<String> {
        self.stg_handler.current.clone()
    }

    pub fn get_selected_config(&self) -> Option<String> {
        if !self.inf_handler.get_name().is_empty() {
            Some(self.inf_handler.get_name().clone())
        } else {
            None
        }
    }

    pub fn exist_config(&self, name: &String) -> u8 {
        let mut counter = 0;
        if self.configs.contains(name) {
            loop {
                counter += 1;
                if self.configs.contains(&format!("[{}] {}", counter, name)) {
                    continue;
                } else {
                    break;
                }
            }
        }
        counter
    }
}
