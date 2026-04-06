use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
};

use super::spinner;
use super::widgets::{format_cost, format_tokens};
use crate::tui::app::App;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .title(Span::styled(
            " Leaderboard ",
            Style::default()
                .fg(app.theme.accent)
                .add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(app.theme.background));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_height = inner.height.saturating_sub(1) as usize;
    app.max_visible_items = visible_height;

    let is_narrow = app.is_narrow();
    let is_very_narrow = app.is_very_narrow();
    let scroll_offset = *app.leaderboard_scroll.borrow();
    let theme_accent = app.theme.accent;
    let theme_muted = app.theme.muted;
    let theme_selection = app.theme.selection;

    let loading = *app.leaderboard_loading.borrow();

    if loading {
        let center = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(inner)[1];

        let mut spans = spinner::get_scanner_spans(app.spinner_frame);
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            "Fetching leaderboard...",
            Style::default().fg(theme_muted),
        ));

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).alignment(Alignment::Center);
        frame.render_widget(paragraph, center);
        return;
    }

    let data = app.leaderboard_data.borrow();
    let data_ref = match data.as_ref() {
        Some(d) => d,
        None => {
            let empty_msg = Paragraph::new("Press [r] to fetch the leaderboard")
                .style(Style::default().fg(theme_muted))
                .alignment(Alignment::Center);
            frame.render_widget(empty_msg, inner);
            return;
        }
    };

    // Check for error
    if let Some(error) = data_ref.get("error") {
        if let Some(msg) = error.as_str() {
            let error_para = Paragraph::new(format!("Error: {}", msg))
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            frame.render_widget(error_para, inner);
            return;
        }
    }

    let users = match data_ref.get("users").and_then(|u| u.as_array()) {
        Some(u) => u,
        None => {
            let empty_msg = Paragraph::new("No leaderboard data available")
                .style(Style::default().fg(theme_muted))
                .alignment(Alignment::Center);
            frame.render_widget(empty_msg, inner);
            return;
        }
    };

    if users.is_empty() {
        let empty_msg = Paragraph::new("Leaderboard is empty")
            .style(Style::default().fg(theme_muted))
            .alignment(Alignment::Center);
        frame.render_widget(empty_msg, inner);
        return;
    }

    let header_cells = if is_very_narrow {
        vec!["#", "User", "Tokens"]
    } else if is_narrow {
        vec!["#", "User", "Tokens", "Cost"]
    } else {
        vec!["#", "Username", "Tokens", "Cost", "Subs"]
    };

    let header = Row::new(
        header_cells
            .iter()
            .map(|h| {
                Cell::from(*h)
            })
            .collect::<Vec<_>>(),
    )
    .style(
        Style::default()
            .fg(theme_accent)
            .add_modifier(Modifier::BOLD),
    )
    .height(1);

    let users_len = users.len();
    let start = scroll_offset.min(users_len.saturating_sub(1));
    let end = (start + visible_height).min(users_len);

    if start >= users_len {
        return;
    }

    let rows: Vec<Row> = users[start..end]
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let idx = i + start;
            let is_selected = idx == app.selected_index;
            let is_striped = idx % 2 == 1;

            let rank = user
                .get("rank")
                .and_then(|r| r.as_u64())
                .unwrap_or((idx + 1) as u64);
            let username = user
                .get("username")
                .and_then(|u| u.as_str())
                .unwrap_or("?");
            let tokens = user
                .get("totalTokens")
                .and_then(|t| t.as_u64())
                .unwrap_or(0);
            let cost = user
                .get("totalCost")
                .and_then(|c| c.as_f64())
                .unwrap_or(0.0);
            let submissions = user
                .get("submissionCount")
                .and_then(|s| s.as_u64())
                .unwrap_or(0);

            let cells: Vec<Cell> = if is_very_narrow {
                vec![
                    Cell::from(format!("{}", rank)).style(Style::default().fg(theme_muted)),
                    Cell::from(truncate(username, 16)).style(
                        Style::default()
                            .fg(app.theme.foreground)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from(format_tokens(tokens)),
                ]
            } else if is_narrow {
                vec![
                    Cell::from(format!("{}", rank)).style(Style::default().fg(theme_muted)),
                    Cell::from(truncate(username, 18)).style(
                        Style::default()
                            .fg(app.theme.foreground)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from(format_tokens(tokens)),
                    Cell::from(format_cost(cost)).style(Style::default().fg(Color::Green)),
                ]
            } else {
                vec![
                    Cell::from(format!("{}", rank)).style(Style::default().fg(theme_muted)),
                    Cell::from(truncate(username, 22)).style(
                        Style::default()
                            .fg(app.theme.foreground)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Cell::from(format_tokens(tokens)),
                    Cell::from(format_cost(cost)).style(Style::default().fg(Color::Green)),
                    Cell::from(submissions.to_string())
                        .style(Style::default().fg(theme_muted)),
                ]
            };

            let row_style = if is_selected {
                Style::default().bg(theme_selection)
            } else if is_striped {
                Style::default().bg(Color::Rgb(20, 24, 30))
            } else {
                Style::default()
            };

            Row::new(cells).style(row_style).height(1)
        })
        .collect();

    let widths = if is_very_narrow {
        vec![
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Min(10),
        ]
    } else if is_narrow {
        vec![
            Constraint::Length(3),
            Constraint::Min(16),
            Constraint::Length(10),
            Constraint::Length(10),
        ]
    } else {
        vec![
            Constraint::Length(3),
            Constraint::Min(16),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(6),
        ]
    };

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().bg(theme_selection));

    frame.render_widget(table, inner);

    if users_len > visible_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let mut scrollbar_state = ScrollbarState::new(users_len).position(scroll_offset);

        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut scrollbar_state,
        );
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let char_count = s.chars().count();
    if char_count <= max_chars {
        s.to_string()
    } else if max_chars <= 3 {
        s.chars().take(max_chars).collect()
    } else {
        let head: String = s.chars().take(max_chars - 3).collect();
        format!("{}...", head)
    }
}
