use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{
        BarChart, Block, BorderType, Borders, Gauge, LineGauge, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};

use super::App;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let size = f.size();
    let root_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size)[0];

    let main_layout_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(root_chunk);
    let title_chunk = main_layout_chunk[0];
    let body_chunk = main_layout_chunk[1];

    let main_title = draw_title("TTYPER");
    f.render_widget(main_title, title_chunk);
    render_body(f, body_chunk, app);
}

fn draw_title<'a>(title: &str) -> Paragraph<'a> {
    Paragraph::new(String::from(title))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).bg(Color::Green))
                .border_type(BorderType::Rounded),
        )
}

fn render_body<B>(f: &mut Frame<B>, chunk: Rect, app: &mut App)
where
    B: Backend,
{
    // render_pomodoro(f, chunk, app);
    let body_constraints = match app.mode {
        super::InputMode::Editing => [Constraint::Percentage(30), Constraint::Percentage(70)],
        _ => [Constraint::Percentage(0), Constraint::Percentage(100)],
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(body_constraints.as_ref())
        .split(chunk);

    let input_paragraph = Paragraph::new(&*app.input)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Double)
                .title("Input"),
        );
    f.render_widget(input_paragraph, chunks[0]);

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(_, m)| {
            let content = Span::from(Span::raw(format!("{}", m)));
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(messages, chunks[1], &mut app.state);
}

fn render_pomodoro<B>(f: &mut Frame<B>, chunk: Rect, app: &mut App)
where
    B: Backend,
{
    let gauge = BarChart::default()
        .block(Block::default().title("BarChart").borders(Borders::ALL))
        .bar_width(3)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
        .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::White))
        .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
        .max(4);
    f.render_widget(gauge, chunk);
}
