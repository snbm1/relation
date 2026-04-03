pub mod app;
mod bridge;
mod configurator;
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
            command: Command::Start { config_path },
        }
    }

    pub fn stop() -> Self {
        Request {
            command: Command::Stop,
        }
    }

    pub fn enable_sys_proxy(port: u16) -> Self {
        Request {
            command: Command::EnableSysProxy { port },
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
    Start { config_path: String },
    Stop,
    EnableSysProxy { port: u16 },
    DisableSysProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusResponse {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub reply: Result<(), String>,
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
