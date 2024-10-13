use crate::food::Food;
use crate::utils::Stat;
use ratatui::widgets::canvas::Shape;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Friend<T>
where
    T: Shape,
{
    name: String,
    hunger: Stat,
    joy: Stat,
    energy: Stat,
    waste_level: Stat,
    shape: T,
    alive: bool,
}

impl<T> Friend<T>
where
    T: Shape,
{
    pub fn new(
        name: String,
        hunger: Stat,
        joy: Stat,
        energy: Stat,
        waste_level: Stat,
        shape: T,
        alive: bool,
    ) -> Self {
        Self {
            name,
            hunger,
            joy,
            energy,
            waste_level,
            shape,
            alive,
        }
    }

    /// Updates this Friend's status for one second passed
    pub fn update_state(&mut self) {
        todo!()
    }

    fn update_alive_status(&mut self) {
        if self.hunger.value() == 0 {
            self.alive = false;
        }

        if self.joy.value() == 0 {
            self.alive = false;
        }

        if self.energy.value() == 0 {
            self.alive = false;
        }

        if self.waste_level.value() >= 100 {
            self.alive = false;
        }
    }

    pub fn eat(&mut self, food: Food) {
        self.hunger.subtract(food.points());
    }

    pub fn sleep(&mut self) {
        self.energy.add(10);
    }

    pub fn play(&mut self) {
        self.joy.add(20);
    }

    pub fn poop(&mut self) {
        self.waste_level.subtract(50);
    }
}