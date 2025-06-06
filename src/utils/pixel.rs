use ratatui::prelude::Color;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}
