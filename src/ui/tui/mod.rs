mod consts;
mod ifaces;
mod minireq;
mod tuiguard;

use consts::*;
use ifaces::*;
use minireq::*;
use std::{
    io,
    os::{fd::{AsRawFd, FromRawFd}, linux::raw::stat},
    time::Instant,
};
use tuiguard::TuiGuard;
use std::fs::{File, OpenOptions};

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(not(feature = "daemon"))]
use crate::datamanager::app::App;

#[cfg(feature = "daemon")]
use crate::datamanager::async_app::App;

use crossterm::event::{self, Event, KeyCode};

use std::collections::VecDeque;

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
        block::{Position, Title},
    },
};

use anyhow::{Ok, Result};

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<File>>,
    pub _guard: TuiGuard,
}

struct AppState {
    selected_index: usize, 
    len: usize, 
    running: Option<String>, 
    enter_mode: bool, 
}

struct InputState {
    input_mode: bool, 
    tun_mode: bool, 
    error_input: bool, 
    buffer: String, 
}

struct SettingsState {
    route_action: Option<String>,
    route_type: Option<String>,
    route_value: Option<String>,
    dns_type: Option<String>,
    dns_value1: Option<String>,
    dns_value2: Option<String>,
}

struct UiState {
    settings_panel: bool, 
    transit: bool, 
    context_menu: bool, 
    popup_selected: usize, 
    custom: bool, 
    value_input: bool, 
    settings_selected: usize,
}

struct TuiState {
    app: AppState, 
    ui: UiState, 
    settings: SettingsState, 
    input: InputState, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Normal, 
    AddConfig, 
    AddTunConfig, 
    RouteValue,
}

enum InputAction {
    Continue, 
    Quit, 
}

impl TuiState {
    fn new(app: &mut App) -> Result<Self> {
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
                value_input: false,
                custom: false,
            },
            input: InputState {
                input_mode: false,
                tun_mode: false,
                buffer: String::new(),
                error_input: false, 
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

    fn moder(&self) -> InputMode {
        if self.input.input_mode {
            if self.input.tun_mode {
                InputMode::AddTunConfig
            } else {
                InputMode::AddConfig
            }
        } else if self.ui.value_input {
            InputMode::RouteValue
        } else {
            InputMode::Normal
        }
    }
} 



fn setup_tty() -> Result<Tui> {
    let _guard = TuiGuard::new()?;

    let ui_fd = unsafe { libc::dup(libc::STDOUT_FILENO) };
    if ui_fd < 0 {
        return Err(io::Error::last_os_error().into());
    }

    let null = OpenOptions::new().write(true).open("/dev/null")?;

    unsafe {
        if libc::dup2(null.as_raw_fd(), libc::STDOUT_FILENO) < 0 {
            return Err(io::Error::last_os_error().into());
        }

        if libc::dup2(null.as_raw_fd(), libc::STDERR_FILENO) < 0 {
            return Err(io::Error::last_os_error().into());
        }
    }

    let ui_out = unsafe { File::from_raw_fd(ui_fd) };
    let backend = CrosstermBackend::new(ui_out);
    let terminal = Terminal::new(backend)?;

    Ok(Tui { terminal, _guard })
}

fn handle_add_config_input(
    app: &mut App,
    state: &mut TuiState,
    key: KeyCode,
    tun_mode: bool,
) -> Result<()> {
    match key {
        KeyCode::Esc => {
            state.input.input_mode = false;
            state.input.tun_mode = false;
            state.input.error_input = false;
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
                            state.input.error_input = true;
                        } else {
                            state.input.error_input = false;
                            state.input.buffer.clear();
                            state.app.len = app.get_len();
                            state.input.input_mode = false;
                            state.input.tun_mode = false;
                            state.app.selected_index = 0;
                        }
                    }
                    Err(_) => {
                        state.input.error_input = true;
                    }
                }
            }
        }
        KeyCode::Backspace => {
            state.input.buffer.pop();
            if state.input.buffer.is_empty() {
                state.input.error_input = false;
            }
        }
        KeyCode::Char(c) => {
            state.input.buffer.push(c);
        }
        _ => {}
    }

    Ok(())
}

