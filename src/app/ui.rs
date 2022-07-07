use std::borrow::{Borrow, BorrowMut};

use chrono::{Date, Datelike, Utc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use super::App;

const TABLE_WIDTH: u16 = 39;
const TABLE_HEIGHT: u16 = 19;
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn split_area<B>(f: &mut Frame<B>) -> Rect
where
    B: Backend,
{
    let hor_constraints = [
        Constraint::Ratio(1, 3),
        Constraint::Length(TABLE_WIDTH),
        Constraint::Ratio(1, 3),
    ];
    let ver_constraints = [
        Constraint::Ratio(1, 3),
        Constraint::Length(TABLE_HEIGHT),
        Constraint::Ratio(1, 3),
    ];
    let sub = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(hor_constraints.clone().as_ref())
        .split(f.size());
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(ver_constraints.clone().as_ref())
        .split(sub[1]);
    return main[1];
}

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let layout = split_area(f);
    let bg_block = Block::default()
        .title("Rabit, the Habit Tracker")
        .title_alignment(Alignment::Center);
    f.render_widget(bg_block, layout);
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(12), // Main Table
                Constraint::Length(1),  // Command Bar
                Constraint::Length(1),  // Help Bar
            ]
            .as_ref(),
        )
        .horizontal_margin(4)
        .vertical_margin(2)
        .split(layout);

    let (top_chunk, cl_chunk, help_chunk) = (main_chunk[0], main_chunk[1], main_chunk[2]);

    let inner_table_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(TABLE_HEIGHT - 2)])
        .split(top_chunk);

    let (heading_chunk, table_chunk) = (inner_table_chunk[0], inner_table_chunk[1]);

    let month = MONTHS[app.tracker.start_date.month() as usize - 1].to_owned();

    let title = Paragraph::new(Text::from(month)).alignment(Alignment::Left);
    f.render_widget(title, heading_chunk);

    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Max(3 * 7)].as_ref())
        .split(table_chunk);
    let (label_chunk, values_chunk) = (table_chunks[0], table_chunks[1]);
    // Cell Styles
    let cell_normal_style = Style::default().fg(Color::White);
    let cell_selected_style = Style::default().fg(Color::Black).bg(Color::White);
    let cell_disabled_style = Style::default().bg(Color::DarkGray);
    let header_labels = app.tracker.get_header_labels();
    let header_cells = header_labels.iter().map(|h| {
        return Cell::from(h.to_owned()).style(Style::default());
    });
    let header = Row::new(header_cells).height(1);
    let column_constraint = Constraint::Length(3);
    let column_width = &[column_constraint; 7];

    // All the tracked data
    let values = app.tracker.values();
    let value_rows = values.iter().enumerate().map(|(i, row)| {
        let cells = row.iter().enumerate().map(|(j, r)| {
            let (a, b) = match app.state.selected() {
                Some((x, y)) => (x, y),
                None => (0, 0),
            };

            let mut cell_style = cell_normal_style;
            let now = Utc::now().day().to_string();
            if (i, j) == (a, b) {
                if header_labels[j] == now {
                    cell_style = cell_selected_style
                } else {
                    cell_style = cell_disabled_style
                }
            } else if header_labels[j] == now {
                cell_style = cell_disabled_style
            }

            let text = match *r {
                true => " • ",
                false => "",
            };

            Cell::from(text).style(cell_style)
        });
        Row::new(cells)
    });

    // The table with the boolean values
    let values_table = Table::new(value_rows)
        .header(header)
        .widths(column_width)
        .column_spacing(0);
    f.render_widget(values_table, values_chunk);

    let labels = app.tracker.labels();

    let habit_rows = labels.iter().map(|h| {
        let cell = Cell::from(h.as_str()).style(Style::default().fg(Color::Green));
        Row::new([cell])
    });
    let labels_table = Table::new(habit_rows)
        .header(Row::new([Cell::from("Habits")]))
        .widths([Constraint::Length(10)].as_ref());

    f.render_widget(labels_table, label_chunk);

    let command_bg = Block::default().style(Style::default().bg(Color::DarkGray));

    let command = Paragraph::new(Text::from([":".to_owned(), app.input.to_owned()].join(" ")))
        .alignment(Alignment::Left)
        .block(command_bg);
    f.render_widget(command, cl_chunk);

    let mode = match app.mode {
        super::AppMode::NORMAL => "NORMAL Mode",
        super::AppMode::COMMAND => "COMMAND Mode",
    };

    let text = Paragraph::new(Text::from(
        [mode.to_owned(), "'q' to quit".to_owned()].join(" | "),
    ))
    .alignment(Alignment::Center);
    f.render_widget(text, help_chunk)
}
