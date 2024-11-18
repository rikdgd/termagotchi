use rand::prelude::*;

const FOOD_COUNT: u8 = 3;

pub enum Food {
    Soup,
    Fries,
    Burger,
}

impl Food {
    pub fn points(&self) -> u32 {
        match self {
            Food::Soup => 10,
            Food::Fries => 15,
            Food::Burger => 25,
        }
    }
    
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..FOOD_COUNT) {
            0 => Food::Soup,
            1 => Food::Fries,
            _ => Food::Burger,
        }
    }
}
