use ratatui::prelude::Color;

pub trait PixelImage {
    /// Returns a vector with each pixel that should be colored for this pixel image.
    fn pixels(&self) -> Vec<Pixel>;
    
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}
