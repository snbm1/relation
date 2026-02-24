use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{BarChart, Block, Borders, Paragraph, BorderType},
    text::Line,
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let iface = "wlp2s0"; // interface

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut prev = read_iface(iface)?;
    let mut prev_time = Instant::now();

    let mut rx_rate: u64 = 0;
    let mut tx_rate: u64 = 0;

    loop {
        // INPUT
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

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

        // DRAW
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(30), 
                    Constraint::Min(0),   
                    Constraint::Length(7)  
                ])
                .split(size);

            let menu = Block::default().title("Configs").borders(Borders::ALL).border_type(BorderType::Rounded);


            f.render_widget(menu, chunks[0]);

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
                .bar_width(10)
                .bar_gap(4)
                .max(1000); 

            f.render_widget(chart, chunks[1]);

            let helper = Paragraph::new(vec![
                Line::from("Help: q - exit s - exit"),
            ]).block(Block::default().borders(Borders::NONE));

            f.render_widget(helper, chunks[2]);
        })?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}