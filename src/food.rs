use rand::prelude::*;

const FOOD_COUNT: u8 = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Food {
    Soup,
    Cookie,
    Burger,
}

impl Food {
    pub fn points(&self) -> u32 {
        match self {
            Food::Soup => 20,
            Food::Cookie => 30,
            Food::Burger => 40,
        }
    }
    
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..FOOD_COUNT) {
            0 => Food::Soup,
            1 => Food::Cookie,
            _ => Food::Burger,
        }
    }
}
