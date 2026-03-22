mod ifaces;
use clap::builder::styling::Style as ClapStyle;
use crossterm::event::KeyModifiers;
use ifaces::*;
use ratatui::widgets::block::{Title, Position};
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::{
    io,
    time::{Duration, Instant},
};

use crate::App;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use std::collections::VecDeque;

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};

pub fn run(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let iface = iface_detect();

    enable_raw_mode()?;

    // 1) Входим в alternate screen через обычный stdout (fd=1)
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // 2) Дублируем fd=1 (TTY) — это будет “канал” для UI
    let ui_fd = unsafe { libc::dup(stdout.as_raw_fd()) };
    if ui_fd < 0 {
        return Err("dup(stdout) failed".into());
    }

    // 3) Глушим fd=1 и fd=2, чтобы Go/bridge больше не мог печатать в терминал
    let null = OpenOptions::new().write(true).open("/dev/null")?;
    unsafe {
        libc::dup2(null.as_raw_fd(), libc::STDOUT_FILENO);
        libc::dup2(null.as_raw_fd(), libc::STDERR_FILENO);
    }

    // 4) Создаём writer из сохранённого fd для ratatui
    let ui_out = unsafe { std::fs::File::from_raw_fd(ui_fd) };
    let backend = CrosstermBackend::new(ui_out);
    let mut terminal = Terminal::new(backend)?;

    let old_log = app.get_data_path().join("box.log");
    let _ = std::fs::write(&old_log, "");

    let mut prev = read_iface(&iface)?;
    let mut prev_time = Instant::now();

    let mut rx_rate: u64 = 0;
    let mut tx_rate: u64 = 0;

    let mut rx_list: VecDeque<u64> = VecDeque::new();
    let mut tx_list: VecDeque<u64> = VecDeque::new();

    let mut selected_index: usize = 0;
    let mut len = app.get_len();

    let mut enter_mode: bool = false;
    let mut input_mode = false;
    let mut tun_mode: bool = false;
    let mut error_input = false;
    let mut running: Option<String> = None;

    let mut input_buffer = String::new();

    loop {
        // -------- INPUT --------
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if input_mode {
                    match key.code {
                        KeyCode::Esc => {
                            input_mode = false;
                            if tun_mode {tun_mode = false;}
                            error_input = false;
                            input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !input_buffer.is_empty() {
                                let cfg = app.handler_mut().clean(); 
                                let result = if tun_mode {
                                    cfg.default_tun().set_outbound_from_url(&input_buffer.clone())
                                }
                                else {
                                    cfg.default().set_outbound_from_url(&input_buffer.clone())
                                }; 
                    
                                match result {
                                    Ok(_) => {
                                        if let Err(err) = app.add_config(None) {
                                            error_input = true;
                                        } else {
                                            error_input = false;
                                            input_buffer.clear();
                                            len = app.get_len();
                                            input_mode = false;
                                            if tun_mode {tun_mode = false;}
                                            selected_index = 0;
                                        }
                                    }
                                    Err(err) => {
                                        error_input = true;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            input_buffer.pop();
                            if input_buffer.is_empty(){
                                error_input = false;
                            }
                        }
                        KeyCode::Char(c) => {
                            input_buffer.push(c);
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => {
                            if enter_mode {
                                app.stop_app();
                            }

                            break;
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
                                app.remove_config_by_number(selected_index);

                                if running.as_deref() == Some(name.as_str()) {
                                    app.stop_app();
                                    running = None;
                                    enter_mode = false;
                                }

                                len = app.get_len();

                                if selected_index >= len && len > 0 {
                                    selected_index = len - 1;
                                }
                            }
                        }
                        KeyCode::Enter => {
                            let len = app.get_len();
                            if len > 0 && !enter_mode {
                                let number = selected_index as u16 + 1;
                                running = Some(app.get_list()[selected_index].clone());
                                app.set_log_file();
                                app.run_app(None, Some(number), false);
                                enter_mode = true;
                            } else if enter_mode {
                                let name = app.get_list()[selected_index].clone();
                                if running.as_deref() == Some(name.as_str()) {
                                    running = None;
                                    app.stop_app();
                                    enter_mode = false;
                                } else {
                                    app.stop_app();
                                    std::thread::sleep(Duration::from_millis(100));
                                    let number = selected_index as u16 + 1;
                                    running = Some(name.clone());
                                    app.set_log_file();
                                    app.run_app(None, Some(number), false);
                                }
                            }
                        }

                        KeyCode::Down | KeyCode::Char('j') => {
                            if len > 0 {
                                selected_index = (selected_index + 1) % len;
                            }
                        }

                        KeyCode::Up | KeyCode::Char('k') => {
                            if len > 0 {
                                selected_index = (selected_index + len - 1) % len;
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        if prev_time.elapsed() >= Duration::from_millis(500) {
            let now = Instant::now();
            let dt = (now - prev_time).as_secs_f64().max(0.001);

            let current = read_iface(&iface)?;

            let drx = current.rx.saturating_sub(prev.rx) as f64;
            let dtx = current.tx.saturating_sub(prev.tx) as f64;

            rx_rate = (drx / dt) as u64;
            tx_rate = (dtx / dt) as u64;

            rx_list.push_back(rx_rate);
            tx_list.push_back(tx_rate);
            while rx_list.len() > 800 {
                rx_list.pop_front();
            }
            while tx_list.len() > 800 {
                tx_list.pop_front();
            }

            prev = current;
            prev_time = now;
        }

        terminal.draw(|f| {
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
           let max_rate = rx_list.iter().chain(tx_list.iter()).copied().max().unwrap_or(64 * 1024).max(64 * 1024); 

           let title = if max_rate >= 1024 * 1024 {
                format!("Traffic ({:.1}) MB/s", max_rate as f64 / 1024.0 / 1024.0)
           } else {
                format!("Traffic ({:.0}) KB/s", max_rate as f64 / 1024.0)
           }; 

           let traffic_block = Block::default()
                .title(title)
                .title(Title::from(format!("interface: {} ip: {}", iface_detect(), ip_addr())).position(Position::Bottom))
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

                let rx_points : Vec<u64> = rx_list
                    .iter()
                    .rev()
                    .take(traffic_width as usize)
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect(); 

                let tx_points : Vec<u64> = tx_list
                    .iter()
                    .rev()
                    .take(traffic_width as usize)
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect(); 

                let mut prev_rx_y: Option<u16> = None; 

                for (id, value) in rx_points.iter().enumerate() {
                    let x = traffic_x + id as u16;
                    let level = (((*value as f64 / max_rate.max(1) as f64) * (rx_rows.saturating_sub(1)) as f64).round() as usize).min(rx_rows.saturating_sub(1)); 
                    let y = traffic_inner.y + (rx_rows.saturating_sub(1) - level) as u16; 

                    if let Some(prev_y) = prev_rx_y {
                        let start = prev_y.min(y); 
                        let end = prev_y.max(y);

                        for yy in start..=end {
                            let cell = f.buffer_mut().cell_mut((x, yy)).expect("traffic rx cell"); 
                            cell.set_char('⣿'); 
                            cell.set_style(Style::default().fg(Color::Cyan));
                        } 
                    }

                    let cell = f.buffer_mut().cell_mut((x, y)).expect("traffic rx cell"); 
                    cell.set_char('⣿'); 
                    cell.set_style(Style::default().fg(Color::Cyan));
                    prev_rx_y = Some(y); 
                }
                
                let tx_y = traffic_inner.y + rx_rows as u16; 
                let mut prev_tx_y: Option<u16> = None; 
                for (id, value) in tx_points.iter().enumerate() {
                    let x = traffic_x + id as u16; 
                    let level = (((*value as f64 / max_rate.max(1) as f64) * (tx_rows.saturating_sub(1)) as f64).round() as usize).min(tx_rows.saturating_sub(1)); 
                    let y: u16 = tx_y + level as u16; 

                    if let Some(prev_y) = prev_tx_y {
                        let start = prev_y.min(y); 
                        let end = prev_y.max(1); 

                        for yy in start..=end {
                            let cell = f.buffer_mut().cell_mut((x, yy)).expect("traffic tx cell"); 
                            cell.set_char('⣿'); 
                            cell.set_style(Style::default().fg(Color::Magenta)); 
                        }
                    }

                    let cell = f.buffer_mut().cell_mut((x, y)).expect("traffic tx cell");
                    cell.set_char('⣿'); 
                    cell.set_style(Style::default().fg(Color::Magenta));
                    prev_tx_y = Some(y);
                }
            }

            // HELP PANEL
            let helper = Paragraph::new(Line::from(
                "↑/↓ navigate   q exit   a adding config  A adding tun config d delete config",
            ))
            .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(helper, root[1]);

            // RIGHT PANEL
            let logs = app.get_logs();
            let log_items: Vec<ListItem> = logs.iter().map(|l| ListItem::new(l.clone())).collect();
            let log_list = List::new(log_items).block(
                Block::default()
                    .title(Line::from("Logs").centered())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

            f.render_widget(log_list, horizontal[1]);
        })?;
        app.read_logs();
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
