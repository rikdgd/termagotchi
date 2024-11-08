pub trait Movement {
    fn next_position(&mut self) -> Location;
    fn frame_count(&self) -> u32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}
