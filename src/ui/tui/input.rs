use std::sync::{Arc, Mutex};

use anyhow::{Result};
use crossterm::event::KeyCode;

#[cfg(not(feature = "daemon"))]
use crate::datamanager::app::App;

#[cfg(feature = "daemon")]
use crate::datamanager::async_app::App;
use crate::ui::tui::state::InputMode;

use super::consts::{keys, route, timing, ui};
use super::state::{InputAction, TuiState};



pub fn handle_add_config_input(
    app: &mut App,
    state: &mut TuiState,
    key: KeyCode,
    tun_mode: bool,
) -> Result<()> {
    match key {
        KeyCode::Esc => {
            state.input.mode = InputMode::Normal;
            state.input.error = false;
            state.input.buffer.clear();
        }
        KeyCode::Enter => {
            if !state.input.buffer.is_empty() {
                let cfg = app.handler_mut().clean();
                let result = if tun_mode {
                    cfg.default_tun()
                        .set_outbound_from_url(&state.input.buffer.clone())
                } else {
                    cfg.default()
                        .set_outbound_from_url(&state.input.buffer.clone())
                };

                match result {
                    Ok(_) => {
                        if app.add_config(None).is_err() {
                            state.input.error = true;
                        } else {
                            state.input.error = false;
                            state.input.buffer.clear();
                            state.app.len = app.get_len();
                            state.input.mode = InputMode::Normal;
                            state.app.selected_index = 0;
                        }
                    }
                    Err(_) => {
                        state.input.error = true;
                    }
                }
            }
        }
        KeyCode::Backspace => {
            state.input.buffer.pop();
            if state.input.buffer.is_empty() {
                state.input.error = false;
            }
        }
        KeyCode::Char(c) => {
            state.input.buffer.push(c);
        }
        _ => {}
    }

    Ok(())
}

pub fn handle_route_value_input(state: &mut TuiState, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            state.input.mode = InputMode::Normal;
            state.input.buffer.clear();
        }
        KeyCode::Enter => {
            if !state.input.buffer.is_empty() {
                state.settings.route_value = Some(state.input.buffer.clone());
            }
            state.input.mode = InputMode::Normal;
            state.input.buffer.clear();
        }
        KeyCode::Backspace => {
            state.input.buffer.pop();
        }
        KeyCode::Char(c) => {
            state.input.buffer.push(c);
        }
        _ => {}
    }
}

