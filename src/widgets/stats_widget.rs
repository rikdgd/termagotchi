use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
    DefaultTerminal, Frame,
};
use crate::friend::Friend;

pub struct StatsWidgetGenerator<'a> {
    friend: &'a Friend,
}
impl<'a> StatsWidgetGenerator<'a> {
    pub fn new(friend: &'a Friend) -> Self {
        Self {
            friend
        }
    }
    
    pub fn get_widget(&self, friend: &Friend) -> BarChart {
        let title = Line::from("Stats").centered();
        BarChart::default()
            .block(Block::new().title(title))
            .data(BarGroup::default().bars(self.get_stat_bars()))
            .bar_width(1)
            .bar_gap(0)
            .direction(Direction::Horizontal)
    }
    
    fn get_stat_bars(&self) -> &[Bar<'a>] {
        // TODO: Generate the bars based on the creature's stats.
        todo!()
    }

    fn horizontal_bar(stat_name: &str) -> Bar {
        let style = Self::temperature_style(50);
        Bar::default()
            .value(u64::from(50u64))
            .label(Line::from("bar_label"))
            .text_value("text_value".to_string())
            .style(style)
            .value_style(style.reversed())
    }

    /// create a yellow to red value based on the value (50-90)
    fn temperature_style(value: u8) -> Style {
        let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
        let color = Color::Rgb(255, green, 0);
        Style::new().fg(color)
    }
}