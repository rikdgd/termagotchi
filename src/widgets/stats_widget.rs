use ratatui::{prelude::*, widgets::*};
use crate::friend::Friend;
use crate::utils::Stat;
use ratatui::widgets::canvas::Shape;

pub fn stats_widget<T>(area: &Rect, friend: &Friend<T>) -> [(Gauge<'static>, Rect); 4] 
where T: Shape
{
    let layout = layout(area);
    
    [
        (stat_gauge(friend.food(), "Food", Color::Red), layout[0]),
        (stat_gauge(friend.joy(),"Joy", Color::Yellow), layout[1]),
        (stat_gauge(friend.energy(), "energy", Color::Blue), layout[2]),
        (stat_gauge(friend.waste_level(), "Poop", Color::Magenta), layout[3]),
    ]
}

fn layout(area: &Rect) -> [Rect; 4] {
    Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
        .areas(*area)
}

fn stat_gauge(stat: &Stat, title: &'static str, color: Color) -> Gauge<'static> {
    Gauge::default()
        .block(Block::new().title(title))
        .gauge_style(
            Style::default()
                .fg(color)
                // .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(stat.value() as u16)
}
