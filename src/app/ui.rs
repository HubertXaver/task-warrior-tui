use std::borrow::{Borrow, BorrowMut};

use chrono::Datelike;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use super::App;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .horizontal_margin(10)
        .vertical_margin(2)
        .split(f.size());

    let (table_chunk, command_chunk, message_chunk) = (main_chunk[0], main_chunk[1], main_chunk[2]);
    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Max(3 * 7)].as_ref())
        .vertical_margin(2)
        .split(table_chunk);
    let (habit_chunk, values_chunk) = (table_chunks[0], table_chunks[1]);

    let cell_normal_style = Style::default().bg(Color::Black).fg(Color::White);
    let cell_selected_style = Style::default().fg(Color::Black).bg(Color::White);
    let normal_style = Style::default().bg(Color::Blue);
    let header_labels = app.tracker.get_header_labels();
    let header_cells = header_labels.iter().map(|h| {
        return Cell::from(h.to_owned()).style(Style::default());
    });
    let header = Row::new(header_cells).height(1);
    let column_constraint = Constraint::Length(3);
    let column_width = &[column_constraint; 7];
    let values = app.tracker.values();
    let value_rows = values.iter().enumerate().map(|(i, row)| {
        let cells = row.iter().enumerate().map(|(j, r)| {
            let (a, b) = match app.state.selected() {
                Some((x, y)) => (x, y),
                None => (0, 0),
            };

            let mut cell_style = cell_normal_style;
            if (i, j) == (a, b) {
                cell_style = cell_selected_style
            }

            let text = match *r {
                true => " • ",
                false => "",
            };

            Cell::from(text).style(cell_style)
        });
        Row::new(cells)
    });
    let values = Table::new(value_rows).header(header).widths(column_width);
    f.render_widget(values, values_chunk);

    let labels = app.tracker.labels();

    let habit_rows = labels.iter().map(|h| {
        let cell = Cell::from(h.as_str()).style(Style::default().fg(Color::Green));
        Row::new([cell])
    });
    let habit_table = Table::new(habit_rows)
        .header(Row::new([Cell::from("Habits")]))
        .widths([Constraint::Length(10)].as_ref());

    f.render_widget(habit_table, habit_chunk);

    let command_bg = Block::default().style(Style::default().bg(Color::Black));

    let command = Paragraph::new(Text::from([":".to_owned(), app.input.to_owned()].join(" ")))
        .alignment(Alignment::Left)
        .block(command_bg);
    f.render_widget(command, command_chunk);

    let mode = match app.mode {
        super::AppMode::NORMAL => "NORMAL Mode",
        super::AppMode::COMMAND => "COMMAND Mode",
    };

    let text = Paragraph::new(Text::from(
        [mode.to_owned(), "'q' to quit".to_owned()].join(" | "),
    ))
    .alignment(Alignment::Center);
    f.render_widget(text, message_chunk)
}
