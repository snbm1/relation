mod consts;
mod ifaces;
mod minireq;
mod tuiguard;
mod render_traffic;
mod setup;
mod state;
mod input;


use input::{
    handle_add_config_input,
    handle_normal_input,
    handle_route_value_input,
};

use state::{InputAction, InputMode, TuiState};

use setup::setup_tty;

use render_traffic::render_traffic_bar;

use consts::*;
use ifaces::*;
use minireq::*;
use std::{
    io,
    os::{fd::{AsRawFd, FromRawFd}},
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
                match state.input.mode {
                    InputMode::AddConfig { tun } => {
                        handle_add_config_input(app, &mut state, key.code, tun)?;
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
            if matches!(state.input.mode, InputMode::AddConfig { tun: false }) {
                let (color, message) = if state.input.error {
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
            if matches!(state.input.mode, InputMode::AddConfig { tun: true }) {
                let (color, message) = if state.input.error {
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
            if state.ui.context_menu && state.ui.settings_selected != 6 {
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

            if state.input.mode == InputMode::RouteValue {
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
