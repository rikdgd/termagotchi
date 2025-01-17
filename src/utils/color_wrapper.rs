use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use rand::prelude::*;


const NUM_COLORS: u32 = 9;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorWrapper {
    Red,
    Green,
    Blue,
    Cyan,
    LightMagenta,
    Yellow,
    LightRed,
    White,
    Black,
}

impl ColorWrapper {
    pub fn get_ratatui_color(&self) -> Color {
        match self {
            ColorWrapper::Red => Color::Red,
            ColorWrapper::Green => Color::LightGreen,
            ColorWrapper::Blue => Color::LightBlue,
            ColorWrapper::Cyan => Color::Cyan,
            ColorWrapper::LightMagenta => Color::LightMagenta,
            ColorWrapper::Yellow => Color::Yellow,
            ColorWrapper::LightRed => Color::LightRed,
            
            ColorWrapper::White => Color::White,
            ColorWrapper::Black => Color::Black,
        }
    }
    
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..NUM_COLORS - 2) {
            0 => ColorWrapper::Red,
            1 => ColorWrapper::Green,
            2 => ColorWrapper::Blue,
            3 => ColorWrapper::Cyan,
            4 => ColorWrapper::LightMagenta,
            5 => ColorWrapper::Yellow,
            _ => ColorWrapper::LightRed,
        }
    }
}