fn handle_route_value_input(state: &mut TuiState, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            state.ui.value_input = false;
            state.input.buffer.clear();
        }
        KeyCode::Enter => {
            if !state.input.buffer.is_empty() {
                state.settings.route_value = Some(state.input.buffer.clone());
            }
            state.ui.value_input = false;
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

fn handle_normal_input(
    app: &mut App,
    state: &mut TuiState,
    key: KeyCode,
    change_flag: &Arc<Mutex<bool>>,
) -> Result<(InputAction)> {
    match key {
        KeyCode::Esc => {
            if state.ui.context_menu {
                state.ui.context_menu = false; 
                state.input.buffer.clear();


            }
            else if state.app.running.is_some() {
                app.send_quit()?;
            }
            return Ok((InputAction::Quit));
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
            state.input.input_mode = true;
        }
        
        KeyCode::Char(keys::ADD_TUN_CONFIG) => {
            state.input.input_mode = true;
            state.input.tun_mode = true;
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
            if state.ui.settings_selected == ui::DNS_TYPE_INDEX {
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
                    state.ui.value_input = true;
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
            } else if state.ui.transit && state.ui.settings_panel && !state.ui.value_input {
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
            } else if state.ui.transit && state.ui.settings_panel && !state.ui.value_input {
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
    
    Ok((InputAction::Continue))
}

pub fn run(app: &mut App) -> Result<()> {
    let iface = iface_detect();

    std::panic::set_hook(Box::new(|panic_info| {
        use std::io::Write;

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/relation-panic.log")
        {
            let _ = writeln!(file, "panic: {panic_info}");
            let backtrace = std::backtrace::Backtrace::force_capture();
            let _ = writeln!(file, "{backtrace}");
        }
    }));

    let mut tui = setup_tty()?;

    let old_log = app.get_data_path().join("box.log");
    let _ = std::fs::write(&old_log, "");

    let mut prev = read_iface(&iface)?;
    let mut prev_time = Instant::now();

    let mut rx_rate: u64;
    let mut tx_rate: u64;

    let mut rx_list: VecDeque<u64> = VecDeque::new();
    let mut tx_list: VecDeque<u64> = VecDeque::new();

    let mut state = TuiState::new(app)?; 

    let current_ip = Arc::new(Mutex::new(net::LOADING_IP.to_string()));
    let ip_shared = Arc::clone(&current_ip);

    let change_flag = Arc::new(Mutex::new(true));
    let change_shared = Arc::clone(&change_flag);

    #[cfg(not(feature = "daemon"))]
    thread::spawn(move || {
        loop {
            let need_refresh = {
                if let Ok(mut flag) = change_shared.lock() {
                    if *flag {
                        *flag = false;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if need_refresh {
                let ip = match get_ip(Some(net::LOCAL_PROXY_ADDR)) {
                    Ok(ip) => ip,
                    Err(_) => match get_ip(None) {
                        Ok(ip) => ip,
                        Err(_) => net::FALLBACK_IP.to_string(),
                    },
                };

                if let Ok(mut ip_address) = ip_shared.lock() {
                    *ip_address = ip;
                }
            }

            thread::sleep(timing::IP_REFRESH_SLEEP);
        }
    });

    #[cfg(feature = "daemon")]
    thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(_) => return,
        };

        rt.block_on(async move {
            loop {
                let need_refresh = {
                    if let Ok(mut flag) = change_shared.lock() {
                        if *flag {
                            *flag = false;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if need_refresh {
                    let ip = match tokio::time::timeout(
                        timing::IP_REQUEST_TIMEOUT,
                        get_ip(Some(net::LOCAL_PROXY_ADDR)),
                    )
                    .await
                    {
                        Ok(Ok(ip)) => ip,
                        _ => match tokio::time::timeout(timing::IP_REQUEST_TIMEOUT, get_ip(None))
                            .await
                        {
                            Ok(Ok(ip)) => ip,
                            _ => net::FALLBACK_IP.to_string(),
                        },
                    };

                    if let Ok(mut ip_address) = ip_shared.lock() {
                        *ip_address = ip;
                    }
                }

                tokio::time::sleep(timing::IP_REFRESH_SLEEP).await;
            }
        });
    });

    loop {
        let ip_base = current_ip
            .lock()
            .map(|ip| ip.clone())
            .unwrap_or_else(|_| net::UNAVAILABLE_IP.to_string());

        // -------- INPUT --------
        if event::poll(timing::EVENT_POLL)? {
            if let Event::Key(key) = event::read()? {
                match state.moder() {
                    InputMode::AddConfig => {
                        handle_add_config_input(app, &mut state, key.code, false)?; 
                    }
                    InputMode::AddTunConfig => {
                        handle_add_config_input(app, &mut state, key.code, true)?; 
                    } 
                    InputMode::RouteValue => {
                        handle_route_value_input(&mut state, key.code);
                    }
                    InputMode::Normal => {
                        match handle_normal_input(app, &mut state, key.code, &change_flag)? {
                            InputAction::Continue => {}
                            InputAction::Quit => break
                        }
                    }
                }
            }
        }

        if prev_time.elapsed() >= timing::TRAFFIC_REFRESH {
            let now = Instant::now();
            let dt = (now - prev_time).as_secs_f64().max(0.001);

            let current = read_iface(&iface)?;

            let drx = current.rx.saturating_sub(prev.rx) as f64;
            let dtx = current.tx.saturating_sub(prev.tx) as f64;

            rx_rate = (drx / dt) as u64;
            tx_rate = (dtx / dt) as u64;

            rx_list.push_back(rx_rate);
            tx_list.push_back(tx_rate);
            while rx_list.len() > traffic::HISTORY_LIMIT {
                rx_list.pop_front();
            }
            while tx_list.len() > traffic::HISTORY_LIMIT {
                tx_list.pop_front();
            }

            prev = current;
            prev_time = now;
        }

        tui.terminal.draw(|f| {
            let size = f.area();

            let root = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(ui::HELP_HEIGHT)])
                .split(size);
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(ui::LEFT_PANEL_WEIGHT),
                    Constraint::Fill(ui::RIGHT_PANEL_WEIGHT),
                ])
                .split(root[0]);

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Fill(1)])
                .split(horizontal[0]);

            let configs = app.get_list();

            let items: Vec<ListItem> = configs
                .iter()
                .map(|name| {
                    let is_running = state.app.running.as_deref() == Some(name.as_str());
                    if is_running {
                        ListItem::new(Line::from(vec![
                            Span::styled(
                                ui::RUNNING_SYMBOL,
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                name.clone(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ]))
                    } else {
                        ListItem::new(name.clone())
                    }
                })
                .collect();

            let mut tui_state = ListState::default();
            tui_state.select(Some(state.app.selected_index));

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(text::CONFIGS_TITLE)
                        .borders(Borders::ALL)
                        .border_style(if !state.ui.transit {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        })
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::LightCyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(ui::SELECTED_SYMBOL);

            f.render_stateful_widget(list, vertical[0], &mut tui_state);

            // ADDING CONFIG LINE
            if state.input.input_mode && !state.input.tun_mode {
                let (color, message) = if state.input.error_input {
                    (Color::Red, text::ERROR_INPUT)
                } else {
                    (Color::Yellow, text::ADD_CONFIG_URL)
                };

                let input = Paragraph::new(state.input.buffer.as_str())
                    .wrap(Wrap { trim: true })
                    .block(
                        Block::default()
                            .title(message)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(color));

                let input_area = ratatui::layout::Rect {
                    x: vertical[0].x,
                    y: vertical[0].y + vertical[0].height - ui::INPUT_HEIGHT,
                    width: vertical[0].width,
                    height: ui::INPUT_HEIGHT,
                };
                f.render_widget(input, input_area);
            }

            // ADDING TUN CONFIG LINE
            if state.input.input_mode && state.input.tun_mode {
                let (color, message) = if state.input.error_input {
                    (Color::Red, text::ERROR_INPUT)
                } else {
                    (Color::Blue, text::ADD_TUN_CONFIG_URL)
                };

                let input = Paragraph::new(state.input.buffer.as_str())
                    .block(
                        Block::default()
                            .title(message)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(color));

                let input_area = ratatui::layout::Rect {
                    x: vertical[0].x,
                    y: vertical[0].y + vertical[0].height - ui::INPUT_HEIGHT,
                    width: vertical[0].width,
                    height: ui::INPUT_HEIGHT,
                };
                f.render_widget(input, input_area);
            }

            // TRAFFIC BAR
            render_traffic_bar(f, vertical[1], &iface, &ip_base, &rx_list, &tx_list);

            // HELP PANEL
            let helper = Paragraph::new(Line::from(text::HELP))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(helper, root[1]);

            // LOG/SETTINGS PANEL
            if !state.ui.settings_panel {
                let logs = app.get_logs();
                let log_items: Vec<ListItem> =
                    logs.iter().map(|l| ListItem::new(l.clone())).collect();
                let log_list = List::new(log_items).block(
                    Block::default()
                        .title(Line::from(text::LOGS_TITLE))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );

                f.render_widget(log_list, horizontal[1]);
                app.read_logs();
            } else {
                let action_text = state.settings.route_action.as_deref().unwrap_or(text::EMPTY);
                let type_text = state.settings.route_type.as_deref().unwrap_or(text::EMPTY);
                let value_text = state.settings.route_value.as_deref().unwrap_or(text::EMPTY);
                let type_dns_text = state.settings.dns_type.as_deref().unwrap_or(text::EMPTY);
                let value1_text = state.settings.dns_value1.as_deref().unwrap_or(text::EMPTY);
                let value2_text = state.settings.dns_value2.as_deref().unwrap_or(text::EMPTY);

                let action_style = if state.ui.transit && state.ui.settings_selected == ui::ROUTE_ACTION_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let type_style = if state.ui.transit && state.ui.settings_selected == ui::ROUTE_TYPE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let value_style = if state.ui.transit && state.ui.settings_selected == ui::ROUTE_VALUE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_type_style = if state.ui.transit && state.ui.settings_selected == ui::DNS_TYPE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_value1_style = if state.ui.transit && state.ui.settings_selected == ui::DNS_VALUE1_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_value2_style = if state.ui.transit && state.ui.settings_selected == ui::DNS_VALUE2_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let enter_style = if state.ui.transit && state.ui.settings_selected == ui::ENTER_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };

                let settings = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("         "),
                        Span::raw(text::ROUTING_RULES_TITLE),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(text::ACTION_LABEL, action_style),
                        Span::styled(action_text, action_style),
                        Span::raw("   "),
                        Span::styled(text::TYPE_LABEL, type_style),
                        Span::styled(type_text, type_style),
                        Span::raw("   "),
                        Span::styled(text::VALUE_LABEL, value_style),
                        Span::styled(value_text, value_style),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("         "),
                        Span::raw(text::DNS_SERVERS_TITLE),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(text::TYPE_LABEL, dns_type_style),
                        Span::styled(type_dns_text, dns_type_style),
                        Span::raw("             "),
                        Span::styled(text::VALUE1_LABEL, dns_value1_style),
                        Span::styled(value1_text, dns_value1_style),
                        Span::raw("             "),
                        Span::styled(text::VALUE2_LABEL, dns_value2_style),
                        Span::styled(value2_text, dns_value2_style),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("                 "),
                        Span::styled(text::ENTER_BUTTON, enter_style),
                    ]),
                ])
                .block(
                    Block::default()
                        .title(text::SETTINGS_TITLE)
                        .borders(Borders::ALL)
                        .border_style(if state.ui.transit {
                            Style::default().fg(Color::Blue)
                        } else {
                            Style::default()
                        })
                        .border_type(BorderType::Rounded),
                );
                f.render_widget(settings, horizontal[1]);
            }

            // Context Menu
            if state.ui.context_menu {
                let context_panel_area = ratatui::layout::Rect {
                    x: horizontal[1].x + ui::CONTEXT_X_OFFSET,
                    y: horizontal[1].y + ui::CONTEXT_Y_OFFSET,
                    width: horizontal[1]
                        .width
                        .saturating_sub(ui::CONTEXT_WIDTH_PADDING),
                    height: ui::CONTEXT_HEIGHT,
                };

                f.render_widget(Clear, context_panel_area);

                let context_items: Vec<ListItem> = if state.ui.settings_selected == ui::ROUTE_ACTION_INDEX {
                    let mut items: Vec<ListItem> =
                        route::ACTIONS.iter().copied().map(ListItem::new).collect();

                    if state.ui.custom {
                        items.push(ListItem::new(format!(
                            "{}{}",
                            text::INPUT_PREFIX,
                            state.input.buffer
                        )));
                    } else {
                        items.push(ListItem::new(text::PERSONAL));
                    }

                    items
                } else if state.ui.settings_selected == ui::ROUTE_TYPE_INDEX {
                    route::TYPES
                        .iter()
                        .map(|(label, _)| ListItem::new(*label))
                        .collect()
                } else {
                    vec![text::NO_ITEMS]
                        .into_iter()
                        .map(ListItem::new)
                        .collect()
                };

                let mut tui_state = ListState::default();
                tui_state.select(Some(state.ui.popup_selected));

                let list = List::new(context_items)
                    .block(
                        Block::default()
                            .title(text::SELECT_TITLE)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(ui::SELECTED_SYMBOL);
                f.render_stateful_widget(list, context_panel_area, &mut tui_state);
            }

            if state.ui.value_input {
                let input = Paragraph::new(state.input.buffer.as_str())
                    .block(
                        Block::default()
                            .title(text::ENTER_VALUE)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Green));

                let area = ratatui::layout::Rect {
                    x: horizontal[1].x + ui::VALUE_INPUT_X_OFFSET,
                    y: horizontal[1].y + ui::VALUE_INPUT_Y_OFFSET,
                    width: horizontal[1]
                        .width
                        .saturating_sub(ui::VALUE_INPUT_WIDTH_PADDING),
                    height: ui::VALUE_INPUT_HEIGHT,
                };

                f.render_widget(Clear, area);
                f.render_widget(input, area);
            }
        })?;
    }

    Ok(())
}

