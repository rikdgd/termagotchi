use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorWrapper {
    Cyan,
    LightMagenta,
    Red,
    Green,
    Blue,
    White,
    Black,
}

impl ColorWrapper {
    pub fn get_ratatui_color(&self) -> Color {
        match self {
            ColorWrapper::Cyan => Color::Cyan,
            ColorWrapper::LightMagenta => Color::LightMagenta,
            ColorWrapper::Red => Color::Red,
            ColorWrapper::Green => Color::Green,
            ColorWrapper::Blue => Color::Blue,
            ColorWrapper::White => Color::White,
            ColorWrapper::Black => Color::Black,
        }
    }
}
