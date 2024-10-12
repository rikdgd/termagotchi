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
}
