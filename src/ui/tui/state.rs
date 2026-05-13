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
   pub dns_value1: Option<String>,
   pub dns_value2: Option<String>,
}

pub struct UiState {
    pub settings_panel: bool, 
    pub transit: bool, 
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
    RouteValue,
}

pub enum InputAction {
    Continue, 
    Quit, 
}

impl TuiState {
   pub fn new(app: &mut App) -> Result<Self> {
        let running = app.get_status()?.map(|s| {
            PathBuf::from(s.file)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string()
        });

        let settings_panel = !running.as_ref().is_some_and(|x| !x.is_empty());

        Ok(Self {
            app: AppState {
                selected_index: 0,
                len: app.get_len(),
                running,
                enter_mode: false,
            },
            ui: UiState {
                settings_panel,
                transit: false,
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
                dns_value1: None,
                dns_value2: None,
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
