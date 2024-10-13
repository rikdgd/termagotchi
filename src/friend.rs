use crate::food::Food;
use crate::utils::Stat;
use ratatui::widgets::canvas::Shape;
use serde::{Deserialize, Serialize};
use chrono::Utc;


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
    last_time_lower_hunger: i64,
    last_time_lower_joy: i64,
    last_time_lower_energy: i64,
    shape: T,
    asleep: bool,
    alive: bool,
}

impl<T> Friend<T>
where
    T: Shape,
{
    pub fn new(name: &str, shape: T) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            name: String::from(name),
            hunger: Stat::new(50).unwrap(),
            joy: Stat::new(50).unwrap(),
            energy: Stat::new(50).unwrap(),
            waste_level: Stat::new(50).unwrap(),
            last_time_lower_hunger: now,
            last_time_lower_joy: now,
            last_time_lower_energy: now,
            shape,
            asleep: false,
            alive: true,
        }
    }

    /// Updates this Friend's status for one second passed
    pub fn update_state(&mut self) {
        let now = Utc::now().timestamp_millis();
        if now - self.last_time_lower_hunger >= 1000 * 60 {
            self.hunger.subtract(1);
        }
        // TODO: same if statement for the other stats
        
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