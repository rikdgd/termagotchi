use std::cell::RefCell;
use ratatui::widgets::canvas::{Painter, Shape};
use crate::animations::PixelAnimation;
use crate::shapes::{GrowthStageShapes, PixelImage};
use crate::utils::Pixel;

#[derive(Debug, Clone, PartialEq)]
struct EggHopAnimationState {
    egg_shape: GrowthStageShapes,
    is_grounded: bool,
}

pub struct EggHopAnimation {
    state: RefCell<EggHopAnimationState>,
}

impl EggHopAnimation {
    pub fn new(egg_shape: GrowthStageShapes) -> std::io::Result<Self> {
        match egg_shape {
            GrowthStageShapes::Egg(_) => {
                let state = EggHopAnimationState {
                    egg_shape,
                    is_grounded: true,
                };
                
                Ok(Self { state: RefCell::new(state) })
            },
            
            _ => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The provided shape was not of type 'GrowthStageShapes::Egg'"
                ))
            },
        }
    }
}

impl PixelAnimation for EggHopAnimation {
    fn next_frame(&self) -> Vec<Pixel> {
        let mut state = self.state.borrow_mut();
        
        let pixels = state.egg_shape.pixels();
        if !state.is_grounded {
            return pixels;
        }

        let mut updated_pixels = Vec::new();
        for pixel in pixels {
            let mut pixel = pixel;
            pixel.y += 5;
            updated_pixels.push(pixel);
        }
        
        state.is_grounded = !state.is_grounded;
        updated_pixels
    }
}

impl Shape for EggHopAnimation {
    fn draw(&self, painter: &mut Painter) {
        for pixel in self.next_frame() {
            painter.paint(
                pixel.x as usize,
                pixel.y as usize,
                pixel.color
            );
        }
    }
}
