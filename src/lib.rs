pub mod app;
pub mod bridge;
pub mod configurator;
pub mod ui;

use anyhow::Result;
#[cfg(not(windows))]
use interprocess::local_socket::{GenericFilePath, ToFsName};
#[cfg(windows)]
use interprocess::local_socket::{GenericNamespaced, ToNsName};
use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "relation.sock";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub command: Command,
}

impl Request {
    pub fn status() -> Self {
        Request {
            command: Command::Status,
        }
    }

    pub fn start(config_path: String) -> Self {
        Request {
            command: Command::Start(config_path),
        }
    }

    pub fn stop() -> Self {
        Request {
            command: Command::Stop,
        }
    }

    pub fn enable_sys_proxy(host: String, port: u16, support_socks: bool) -> Self {
        Request {
            command: Command::EnableSysProxy((host, port, support_socks)),
        }
    }

    pub fn disable_sys_proxy() -> Self {
        Request {
            command: Command::DisableSysProxy,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Status,
    Start(String),
    Stop,
    EnableSysProxy((String, u16, bool)),
    DisableSysProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Running,
    Stopped,
    Error(String),
    Ok,
}

#[cfg(unix)]
pub fn socket_path() -> &'static str {
    "/tmp/relation.sock"
}

pub fn socket_name() -> Result<interprocess::local_socket::Name<'static>> {
    #[cfg(windows)]
    let name = SOCKET_NAME.to_ns_name::<GenericNamespaced>()?;

    #[cfg(not(windows))]
    let name = socket_path().to_fs_name::<GenericFilePath>()?;

    Ok(name)
}
