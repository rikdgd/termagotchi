use ratatui::{
    style::Color,
    layout::Rect,
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block,
        Widget
    },
};
use crate::friend::Friend;

pub struct StatsWidget<'a> {
    friend: &'a Friend,
    area: Rect,
}
impl<'a> StatsWidget<'a> {
    pub fn new(friend: &'a Friend) -> Self {
        Self { 
            friend,
            area: Rect::new(0, 0, 20, 100),
        }
    }
    
    pub fn get_widget(&self) -> impl Widget + 'a {
        let canvas = Canvas::default()
            .block(Block::bordered().title("Stats"))
            .x_bounds([0.0, 20.0])  // TODO: Get based of area
            .y_bounds([0.0, 80.0])
            .paint(|ctx|{
                ctx.draw(&Line {
                    x1: 0.0,
                    y1: 10.0,
                    x2: 10.0,
                    y2: 10.0,
                    color: Color::White,
                });

            });

        canvas
    }
}