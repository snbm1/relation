use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use toml;

#[cfg(not(feature = "daemon"))]
pub mod app;

#[cfg(feature = "daemon")]
pub mod async_app;

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
}

pub struct Infor {
    pub config_name: String,
    pub inbound_mod: Vec<InboundMod>,
}

impl Infor {
    pub fn new() -> Self {
        Self {
            config_name: "".to_string(),
            inbound_mod: vec![],
        }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.config_name = name.to_string();
        self
    }

    pub fn set_inbounds(&mut self, inbound: Vec<InboundMod>) -> &mut Self {
        self.inbound_mod = inbound;
        self
    }

    pub fn get_name(&self) -> String {
        self.config_name.clone()
    }

    pub fn get_inbound(&self) -> Option<Vec<InboundMod>> {
        if self.inbound_mod.is_empty() {
            None
        } else {
            Some(self.inbound_mod.clone())
        }
    }

    pub fn get_inbound_ports(&self) -> Option<Vec<u16>> {
        let mut rs = vec![];

        if rs.is_empty() { None } else { Some(rs) }
    }
}
