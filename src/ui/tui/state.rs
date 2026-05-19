use std::path::PathBuf;

use anyhow::Result;

#[cfg(not(feature = "daemon"))]
use crate::datamanager::app::App;

#[cfg(feature = "daemon")]
use crate::datamanager::async_app::App;

pub struct AppState {
    pub selected_index: usize,
    pub len: usize,
    pub running: Option<String>,
    pub enter_mode: bool,
}

pub struct InputState {
    pub mode: InputMode,
    pub error: bool,
    pub buffer: String,
}

pub struct SettingsState {
    pub route_action: Option<String>,
    pub route_type: Option<String>,
    pub route_value: Option<String>,
    pub dns_type: Option<String>,
    pub dns_address: Option<String>,
    pub dns_port: Option<String>,
}

pub struct UiState {
    pub right_panel: RightPanel,
    pub focus: Focus,
    pub context_menu: bool,
    pub popup_selected: usize,
    pub custom: bool,
    pub settings_selected: usize,
}

pub struct TuiState {
    pub app: AppState,
    pub ui: UiState,
    pub settings: SettingsState,
    pub input: InputState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    AddConfig { tun: bool },
    ValueInput,
}

pub enum InputAction {
    Continue,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightPanel {
    Logs,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Configs,
    RightPanel,
}

impl TuiState {
    pub fn new(app: &mut App) -> Result<Self> {
        #[cfg(feature = "daemon")]
        let running = app.get_status()?.map(|s| {
            PathBuf::from(s.file)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string()
        });

        #[cfg(not(feature = "daemon"))]
        let running: Option<String> = None;

        let configs = app.get_list();
        let current = app.get_current_config();

        let selected_index = current
            .as_ref()
            .and_then(|name| configs.iter().position(|cfg| cfg == name))
            .unwrap_or(0);
        if !configs.is_empty() {
            app.set_handler_config_by_number(selected_index)?;
        }

        let right_panel = if running.as_ref().is_some_and(|x| !x.is_empty()) {
            RightPanel::Logs
        } else {
            RightPanel::Settings
        };

        Ok(Self {
            app: AppState {
                selected_index,
                len: app.get_len(),
                running,
                enter_mode: false,
            },
            ui: UiState {
                right_panel,
                focus: Focus::Configs,
                context_menu: false,
                popup_selected: 0,
                settings_selected: 0,
                custom: false,
            },
            input: InputState {
                mode: InputMode::Normal,
                buffer: String::new(),
                error: false,
            },
            settings: SettingsState {
                route_action: None,
                route_type: None,
                route_value: None,
                dns_type: None,
                dns_address: None,
                dns_port: None,
            },
        })
    }

    // pub fn moder(&self) -> InputMode {
    //     if self.input.input_mode {
    //         if self.input.tun_mode {
    //             InputMode::AddTunConfig
    //         } else {
    //             InputMode::AddConfig
    //         }
    //     } else if self.ui.value_input {
    //         InputMode::RouteValue
    //     } else {
    //         InputMode::Normal
    //     }
    // }
}
