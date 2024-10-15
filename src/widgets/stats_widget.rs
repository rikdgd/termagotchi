use ratatui::{prelude::*, widgets::*};
use crate::friend::Friend;
use crate::utils::Stat;
use ratatui::widgets::canvas::Shape;

pub fn stats_widget<T>(area: &Rect, friend: &Friend<T>) -> [(Gauge<'static>, Rect); 4] 
where T: Shape
{
    let layout = layout(area);
    
    [
        (food_gauge(friend.food()), layout[0]),
        (food_gauge(friend.joy()), layout[1]),
        (food_gauge(friend.energy()), layout[2]),
        (food_gauge(friend.waste_level()), layout[3]),
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

fn food_gauge(food_stat: &Stat) -> Gauge<'static> {
    Gauge::default()
        .block(Block::new().title("Food"))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(food_stat.value() as u16)
}
