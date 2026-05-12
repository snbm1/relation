use std::collections::VecDeque;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, block::{Position, Title}},
};

use super::consts::{traffic, text};

pub fn render_traffic_bar(
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