fn render_traffic_bar(
    f: &mut Frame,
    area: Rect,
    iface: &str,
    ip_base: &str,
    rx_list: &VecDeque<u64>,
    tx_list: &VecDeque<u64>,
) {
    let max_rate = rx_list
        .iter()
        .chain(tx_list.iter())
        .copied()
        .max()
        .unwrap_or(traffic::MIN_RATE)
        .max(traffic::MIN_RATE);

    let current_rate = rx_list
        .back()
        .copied()
        .unwrap_or_default()
        .max(tx_list.back().copied().unwrap_or_default());

    let title = if current_rate as f64 >= traffic::MB {
        format!(
            "{} ({:.1}) MB/s",
            text::TRAFFIC_TITLE,
            current_rate as f64 / traffic::MB
        )
    } else {
        format!(
            "{} ({:.0}) KB/s",
            text::TRAFFIC_TITLE,
            current_rate as f64 / traffic::KB
        )
    };

    let traffic_block = Block::default()
        .title(title)
        .title(Title::from(format!("{}: {}", iface, ip_base)).position(Position::Bottom))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let traffic_inner = traffic_block.inner(area);
    f.render_widget(traffic_block, area);

    if traffic_inner.width <= traffic::MIN_WIDTH || traffic_inner.height <= traffic::MIN_HEIGHT {
        return;
    }

    let traffic_width = traffic_inner.width as usize;
    let traffic_height = traffic_inner.height as usize;

    if traffic_width == 0 || traffic_height < 2 {
        return;
    }

    let rx_rows = traffic_height / 2;
    let tx_rows = traffic_height - rx_rows;

    if rx_rows == 0 || tx_rows == 0 {
        return;
    }

    let rx_area_y = traffic_inner.y;
    let tx_area_y = traffic_inner.y + rx_rows as u16;

    let samples_limit = traffic_width * 2;

    let rx_points = latest_points(rx_list, samples_limit);
    let tx_points = latest_points(tx_list, samples_limit);

    render_braille_series(
        f,
        traffic_inner.x,
        rx_area_y,
        traffic_width,
        rx_rows,
        &rx_points,
        max_rate,
        false,
        Color::Cyan,
    );

    render_braille_series(
        f,
        traffic_inner.x,
        tx_area_y,
        traffic_width,
        tx_rows,
        &tx_points,
        max_rate,
        true,
        Color::Magenta,
    );
}

