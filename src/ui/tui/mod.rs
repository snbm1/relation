use std::{
    io,
    time::{Duration, Instant},
};

use crate::App;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        BarChart, Block, Borders, BorderType,
        List, ListItem, ListState,
        Paragraph,
    },
    Terminal,
};

#[derive(Clone, Copy)]
struct Counters {
    rx: u64,
    tx: u64,
}

fn read_iface(iface: &str) -> io::Result<Counters> {
    let text = std::fs::read_to_string("/proc/net/dev")?;

    for line in text.lines().skip(2) {
        if let Some((name, rest)) = line.split_once(':') {
            if name.trim() == iface {
                let cols: Vec<&str> = rest.split_whitespace().collect();
                let rx = cols.get(0).unwrap_or(&"0").parse().unwrap_or(0);
                let tx = cols.get(8).unwrap_or(&"0").parse().unwrap_or(0);
                return Ok(Counters { rx, tx });
            }
        }
    }

    Ok(Counters { rx: 0, tx: 0 })
}

pub fn run(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let iface = "wlp2s0";

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut prev = read_iface(iface)?;
    let mut prev_time = Instant::now();

    let mut rx_rate: u64 = 0;
    let mut tx_rate: u64 = 0;

    let mut selected_index: usize = 0;

    loop {
        // -------- INPUT --------
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Down => {
                        if selected_index + 1 < app.get_list().len() {
                            selected_index += 1;
                        }
                    }

                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }

                    _ => {}
                }
            }
        }

        // -------- UPDATE TRAFFIC --------
        if prev_time.elapsed() >= Duration::from_millis(500) {
            let now = Instant::now();
            let dt = (now - prev_time).as_secs_f64().max(0.001);

            let current = read_iface(iface)?;

            let drx = current.rx.saturating_sub(prev.rx) as f64;
            let dtx = current.tx.saturating_sub(prev.tx) as f64;

            rx_rate = (drx / dt) as u64;
            tx_rate = (dtx / dt) as u64;

            prev = current;
            prev_time = now;
            
        }

        // -------- DRAW --------
        terminal.draw(|f| {
            let size = f.area();

            // Горизонтальное деление
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(120),
                ])
                .split(size);

            // Вертикальное деление слева
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(20),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(horizontal[0]);

            // ===== CONFIG LIST =====
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

            // ===== TRAFFIC =====
            let chart = BarChart::default()
                .block(
                    Block::default()
                        .title("Traffic (KB/s)")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .data(&[
                    ("RX", rx_rate / 1024),
                    ("TX", tx_rate / 1024),
                ])
                .bar_width(8)
                .bar_gap(4)
                .max(1000);

            f.render_widget(chart, vertical[1]);

            // ===== HELP =====
            let helper = Paragraph::new(Line::from("↑/↓ navigate   q exit"));
            f.render_widget(helper, vertical[2]);

            // ===== RIGHT PANEL =====
            let right_panel = Block::default()
                .title("Output")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            f.render_widget(right_panel, horizontal[1]);
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}