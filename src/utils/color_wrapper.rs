use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use rand::prelude::*;


const NUM_COLORS: u32 = 7;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    
    pub fn new_random(use_black_white: bool) -> Self {
        let mut rng = thread_rng();
        if use_black_white {
            match rng.gen_range(0..NUM_COLORS) {
                0 => ColorWrapper::Cyan,
                1 => ColorWrapper::LightMagenta,
                2 => ColorWrapper::Red,
                3 => ColorWrapper::Green,
                4 => ColorWrapper::Blue,
                5 => ColorWrapper::Black,
                _ => ColorWrapper::White,
            }
        } else {
            match rng.gen_range(0..NUM_COLORS - 2) {
                0 => ColorWrapper::Cyan,
                1 => ColorWrapper::LightMagenta,
                2 => ColorWrapper::Red,
                3 => ColorWrapper::Green,
                _ => ColorWrapper::Blue,
            }
        }
    }
}
