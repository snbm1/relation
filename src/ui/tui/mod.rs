mod consts;
mod ifaces;
mod minireq;
mod tuiguard;

use consts::*;
use ifaces::*;
use minireq::*;
use std::{
    io,
    os::fd::{AsRawFd, FromRawFd},
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

use anyhow::Result;

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<File>>,
    pub _guard: TuiGuard,
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

pub fn run(app: &mut App) -> Result<()> {
    let iface = iface_detect();

    let mut tui = setup_tty()?;

    let old_log = app.get_data_path().join("box.log");
    let _ = std::fs::write(&old_log, "");

    let mut prev = read_iface(&iface)?;
    let mut prev_time = Instant::now();

    let mut rx_rate: u64;
    let mut tx_rate: u64;

    let mut rx_list: VecDeque<u64> = VecDeque::new();
    let mut tx_list: VecDeque<u64> = VecDeque::new();

    let mut selected_index = 0;
    let mut len = app.get_len();

    let mut enter_mode = false;
    let mut input_mode = false;
    let mut tun_mode = false;
    let mut error_input = false;
    let mut running: Option<String> = app.get_status()?.map(|s| {
        PathBuf::from(s.file)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    });

    let mut transit = false;

    // settings vars
    let mut settings_panel = true;

    // Route var
    let mut rule_action: Option<String> = None;
    let mut rule_type: Option<String> = None;
    let mut rule_value: Option<String> = None;

    // DNS settings
    let mut type_dns_action: Option<String> = None;
    let mut dns_value1: Option<String> = None;
    let mut dns_value2: Option<String> = None;

    let mut settings_selected = 0;
    let mut context_menu = false;
    let mut popup_selected = 0;
    let mut value_input = false;

    let mut input_buffer = String::new();

    let mut custom = false;

    let current_ip = Arc::new(Mutex::new(net::LOADING_IP.to_string()));
    let ip_shared = Arc::clone(&current_ip);

    let change_flag = Arc::new(Mutex::new(true));
    let change_shared = Arc::clone(&change_flag);

    thread::spawn(move || {
        loop {
            if let Ok(mut flag) = change_shared.lock()
                && *flag
            {
                let ip = match get_ip(Some(net::LOCAL_PROXY_ADDR)) {
                    Ok(ip) => ip,
                    Err(_) => match get_ip(None) {
                        Ok(ip) => ip,
                        Err(_) => net::FALLBACK_IP.to_string(),
                    },
                };
                *flag = false;

                if let Ok(mut ip_address) = ip_shared.lock() {
                    *ip_address = ip;
                }
            }

            thread::sleep(timing::IP_REFRESH_SLEEP);
        }
    });

    loop {
        let ip_base = current_ip
            .lock()
            .map(|ip| ip.clone())
            .unwrap_or_else(|_| net::UNAVAILABLE_IP.to_string());

        // -------- INPUT --------
        if event::poll(timing::EVENT_POLL)? {
            if let Event::Key(key) = event::read()? {
                if input_mode {
                    match key.code {
                        KeyCode::Esc => {
                            input_mode = false;
                            if tun_mode {
                                tun_mode = false;
                            }
                            error_input = false;
                            input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !input_buffer.is_empty() {
                                let cfg = app.handler_mut().clean();
                                let result = if tun_mode {
                                    cfg.default_tun()
                                        .set_outbound_from_url(&input_buffer.clone())
                                } else {
                                    cfg.default().set_outbound_from_url(&input_buffer.clone())
                                };

                                match result {
                                    Ok(_) => {
                                        if let Err(_) = app.add_config(None) {
                                            error_input = true;
                                        } else {
                                            error_input = false;
                                            input_buffer.clear();
                                            len = app.get_len();
                                            input_mode = false;
                                            if tun_mode {
                                                tun_mode = false;
                                            }
                                            selected_index = 0;
                                        }
                                    }
                                    Err(_) => {
                                        error_input = true;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            input_buffer.pop();
                            if input_buffer.is_empty() {
                                error_input = false;
                            }
                        }
                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        }
                        _ => {}
                    }
                } else if value_input {
                    match key.code {
                        KeyCode::Esc => {
                            value_input = false;
                            input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !input_buffer.is_empty() {
                                rule_value = Some(input_buffer.clone());
                            }
                            value_input = false;
                            input_buffer.clear();
                        }
                        KeyCode::Backspace => {
                            input_buffer.pop();
                        }

                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        }

                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => {
                            if running.is_some() {
                                app.send_quit()?;
                            }

                            break;
                        }

                        KeyCode::Char(keys::QUIT) => {
                            if context_menu {
                                if custom {
                                    custom = false;
                                    input_buffer.clear();
                                } else {
                                    context_menu = false;
                                    popup_selected = 0;
                                }
                            } else {
                                break;
                            }
                        }

                        KeyCode::Char(keys::ADD_CONFIG) => {
                            input_mode = true;
                        }
                        KeyCode::Char(keys::ADD_TUN_CONFIG) => {
                            input_mode = true;
                            tun_mode = true;
                        }
                        KeyCode::Char(keys::DELETE_CONFIG) => {
                            if len > 0 {
                                let name = app.get_list()[selected_index].clone();
                                app.remove_config_by_number(selected_index)?;

                                if running.as_deref() == Some(name.as_str()) {
                                    app.stop_app()?;
                                    running = None;
                                    enter_mode = false;
                                }

                                len = app.get_len();

                                if selected_index >= len && len > 0 {
                                    selected_index = len - 1;
                                }
                            }
                        }
                        KeyCode::Tab => {
                            settings_panel = !settings_panel;
                        }

                        KeyCode::Char(c) => {
                            if context_menu
                                && settings_selected == ui::ROUTE_ACTION_INDEX
                                && custom
                                && popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX
                            {
                                input_buffer.push(c);
                            }
                        }

                        KeyCode::Backspace => {
                            if context_menu
                                && settings_selected == ui::ROUTE_ACTION_INDEX
                                && custom
                                && popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX
                            {
                                input_buffer.pop();
                            }
                        }

                        KeyCode::Enter => {
                            if settings_selected == ui::DNS_TYPE_INDEX {
                                let mut route_rules: Vec<String> = Vec::new();
                                if let Some(action) = rule_action.as_ref() {
                                    route_rules.push(action.to_string());
                                }
                                if let Some(r_type) = rule_type.as_ref() {
                                    route_rules.push(r_type.to_string());
                                }
                                if let Some(value) = rule_value.as_ref() {
                                    route_rules.push(value.to_string());
                                }
                                let route_rules = vec![route_rules.join(":")];
                                app.handler_mut().add_route_rules(&route_rules)?;
                            }

                            if context_menu {
                                match settings_selected {
                                    ui::ROUTE_ACTION_INDEX => {
                                        if popup_selected == ui::ROUTE_ACTION_CUSTOM_INDEX {
                                            if !custom {
                                                custom = true;
                                                input_buffer.clear();
                                            } else if !input_buffer.is_empty() {
                                                rule_action = Some(input_buffer.clone());
                                                custom = false;
                                                input_buffer.clear();
                                            }
                                        } else if let Some(value) =
                                            route::ACTIONS.get(popup_selected)
                                        {
                                            rule_action = Some((*value).to_string());
                                            context_menu = false;
                                            popup_selected = 0;
                                        }
                                    }

                                    ui::ROUTE_TYPE_INDEX => {
                                        if let Some((_, value)) = route::TYPES.get(popup_selected) {
                                            rule_type = Some((*value).to_string());
                                        }
                                    }

                                    _ => {}
                                }
                                if !custom {
                                    context_menu = false;
                                    popup_selected = 0;
                                }
                            } else if transit && settings_panel {
                                if settings_selected == ui::ROUTE_VALUE_INDEX {
                                    value_input = true;
                                    input_buffer.clear();
                                } else if settings_selected != ui::DNS_TYPE_INDEX {
                                    context_menu = true;
                                    popup_selected = 0;
                                }
                            } else {
                                let len = app.get_len();
                                if let Ok(mut flag) = change_flag.lock() {
                                    *flag = true;
                                }
                                if len > 0 && !enter_mode {
                                    let number = selected_index as u16 + 1;
                                    running = Some(app.get_list()[selected_index].clone());
                                    app.set_log_file();
                                    app.run_app(None, Some(number as usize - 1), false)?;
                                    settings_panel = false;
                                    transit = false;
                                    enter_mode = true;
                                } else if enter_mode {
                                    let name = app.get_list()[selected_index].clone();
                                    if running.as_deref() == Some(name.as_str()) {
                                        running = None;
                                        app.stop_app()?;
                                        enter_mode = false;
                                    } else {
                                        app.stop_app()?;
                                        std::thread::sleep(timing::RESTART_DELAY);
                                        let number = selected_index as u16 + 1;
                                        running = Some(name.clone());
                                        app.set_log_file();
                                        app.run_app(None, Some(number as usize - 1), false)?;
                                        settings_panel = false;
                                        transit = false;
                                    }
                                }
                            }
                        }

                        KeyCode::Right => {
                            if !transit {
                                transit = true;
                            } else if transit && settings_panel {
                                settings_selected =
                                    (settings_selected + 1) % ui::SETTINGS_FIELDS_COUNT;
                            }
                        }
                        KeyCode::Left => {
                            if settings_selected - 1 < 0 && transit {
                                transit = false;
                            } else if transit && settings_panel {
                                settings_selected = (settings_selected + ui::ROUTE_FIELDS_COUNT
                                    - 1)
                                    % ui::ROUTE_FIELDS_COUNT;
                            }
                        }

                        KeyCode::Down | KeyCode::Char(keys::DOWN_ALT) => {
                            if !transit && len > 0 {
                                selected_index = (selected_index + 1) % len;
                            } else if context_menu {
                                let context_len = if settings_selected == ui::ROUTE_ACTION_INDEX {
                                    route::ACTIONS.len() + 1
                                } else if settings_selected == ui::ROUTE_TYPE_INDEX {
                                    route::TYPES.len()
                                } else {
                                    1
                                };
                                popup_selected = (popup_selected + 1) % context_len;
                            } else if transit && settings_panel && !value_input {
                                settings_selected = match settings_selected {
                                    ui::ROUTE_ACTION_INDEX => ui::DNS_TYPE_INDEX,
                                    ui::ROUTE_TYPE_INDEX => ui::DNS_TYPE_INDEX,
                                    ui::ROUTE_VALUE_INDEX => ui::DNS_VALUE1_INDEX,
                                    ui::DNS_TYPE_INDEX => ui::DNS_VALUE2_INDEX,
                                    ui::DNS_VALUE1_INDEX => ui::DNS_VALUE2_INDEX,

                                    _ => settings_selected,
                                };
                            }
                        }

                        KeyCode::Up | KeyCode::Char(keys::UP_ALT) => {
                            if len > 0 && !transit {
                                selected_index = (selected_index + len - 1) % len;
                            } else if context_menu {
                                let context_len = if settings_selected == ui::ROUTE_ACTION_INDEX {
                                    route::ACTIONS.len() + 1
                                } else if settings_selected == ui::ROUTE_TYPE_INDEX {
                                    route::TYPES.len()
                                } else {
                                    1
                                };
                                popup_selected = (popup_selected + context_len - 1) % context_len;
                            } else if transit && settings_panel && !value_input {
                                settings_selected = match settings_selected {
                                    ui::DNS_VALUE2_INDEX => ui::DNS_VALUE1_INDEX,
                                    ui::DNS_TYPE_INDEX => ui::ROUTE_ACTION_INDEX,
                                    ui::DNS_VALUE1_INDEX => ui::ROUTE_TYPE_INDEX,

                                    _ => settings_selected,
                                };
                            }
                        }

                        _ => {}
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
                    let is_running = running.as_deref() == Some(name.as_str());
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

            let mut state = ListState::default();
            state.select(Some(selected_index));

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(text::CONFIGS_TITLE)
                        .borders(Borders::ALL)
                        .border_style(if !transit {
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

            f.render_stateful_widget(list, vertical[0], &mut state);

            // ADDING CONFIG LINE
            if input_mode && !tun_mode {
                let (color, message) = if error_input {
                    (Color::Red, text::ERROR_INPUT)
                } else {
                    (Color::Yellow, text::ADD_CONFIG_URL)
                };

                let input = Paragraph::new(input_buffer.as_str())
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
            if input_mode && tun_mode {
                let (color, message) = if error_input {
                    (Color::Red, text::ERROR_INPUT)
                } else {
                    (Color::Blue, text::ADD_TUN_CONFIG_URL)
                };

                let input = Paragraph::new(input_buffer.as_str())
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
            if !settings_panel {
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
                let action_text = rule_action.as_deref().unwrap_or(text::EMPTY);
                let type_text = rule_type.as_deref().unwrap_or(text::EMPTY);
                let value_text = rule_value.as_deref().unwrap_or(text::EMPTY);
                let type_dns_text = type_dns_action.as_deref().unwrap_or(text::EMPTY);
                let value1_text = dns_value1.as_deref().unwrap_or(text::EMPTY);
                let value2_text = dns_value2.as_deref().unwrap_or(text::EMPTY);

                let action_style = if transit && settings_selected == ui::ROUTE_ACTION_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let type_style = if transit && settings_selected == ui::ROUTE_TYPE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let value_style = if transit && settings_selected == ui::ROUTE_VALUE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_type_style = if transit && settings_selected == ui::DNS_TYPE_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_value1_style = if transit && settings_selected == ui::DNS_VALUE1_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let dns_value2_style = if transit && settings_selected == ui::DNS_VALUE2_INDEX {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let enter_style = if transit && settings_selected == ui::ENTER_INDEX {
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
                        .border_style(if transit {
                            Style::default().fg(Color::Blue)
                        } else {
                            Style::default()
                        })
                        .border_type(BorderType::Rounded),
                );
                f.render_widget(settings, horizontal[1]);
            }

            // Context Menu
            if context_menu {
                let context_panel_area = ratatui::layout::Rect {
                    x: horizontal[1].x + ui::CONTEXT_X_OFFSET,
                    y: horizontal[1].y + ui::CONTEXT_Y_OFFSET,
                    width: horizontal[1]
                        .width
                        .saturating_sub(ui::CONTEXT_WIDTH_PADDING),
                    height: ui::CONTEXT_HEIGHT,
                };

                f.render_widget(Clear, context_panel_area);

                let context_items: Vec<ListItem> = if settings_selected == ui::ROUTE_ACTION_INDEX {
                    let mut items: Vec<ListItem> =
                        route::ACTIONS.iter().copied().map(ListItem::new).collect();

                    if custom {
                        items.push(ListItem::new(format!(
                            "{}{}",
                            text::INPUT_PREFIX,
                            input_buffer
                        )));
                    } else {
                        items.push(ListItem::new(text::PERSONAL));
                    }

                    items
                } else if settings_selected == ui::ROUTE_TYPE_INDEX {
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

                let mut state = ListState::default();
                state.select(Some(popup_selected));

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
                f.render_stateful_widget(list, context_panel_area, &mut state);
            }

            if value_input {
                let input = Paragraph::new(input_buffer.as_str())
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

    let rx_rows = (traffic_height / 2).max(1);
    let tx_rows = traffic_height.saturating_sub(rx_rows).max(1);

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
        .into_iter()
        .copied()
        .collect::<Vec<_>>()
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

    // Выравниваем по правому краю:
    // новые значения появляются справа, старые уходят влево.
    let start_x = x + width.saturating_sub(cell_count) as u16;

    for (cell_id, pair) in points.chunks(2).take(width).enumerate() {
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
