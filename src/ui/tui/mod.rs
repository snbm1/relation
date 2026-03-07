mod ifaces;
use ifaces::*;
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

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    try_init,
    widgets::{BarChart, Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
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

    let mut selected_index: usize = 0;
    let mut len = app.get_len();

    let mut enter_mode: bool = false;
    let mut input_mode = false;
    let mut input_buffer = String::new();

    loop {
        // -------- INPUT --------
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if input_mode {
                    match key.code {
                        KeyCode::Esc => {
                            input_mode = false;
                            input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !input_buffer.is_empty() {
                                app.handler_mut()
                                    .clean()
                                    .default()
                                    .set_outbound_from_url(&input_buffer.clone());
                                app.add_config(None);
                                input_buffer.clear();
                                len = app.get_len();
                                input_mode = false;
                                selected_index = 0;
                            }
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
                            if enter_mode {
                                app.stop_app();
                            }

                            break;
                        }
                        KeyCode::Char('a') => {
                            input_mode = true;
                        }
                        KeyCode::Char('d') => {
                            if len > 0 {
                                app.remove_config_by_number(selected_index);
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
                                app.set_log_file();
                                app.run_app(None, Some(number), false);
                                enter_mode = true;
                            } else if enter_mode {
                                app.stop_app();
                                enter_mode = false;
                            }
                        }

                        KeyCode::Down | KeyCode::Char('j') => {
                            selected_index = (selected_index + 1) % len;
                        }

                        KeyCode::Up | KeyCode::Char('k') => {
                            selected_index = (selected_index + len - 1) % len;
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
                .constraints([Constraint::Length(50), Constraint::Min(0)])
                .split(root[0]);

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(22), Constraint::Min(0)])
                .split(horizontal[0]);

            let configs = app.get_list();

            let items: Vec<ListItem> = configs
                .iter()
                .map(|name| ListItem::new(name.clone()))
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

            if input_mode {
                let input = Paragraph::new(input_buffer.as_str())
                    .block(
                        Block::default()
                            .title("Add new config url")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Yellow));

                let input_area = ratatui::layout::Rect {
                    x: vertical[0].x,
                    y: vertical[0].y + vertical[0].height - 3,
                    width: vertical[0].width,
                    height: 3,
                };
                f.render_widget(input, input_area);
            }

            //TRAFFIC BAR
            let chart = BarChart::default()
                .block(
                    Block::default()
                        .title("Traffic (KB/s)")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .data(&[("RX", rx_rate / 1024), ("TX", tx_rate / 1024)])
                .bar_width(8)
                .bar_gap(4)
                .max(1000);

            f.render_widget(chart, vertical[1]);

            // HELP PANEL
            let helper = Paragraph::new(Line::from(
                "↑/↓ navigate   q exit   a adding config   d delete config",
            ))
            .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(helper, root[1]);

            // RIGHT PANEL
            let logs = read_logs(app);
            let log_items: Vec<ListItem> = logs.iter().map(|l| ListItem::new(l.clone())).collect();
            let log_list = List::new(log_items).block(
                Block::default()
                    .title(Line::from("Logs").centered())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

            f.render_widget(log_list, horizontal[1]);
        })?;
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