pub fn handle_normal_input(
    app: &mut App,
    state: &mut TuiState,
    key: KeyCode,
    change_flag: &Arc<Mutex<bool>>,
) -> Result<InputAction> {
    match key {
        KeyCode::Esc => {
            if state.ui.context_menu {
                state.ui.context_menu = false; 
                state.input.buffer.clear();
                return Ok(InputAction::Continue);


            }
            else if state.app.running.is_some() {
                app.send_quit()?;
            }
            return Ok(InputAction::Quit);
        }

        KeyCode::Char(keys::QUIT) => {
            if state.ui.context_menu {
                if state.ui.custom {
                    state.ui.custom = false;
                    state.input.buffer.clear();
                } else {
                    state.ui.context_menu = false;
                    state.ui.popup_selected = 0;
                }
            } else {
                return Ok((InputAction::Quit));
            }
        }

        KeyCode::Char(keys::ADD_CONFIG) => {
            state.input.mode = InputMode::AddConfig { tun: (false) };
            state.input.buffer.clear();
            state.input.error = false;
        }
        
        KeyCode::Char(keys::ADD_TUN_CONFIG) => {
            state.input.mode = InputMode::AddConfig { tun: (true) };
            state.input.buffer.clear();
            state.input.error = false;
        }
        
        KeyCode::Char(keys::DELETE_CONFIG) => {
            if state.app.len > 0 {
                let name = app.get_list()[state.app.selected_index].clone();
                app.remove_config_by_number(state.app.selected_index)?;

                if state.app.running.as_deref() == Some(name.as_str()) {
                    app.stop_app()?;
                    state.app.running = None;
                    state.app.enter_mode = false;
                }

                state.app.len = app.get_len();

                if state.app.selected_index >= state.app.len && state.app.len > 0 {
                    state.app.selected_index = state.app.len - 1;
                }
            }
        }
        
        KeyCode::Tab => {
            state.ui.context_menu = false;
            state.ui.settings_panel = !state.ui.settings_panel;
        }

        KeyCode::Char(c) => {
            if state.ui.context_menu
                && state.ui.settings_selected == ui::ROUTE_ACTION_INDEX
                && state.ui.custom
                && state.ui.popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX
            {
                state.input.buffer.push(c);
            }
        }

        KeyCode::Backspace => {
            if state.ui.context_menu
                && state.ui.settings_selected == ui::ROUTE_ACTION_INDEX
                && state.ui.custom
                && state.ui.popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX
            {
                state.input.buffer.pop();
            }
        }

        KeyCode::Enter => {
            if state.ui.settings_selected == 6 {
                app.set_handler_config_by_number(state.app.selected_index)?;
                let mut route_rules: Vec<String> = Vec::new();
                if let Some(action) = state.settings.route_action.as_ref() {
                    route_rules.push(action.to_string());
                }
                if let Some(r_type) = state.settings.route_type.as_ref() {
                    route_rules.push(r_type.to_string());
                }
                if let Some(value) = state.settings.route_value.as_ref() {
                    route_rules.push(value.to_string());
                }
                let route_rules = vec![route_rules.join(":")];
                app.handler_mut().add_route_rules(&route_rules)?;
                app.save()?;
            }

            if state.ui.context_menu {
                match state.ui.settings_selected {
                    ui::ROUTE_ACTION_INDEX => {
                        if state.ui.popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX {
                            if !state.ui.custom {
                                state.ui.custom = true;
                                state.input.buffer.clear();
                            } else if !state.input.buffer.is_empty() {
                                state.settings.route_action = Some(state.input.buffer.clone());
                                state.ui.custom = false;
                                state.input.buffer.clear();
                            }
                        } else if let Some(value) = route::ACTIONS.get(state.ui.popup_selected) {
                            state.settings.route_action = Some((*value).to_string());
                            state.ui.context_menu = false;
                            state.ui.popup_selected = 0;
                        }
                    }

                    ui::ROUTE_TYPE_INDEX => {
                        if let Some((_, value)) = route::TYPES.get(state.ui.popup_selected) {
                            state.settings.route_type = Some((*value).to_string());
                        }
                    }

                    _ => {}
                }
                if !state.ui.custom {
                    state.ui.context_menu = false;
                    state.ui.popup_selected = 0;
                }
            } else if state.ui.transit && state.ui.settings_panel {
                if state.ui.settings_selected == ui::ROUTE_VALUE_INDEX {
                    state.input.mode = InputMode::RouteValue;
                    state.input.buffer.clear();
                } else if state.ui.settings_selected != ui::DNS_TYPE_INDEX {
                    state.ui.context_menu = true;
                    state.ui.popup_selected = 0;
                }
            } else {
                let len = app.get_len();
                if let Ok(mut flag) = change_flag.lock() {
                    *flag = true;
                }
                if len > 0 && !state.app.enter_mode {
                    let number = state.app.selected_index as u16 + 1;
                    state.app.running = Some(app.get_list()[state.app.selected_index].clone());
                    app.set_log_file();
                    app.run_app(None, Some(number as usize - 1), false)?;
                    state.ui.settings_panel = false;
                    state.ui.transit = false;
                    state.app.enter_mode = true;
                } else if state.app.enter_mode {
                    let name = app.get_list()[state.app.selected_index].clone();
                    if state.app.running.as_deref() == Some(name.as_str()) {
                        state.app.running = None;
                        app.stop_app()?;
                        state.app.enter_mode = false;
                    } else {
                        app.stop_app()?;
                        std::thread::sleep(timing::RESTART_DELAY);
                        let number = state.app.selected_index as u16 + 1;
                        state.app.running = Some(name.clone());
                        app.set_log_file();
                        app.run_app(None, Some(number as usize - 1), false)?;
                        state.ui.settings_panel = false;
                        state.ui.transit = false;
                    }
                }
            }
        }

        KeyCode::Right => {
            if !state.ui.transit {
                state.ui.transit = true;
            } else if state.ui.transit && state.ui.settings_panel {
                state.ui.settings_selected = (state.ui.settings_selected + 1) % ui::SETTINGS_FIELDS_COUNT;
            }
        }
        
        KeyCode::Left => {
            if state.ui.settings_selected == 0 && state.ui.transit {
                state.ui.transit = false;
            } else if state.ui.transit && state.ui.settings_panel {
                state.ui.settings_selected = (state.ui.settings_selected + ui::ROUTE_FIELDS_COUNT - 1) % ui::ROUTE_FIELDS_COUNT;
            }
        }

        KeyCode::Down | KeyCode::Char(keys::DOWN_ALT) => {
            if !state.ui.transit && state.app.len > 0 {
                state.app.selected_index = (state.app.selected_index + 1) % state.app.len;
            } else if state.ui.context_menu {
                let context_len = if state.ui.settings_selected == ui::ROUTE_ACTION_INDEX {
                    route::ACTIONS.len() + 1
                } else if state.ui.settings_selected == ui::ROUTE_TYPE_INDEX {
                    route::TYPES.len()
                } else {
                    1
                };
                state.ui.popup_selected = (state.ui.popup_selected + 1) % context_len;
            } else if state.ui.transit && state.ui.settings_panel {
                state.ui.settings_selected = match state.ui.settings_selected {
                    ui::ROUTE_ACTION_INDEX => ui::DNS_TYPE_INDEX,
                    ui::ROUTE_TYPE_INDEX => ui::DNS_TYPE_INDEX,
                    ui::ROUTE_VALUE_INDEX => ui::DNS_VALUE1_INDEX,
                    ui::DNS_TYPE_INDEX => ui::DNS_VALUE2_INDEX,
                    ui::DNS_VALUE1_INDEX => ui::DNS_VALUE2_INDEX,
                    _ => state.ui.settings_selected,
                };
            }
        }

        KeyCode::Up | KeyCode::Char(keys::UP_ALT) => {
            if state.app.len > 0 && !state.ui.transit {
                state.app.selected_index = (state.app.selected_index + state.app.len - 1) % state.app.len;
            } else if state.ui.context_menu {
                let context_len = if state.ui.settings_selected == ui::ROUTE_ACTION_INDEX {
                    route::ACTIONS.len() + 1
                } else if state.ui.settings_selected == ui::ROUTE_TYPE_INDEX {
                    route::TYPES.len()
                } else {
                    1
                };
                state.ui.popup_selected = (state.ui.popup_selected + context_len - 1) % context_len;
            } else if state.ui.transit && state.ui.settings_panel {
                state.ui.settings_selected = match state.ui.settings_selected {
                    ui::DNS_VALUE2_INDEX => ui::DNS_VALUE1_INDEX,
                    ui::DNS_TYPE_INDEX => ui::ROUTE_ACTION_INDEX,
                    ui::DNS_VALUE1_INDEX => ui::ROUTE_TYPE_INDEX,
                    _ => state.ui.settings_selected,
                };
            }
        }

        _ => {}
    }
    
    Ok(InputAction::Continue)
}
