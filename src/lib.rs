pub mod app;
mod bridge;
mod configurator;
pub mod ui;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "relation.sock";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub command: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Status,
    Start { config_path: String },
    Stop,
    EnableSysProxy,
    DisableSysProxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusResponse {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub reply: Result<(), String>,
}
