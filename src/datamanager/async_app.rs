use crate::{Request, socket_name};
use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use interprocess::local_socket::tokio::{Stream, prelude::*};
use libc::{SYS_sched_get_priority_max, hostent};
use std::fs;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::runtime::Runtime;

use crate::datamanager::*;

pub struct App {
    data_dir: PathBuf,
    configs: Vec<String>,
    inf_handler: Infor,
    cfg_handler: Configurator,
    stg_handler: Settings,
    log_handler: Logger,
    runtime: Runtime,
    socket_stream: Stream,
}

impl App {
    pub fn new(app_name: &str) -> Result<Self> {
        let proj_dirs =
            ProjectDirs::from("", "", app_name).expect("Unable to determine project directories");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let config_dir = data_dir.join("config");
        let settings = Settings::new(data_dir.join("settings.toml"))?;

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let socket_stream = runtime.block_on(Self::connect_socket())?;

        let mut app = Self {
            data_dir,
            configs: vec![],
            inf_handler: Infor::new(),
            cfg_handler: Configurator::new(),
            stg_handler: settings,
            log_handler: Logger::new(),
            runtime,
            socket_stream,
        };

        app.configs = app.read_configs()?;
        Ok(app)
    }

    async fn connect_socket() -> Result<Stream> {
        Ok(Stream::connect(socket_name()?).await?)
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
        self.inf_handler
            .set_name(&saved_name)
            .set_inbounds(self.cfg_handler.get_inbounds_ports());
        self.configs.push(saved_name);
        self.configs.sort();
        Ok(self)
    }

    pub fn run_app(
        &mut self,
        tag: Option<&str>,
        number: Option<usize>,
        unable_system_proxy: bool,
    ) -> Result<()> {
        let _ = self.stg_handler.read(self.get_settings_path());

        println!("Worked");

        let file_path;
        if let Some(n) = tag {
            file_path = self.get_configs_path().join(format!("{}.json", n));
            self.stg_handler.current = Some(n.to_string().clone());
            self.set_handler_config_by_name(n)?;
            self.inf_handler
                .set_name(n)
                .set_inbounds(self.cfg_handler.get_inbounds_ports());
        } else if let Some(n) = number {
            file_path = self.get_configs_path().join(format!(
                "{}.json",
                self.get_list()
                    .get(n)
                    .context("No exists config with that number")?
            ));
            self.stg_handler.current = Some(
                self.get_list()
                    .get(n)
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

        println!("Worked 2");
        self.runtime.block_on(async {
            send_start(
                &mut self.socket_stream,
                file_path.to_str().unwrap().to_string(),
            )
            .await
        });
        println!("Worked 3");

        if unable_system_proxy {
            self.stg_handler.unable_system_proxy = Some(unable_system_proxy);
        }

        if !unable_system_proxy {
            if self.handler_ref().get_list_of_system_proxies().len() > 1 {
                return Err(anyhow!("More than 1 system proxy"));
            } else if let Some((host, port, support_socks)) =
                self.handler_ref().get_list_of_system_proxies().first()
            {
                self.runtime.block_on(async {
                    send_enable_sys_proxy(
                        &mut self.socket_stream,
                        host.to_string(),
                        *port,
                        *support_socks,
                    )
                    .await
                });
            }
        }
        let _ = self.stg_handler.save(self.get_settings_path());
        Ok(())
    }

    pub fn stop_app(&mut self) -> Result<()> {
        if !self.stg_handler.unable_system_proxy.unwrap_or(true) {
            self.runtime
                .block_on(async { send_disable_sys_proxy(&mut self.socket_stream).await });
        }
        self.runtime
            .block_on(async { send_stop(&mut self.socket_stream).await });
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
        self.inf_handler.set_name(&new_name);
        Ok(())
    }

    pub fn set_handler_config_by_name(&mut self, name: &str) -> Result<()> {
        self.cfg_handler
            .load_from_file(self.get_configs_path().join(format!("{}.json", name)))?;

        self.inf_handler
            .set_name(name)
            .set_inbounds(self.cfg_handler.get_inbounds_ports());
        Ok(())
    }

    pub fn set_handler_config_by_number(&mut self, number: usize) -> Result<()> {
        self.cfg_handler
            .load_from_file(self.get_configs_path().join(format!(
                "{}.json",
                self.configs.get(number).context("Config doesnt exist")?
            )))?;

        self.inf_handler
            .set_name(&self.configs.get(number).unwrap())
            .set_inbounds(self.cfg_handler.get_inbounds_ports());
        Ok(())
    }

    pub fn set_handler_config_by_current(&mut self) -> Result<()> {
        if let Some(name) = self.stg_handler.current.clone() {
            self.cfg_handler
                .load_from_file(self.get_configs_path().join(format!("{}.json", &name)))?;
            self.inf_handler
                .set_name(&name)
                .set_inbounds(self.cfg_handler.get_inbounds_ports());
        } else {
            if let Some(name) = self.get_list().first() {
                self.cfg_handler
                    .load_from_file(self.get_configs_path().join(format!("{}.json", &name)))?;
                self.inf_handler
                    .set_name(&name)
                    .set_inbounds(self.cfg_handler.get_inbounds_ports());
                self.stg_handler.current = Some(name.clone());
            } else {
                return Err(anyhow!("No configs exist"));
            }
        }

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

    pub fn get_inf_ref(&self) -> &Infor {
        &self.inf_handler
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

#[inline]
async fn send_disable_sys_proxy(
    socket_stream: &mut (impl tokio::io::AsyncWriteExt + tokio::io::AsyncReadExt + Unpin),
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let request = Request::disable_sys_proxy();
    let payload = serde_json::to_vec(&request)?;

    socket_stream.write_all(&payload).await?;
    socket_stream.write_all(b"\n").await?;
    socket_stream.flush().await?;

    let mut response = String::new();
    let mut reader = BufReader::new(socket_stream);
    reader.read_line(&mut response).await?;

    Ok(response)
}

#[inline]
async fn send_stop(
    socket_stream: &mut (impl tokio::io::AsyncWriteExt + tokio::io::AsyncReadExt + Unpin),
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let request = Request::stop();
    let payload = serde_json::to_vec(&request)?;

    socket_stream.write_all(&payload).await?;
    socket_stream.write_all(b"\n").await?;
    socket_stream.flush().await?;

    let mut response = String::new();
    let mut reader = BufReader::new(socket_stream);
    reader.read_line(&mut response).await?;

    Ok(response)
}

#[inline]
async fn send_start(
    socket_stream: &mut (impl tokio::io::AsyncWriteExt + tokio::io::AsyncReadExt + Unpin),
    config_path: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let request = Request::start(config_path);
    let payload = serde_json::to_vec(&request)?;

    socket_stream.write_all(&payload).await?;
    socket_stream.write_all(b"\n").await?;
    socket_stream.flush().await?;

    let mut response = String::new();
    let mut reader = BufReader::new(socket_stream);
    reader.read_line(&mut response).await?;

    Ok(response)
}

#[inline]
async fn send_enable_sys_proxy(
    socket_stream: &mut (impl tokio::io::AsyncWriteExt + tokio::io::AsyncReadExt + Unpin),
    host: String,
    port: u16,
    support_socks: bool,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let request = Request::enable_sys_proxy(host, port, support_socks);
    let payload = serde_json::to_vec(&request)?;

    socket_stream.write_all(&payload).await?;
    socket_stream.write_all(b"\n").await?;
    socket_stream.flush().await?;

    let mut response = String::new();
    let mut reader = BufReader::new(socket_stream);
    reader.read_line(&mut response).await?;

    Ok(response)
}