fn latest_points(list: &VecDeque<u64>, limit: usize) -> Vec<u64> {
    list.iter()
        .rev()
        .take(limit)
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn render_braille_series(
    f: &mut Frame,
    x: u16,
    y: u16,
    width: usize,
    rows: usize,
    points: &[u64],
    max_rate: u64,
    top_down: bool,
    color: Color,
) {
    if width == 0 || rows == 0 || points.is_empty() {
        return;
    }

    let max_level = rows * 4;

    // В одной braille-ячейке рисуются 2 значения.
    let cell_count = points.len().div_ceil(2).min(width);

    if cell_count == 0 {
        return;
    }

    // Выравниваем по правому краю:
    // новые значения появляются справа, старые уходят влево.
    let start_x = x + width.saturating_sub(cell_count) as u16;

    for (cell_id, pair) in points.chunks(2).take(cell_count).enumerate() {
        let left_value = pair.first().copied().unwrap_or_default();

        // Если правого значения нет, дублируем левое,
        // чтобы последняя ячейка не была наполовину пустой.
        let right_value = pair.get(1).copied().unwrap_or(left_value);

        let left_level = value_to_braille_level(left_value, max_rate, max_level);
        let right_level = value_to_braille_level(right_value, max_rate, max_level);

        for row in 0..rows {
            let left_part = row_braille_part(left_level, row, rows, top_down);
            let right_part = row_braille_part(right_level, row, rows, top_down);

            let direction = if top_down { 100 } else { 0 };
            let key = direction + left_part * 10 + right_part;

            let Some(ch) = traffic::BAR_MAP.get(&(key as u8)).copied() else {
                continue;
            };

            let cell_x = start_x + cell_id as u16;
            let cell_y = y + row as u16;

            if let Some(cell) = f.buffer_mut().cell_mut((cell_x, cell_y)) {
                cell.set_char(ch);
                cell.set_style(Style::default().fg(color));
            }
        }
    }
}

fn value_to_braille_level(value: u64, max_rate: u64, max_level: usize) -> usize {
    if max_level == 0 {
        return 0;
    }

    let level = ((value as f64 / max_rate.max(1) as f64) * max_level as f64).round() as usize;

    // Минимум 1 уровень, чтобы нижний/верхний ряд давал key 11,
    // а не 00. Так столбцы не будут пустыми.
    level.clamp(1, max_level)
}

fn row_braille_part(level: usize, row: usize, rows: usize, top_down: bool) -> u8 {
    let filled_before_row = if top_down {
        row * 4
    } else {
        rows.saturating_sub(row + 1) * 4
    };

    level.saturating_sub(filled_before_row).min(4) as u8
}


