use ratatui::{prelude::*, widgets::*};

pub fn stats_widget(area: &Rect) -> [(Gauge, Rect); 4] {
    let layout = layout(area);
    
    [
        (food_gauge(), layout[0]),
        (food_gauge(), layout[1]),
        (food_gauge(), layout[2]),
        (food_gauge(), layout[3]),
    ]
}

fn layout(area: &Rect) -> [Rect; 4] {
    Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
        .areas(area.clone())
}

fn food_gauge() -> Gauge<'static> {
    Gauge::default()
        .block(Block::new().title("Food"))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(20)
}
