mod ifaces;
mod minireq;
mod tuiguard;

use ifaces::*;
use minireq::*;
use std::{
    io,
    os::fd::{AsRawFd, FromRawFd},
    time::{Duration, Instant},
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
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
        block::{Position, Title},
    },
};

use anyhow::{Context, Result, anyhow};

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<File>>,
    pub guard: TuiGuard,
}

fn setup_tty() -> Result<Tui> {
    let guard = TuiGuard::new()?;

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

    Ok(Tui { terminal, guard })
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
    // let mut context_menu_selected = 0;

    let mut input_buffer = String::new();

    let mut custom = false;

    let current_ip = Arc::new(Mutex::new("loading...".to_string()));
    let ip_shared = Arc::clone(&current_ip);

    let change_flag = Arc::new(Mutex::new(true));
    let change_shared = Arc::clone(&change_flag);

    thread::spawn(move || {
        loop {
            if let Ok(mut flag) = change_shared.lock()
                && *flag
            {
                let ip = match get_ip(Some("127.0.0.1:12334")) {
                    Ok(ip) => ip,
                    Err(_) => match get_ip(None) {
                        Ok(ip) => ip,
                        Err(_) => "0.0.0.0".to_string(),
                    },
                };
                *flag = false;

                if let Ok(mut ip_address) = ip_shared.lock() {
                    *ip_address = ip;
                }
            }

            thread::sleep(Duration::from_millis(200));
        }
    });

    loop {
        let ip_base = current_ip
            .lock()
            .map(|ip| ip.clone())
            .unwrap_or_else(|_| "ip unavailable".to_string());

        // -------- INPUT --------
        if event::poll(Duration::from_millis(200))? {
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
                        KeyCode::Char('q') => {
                            if running.is_some() {
                                // #[cfg(not(feature = "daemon"))]
                                app.send_quit()?;
                            }

                            break;
                        }

                        KeyCode::Esc => {
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

                        KeyCode::Char('a') => {
                            input_mode = true;
                        }
                        KeyCode::Char('A') => {
                            input_mode = true;
                            tun_mode = true;
                        }
                        KeyCode::Char('d') => {
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
                                && settings_selected == 0
                                && custom
                                && popup_selected == 3
                            {
                                input_buffer.push(c);
                            }
                        }

                        KeyCode::Backspace => {
                            if context_menu
                                && settings_selected == 0
                                && custom
                                && popup_selected == 3
                            {
                                input_buffer.pop();
                            }
                        }

                        KeyCode::Enter => {
                            if settings_selected == 3 {
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
                                    0 => {
                                        if popup_selected == 3 {
                                            if !custom {
                                                custom = true;
                                                input_buffer.clear();
                                            } else if !input_buffer.is_empty() {
                                                rule_action = Some(input_buffer.clone());
                                                custom = false;
                                                input_buffer.clear();
                                            }
                                        } else {
                                            let value = match popup_selected {
                                                0 => "r",
                                                1 => "h",
                                                2 => "s",
                                                _ => "",
                                            };
                                            if !value.is_empty() {
                                                rule_action = Some(value.to_string());
                                                context_menu = false;
                                                popup_selected = 0;
                                            }
                                        }
                                    }

                                    1 => {
                                        let value = match popup_selected {
                                            0 => "ib",
                                            1 => "iv",
                                            2 => "au",
                                            3 => "pl",
                                            4 => "cl",
                                            5 => "dm",
                                            6 => "ds",
                                            7 => "dk",
                                            8 => "dr",
                                            9 => "gs",
                                            10 => "sg",
                                            11 => "gp",
                                            12 => "sc",
                                            13 => "si",
                                            14 => "ic",
                                            15 => "ip",
                                            16 => "sp",
                                            17 => "sr",
                                            18 => "pt",
                                            19 => "pr",
                                            20 => "pn",
                                            21 => "pp",
                                            22 => "pg",
                                            23 => "kn",
                                            24 => "ur",
                                            25 => "ui",
                                            26 => "cm",
                                            27 => "nt",
                                            28 => "nk",
                                            29 => "ne",
                                            30 => "nc",
                                            _ => "",
                                        };

                                        if !value.is_empty() {
                                            rule_type = Some(value.to_string());
                                        }
                                    }

                                    _ => {}
                                }
                                if !custom {
                                    context_menu = false;
                                    popup_selected = 0;
                                }
                            } else if transit && settings_panel {
                                if settings_selected == 2 {
                                    value_input = true;
                                    input_buffer.clear();
                                } else {
                                    if settings_selected != 3 {
                                        context_menu = true;
                                        popup_selected = 0;
                                    }
                                }
                                // context_menu_selected = settings_selected;
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
                                        std::thread::sleep(Duration::from_millis(100));
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
                                settings_selected = (settings_selected + 1) % 6;
                            }
                        }
                        KeyCode::Left => {
                            if settings_selected - 1 < 0 && transit {
                                transit = false;
                            } else if transit && settings_panel {
                                settings_selected = (settings_selected + 3 - 1) % 3;
                            }
                        }

                        KeyCode::Down | KeyCode::Char('j') => {
                            if !transit && len > 0 {
                                selected_index = (selected_index + 1) % len;
                            } else if context_menu {
                                let context_len = if settings_selected == 0 {
                                    4
                                } else if settings_selected == 1 {
                                    31
                                } else {
                                    1
                                };
                                popup_selected = (popup_selected + 1) % context_len;
                            } else if transit && settings_panel && !value_input {
                                settings_selected = match settings_selected {
                                    0 => 3,
                                    1 => 3,
                                    2 => 4,
                                    3 => 5,
                                    4 => 5,

                                    _ => settings_selected,
                                };
                            }
                        }

                        KeyCode::Up | KeyCode::Char('k') => {
                            if len > 0 && !transit {
                                selected_index = (selected_index + len - 1) % len;
                            } else if context_menu {
                                let context_len = if settings_selected == 0 {
                                    4
                                } else if settings_selected == 1 {
                                    31
                                } else {
                                    1
                                };
                                popup_selected = (popup_selected + context_len - 1) % context_len;
                            } else if transit && settings_panel && !value_input {
                                settings_selected = match settings_selected {
                                    5 => 4,
                                    3 => 0,
                                    4 => 1,

                                    _ => settings_selected,
                                };
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        if prev_time.elapsed() >= Duration::from_millis(200) {
            let now = Instant::now();
            let dt = (now - prev_time).as_secs_f64().max(0.001);

            let current = read_iface(&iface)?;

            let drx = current.rx.saturating_sub(prev.rx) as f64;
            let dtx = current.tx.saturating_sub(prev.tx) as f64;

            rx_rate = (drx / dt) as u64;
            tx_rate = (dtx / dt) as u64;

            rx_list.push_back(rx_rate);
            tx_list.push_back(tx_rate);
            while rx_list.len() > 900 {
                rx_list.pop_front();
            }
            while tx_list.len() > 900 {
                tx_list.pop_front();
            }

            prev = current;
            prev_time = now;
        }

        tui.terminal.draw(|f| {
            let size = f.area();

            let root = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Fill(2), Constraint::Fill(5)])
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
                                "● ",
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
                        .title("Configs")
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
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, vertical[0], &mut state);

            //ADDING CONFIG LINE
            if input_mode && !tun_mode {
                let (color, message) = if error_input {
                    (Color::Red, "Error input")
                } else {
                    (Color::Yellow, "Add new config url")
                };

                let input = Paragraph::new(input_buffer.as_str())
                    .wrap(Wrap { trim: true }) // To future float
                    .block(
                        Block::default()
                            .title(message)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(color));

                let input_area = ratatui::layout::Rect {
                    x: vertical[0].x,
                    y: vertical[0].y + vertical[0].height - 3,
                    width: vertical[0].width,
                    height: 3,
                };
                f.render_widget(input, input_area);
            }

            //ADDING TUN CONFIG LINE
            if input_mode && tun_mode {
                let (color, message) = if error_input {
                    (Color::Red, "Error input")
                } else {
                    (Color::Blue, "Add new config url with tun arg")
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
                    y: vertical[0].y + vertical[0].height - 3,
                    width: vertical[0].width,
                    height: 3,
                };
                f.render_widget(input, input_area);
            }

            //TRAFFIC BAR
            let max_rate = rx_list
                .iter()
                .chain(tx_list.iter())
                .copied()
                .max()
                .unwrap_or(64 * 1024)
                .max(64 * 1024);

            let current_rate = rx_list
                .back()
                .copied()
                .unwrap_or_default()
                .max(tx_list.back().copied().unwrap_or_default());

            let title = if current_rate >= 1024 * 1024 {
                format!(
                    "Traffic ({:.1}) MB/s",
                    current_rate as f64 / 1024.0 / 1024.0
                )
            } else {
                format!("Traffic ({:.0}) KB/s", current_rate as f64 / 1024.0)
            };

            let traffic_block = Block::default()
                .title(title)
                .title(Title::from(format!("{}: {}", iface, ip_base)).position(Position::Bottom))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let traffic_inner = traffic_block.inner(vertical[1]);
            f.render_widget(traffic_block, vertical[1]);

            if traffic_inner.width > 3 && traffic_inner.height > 4 {
                let traffic_x = traffic_inner.x;
                let traffic_width = traffic_inner.width;
                let traffic_height = traffic_inner.height as usize;
                let rx_rows = (traffic_height / 2).max(1);
                let tx_rows = traffic_height.saturating_sub(rx_rows).max(1);

                let rx_base_y = traffic_inner.y + rx_rows as u16 - 1;
                let tx_base_y = traffic_inner.y + rx_rows as u16;

                let rx_points: Vec<u64> = rx_list
                    .iter()
                    .rev()
                    .take(traffic_width as usize)
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();

                let tx_points: Vec<u64> = tx_list
                    .iter()
                    .rev()
                    .take(traffic_width as usize)
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();

                let history_len = rx_points.len().max(tx_points.len()) as u16;

                for id in 0..history_len {
                    let x = traffic_x + id;
                    let rx_cell = f
                        .buffer_mut()
                        .cell_mut((x, rx_base_y))
                        .expect("base traffic rx");
                    rx_cell.set_char('⣿');
                    rx_cell.set_style(Style::default().fg(Color::Cyan));

                    let tx_cell = f
                        .buffer_mut()
                        .cell_mut((x, tx_base_y))
                        .expect("base traffic tx");
                    tx_cell.set_char('⣿');
                    tx_cell.set_style(Style::default().fg(Color::Magenta));
                }

                for (id, value) in rx_points.iter().enumerate() {
                    let x = traffic_x + id as u16;
                    let level = (((*value as f64 / max_rate.max(1) as f64)
                        * (rx_rows.saturating_sub(1)) as f64)
                        .round() as usize)
                        .min(rx_rows.saturating_sub(1));
                    let y = traffic_inner.y + (rx_rows.saturating_sub(1) - level) as u16;

                    let start = y.min(rx_base_y);
                    let end = y.max(rx_base_y);

                    for yy in start..=end {
                        let cell = f.buffer_mut().cell_mut((x, yy)).expect("traffic rx cell");
                        cell.set_char('⣿');
                        cell.set_style(Style::default().fg(Color::Cyan));
                    }
                }

                let tx_y = traffic_inner.y + rx_rows as u16;
                for (id, value) in tx_points.iter().enumerate() {
                    let x = traffic_x + id as u16;
                    let level = (((*value as f64 / max_rate.max(1) as f64)
                        * (tx_rows.saturating_sub(1)) as f64)
                        .round() as usize)
                        .min(tx_rows.saturating_sub(1));
                    let y: u16 = tx_y + level as u16;

                    let start = y.min(tx_base_y);
                    let end = y.max(tx_base_y);

                    for yy in start..=end {
                        let cell = f.buffer_mut().cell_mut((x, yy)).expect("traffic tx cell");
                        cell.set_char('⣿');
                        cell.set_style(Style::default().fg(Color::Magenta));
                    }
                }
            }

            // HELP PANEL
            let helper = Paragraph::new(Line::from(
                "↑/↓ navigate   q exit   a adding config  A adding tun config d delete config",
            ))
            .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(helper, root[1]);

            // LOG/SETTINGS PANEL
            if !settings_panel {
                let logs = app.get_logs();
                let log_items: Vec<ListItem> =
                    logs.iter().map(|l| ListItem::new(l.clone())).collect();
                let log_list = List::new(log_items).block(
                    Block::default()
                        .title(Line::from("Logs"))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );

                f.render_widget(log_list, horizontal[1]);
                app.read_logs();
            } else {
                let action_text = rule_action.as_deref().unwrap_or("empty");
                let type_text = rule_type.as_deref().unwrap_or("empty");
                let value_text = rule_value.as_deref().unwrap_or("empty");
                let type_dns_text = type_dns_action.as_deref().unwrap_or("empty");
                let value1_text = dns_value1.as_deref().unwrap_or("empty");
                let value2_text = dns_value2.as_deref().unwrap_or("empty");

                let action_style = if transit && settings_selected == 0 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let type_style = if transit && settings_selected == 1 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let value_style = if transit && settings_selected == 2 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let Dns_type_style = if transit && settings_selected == 3 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let Dns_value1_style = if transit && settings_selected == 4 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let Dns_value2_style = if transit && settings_selected == 5 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };
                let enter_style = if transit && settings_selected == 6 {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                };

                let settings = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![Span::raw("         "), Span::raw("Routing Rules")]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Action: ", action_style),
                        Span::styled(action_text, action_style),
                        Span::raw("   "),
                        Span::styled("Type: ", type_style),
                        Span::styled(type_text, type_style),
                        Span::raw("   "),
                        Span::styled("Value: ", value_style),
                        Span::styled(value_text, value_style),
                    ]),
                    Line::from(""),
                    Line::from(vec![Span::raw("         "), Span::raw("DNS Servers")]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Type: ", Dns_type_style),
                        Span::styled(type_dns_text, Dns_type_style),
                        Span::raw("             "),
                        Span::styled("Value 1: ", Dns_value1_style),
                        Span::styled(value1_text, Dns_value1_style),
                        Span::raw("             "),
                        Span::styled("Value 2: ", Dns_value2_style),
                        Span::styled(value2_text, Dns_value2_style),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("                 "),
                        Span::styled("[ENTER]", enter_style),
                    ]),
                ])
                .block(
                    Block::default()
                        .title("Settings")
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
                    x: horizontal[1].x + 4,
                    y: horizontal[1].y + 2,
                    width: horizontal[1].width.saturating_sub(8),
                    height: 7,
                };

                f.render_widget(Clear, context_panel_area);

                let context_items: Vec<ListItem> = if settings_selected == 0 {
                    let mut items: Vec<ListItem> =
                        vec!["r", "h", "s"].into_iter().map(ListItem::new).collect();
                    if custom {
                        items.push(ListItem::new(format!("Input: {}", input_buffer)));
                    } else {
                        items.push(ListItem::new("personal"));
                    }

                    items
                } else if settings_selected == 1 {
                    vec![
                        "inbound",
                        "ip version",
                        "auth user",
                        "protocol",
                        "client",
                        "domain",
                        "domain suffix",
                        "domain keyword",
                        "domain regex",
                        "geosite",
                        "source geoip",
                        "geoip",
                        "source ip cidr",
                        "ip is private",
                        "ip cidr",
                        "ip is private",
                        "source port",
                        "range",
                        "port",
                        "range",
                        "process name",
                        "process path",
                        "regex",
                        "package name",
                        "user",
                        "user id",
                        "clash mode",
                        "network type",
                        "network",
                        "is expensive",
                        "constrained",
                    ]
                    .into_iter()
                    .map(ListItem::new)
                    .collect()
                } else {
                    vec!["No items!"].into_iter().map(ListItem::new).collect()
                };

                let mut state = ListState::default();
                state.select(Some(popup_selected));

                let list = List::new(context_items)
                    .block(
                        Block::default()
                            .title("Select")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_style(
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");
                f.render_stateful_widget(list, context_panel_area, &mut state);
            }

            if value_input {
                let input = Paragraph::new(input_buffer.as_str())
                    .block(
                        Block::default()
                            .title("Enter value")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Green));

                let area = ratatui::layout::Rect {
                    x: horizontal[1].x + 2,
                    y: horizontal[1].y + 4,
                    width: horizontal[1].width.saturating_sub(4),
                    height: 3,
                };

                f.render_widget(Clear, area);
                f.render_widget(input, area);
            }
        })?;
    }
    Ok(())
}
