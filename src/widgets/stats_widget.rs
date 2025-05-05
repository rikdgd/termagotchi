use ratatui::{
    layout::Direction,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
};
use crate::friend::Friend;
use crate::utils::Stat;

/// Manages the state of the *"Stats widget"* and allows for generating the widget so it can be 
/// rendered in the TUI.
/// ## Fields:
/// * `friend` - A reference to the current Termagotchi pet, used to get the stats of the pet.
/// * `bars` - The `ratatui::widgets::Bar` items that should be displayed in the widget, each corresponding
/// a specific creature stat.
pub struct StatsWidgetManager<'a> {
    friend: &'a Friend,
    bars: [Bar<'a>; 4],
}
impl<'a> StatsWidgetManager<'a> {
    pub fn new(friend: &'a Friend) -> Self {
        Self {
            friend,
            bars: Self::generate_stat_bars(friend),
        }
    }

    fn generate_stat_bars(friend: &Friend) -> [Bar<'a>; 4] {
        [
            Self::generate_stat_bar("Food", *friend.food()),
            Self::generate_stat_bar("Joy", *friend.joy()),
            Self::generate_stat_bar("Energy", *friend.energy()),
            Self::generate_stat_bar("Health", *friend.health()),
        ]
    }
    
    /// Returns the widget that can be rendered in the TUI with all the stats of the creature displayed.
    pub fn get_widget(&self) -> BarChart {
        let title = Line::from("Stats").centered();
        BarChart::default()
            .block(Block::bordered().title(title))
            .data(BarGroup::default().bars(&self.bars))
            .bar_width(1)
            .bar_gap(0)
            .direction(Direction::Horizontal)
    }

    fn generate_stat_bar(stat_name: &str, stat: Stat) -> Bar {
        // Using the stat.value() as a u8 here is safe, since the stats value can at max be 100.
        let style = Self::stat_style(stat);
        Bar::default()
            .value(u64::from(stat.value()))
            // .label(Line::from("bar_label"))
            .text_value(stat_name.to_string())
            .style(style)
            .value_style(style.reversed())
    }

    /// create a yellow to red value based on the value (50-90)
    fn stat_style(stat: Stat) -> Style {
        let green = (stat.value() * 2) as u8;
        let red = 200 - green;
        let color = Color::Rgb(red, green, 0);
        Style::new().fg(color)
    }
}