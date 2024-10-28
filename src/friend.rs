use crate::food::Food;
use crate::utils::Stat;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::shapes::creatures::CreatureShapes;

#[derive(Debug, Serialize, Deserialize)]
pub struct Friend {
    name: String,
    food: Stat,
    joy: Stat,
    energy: Stat,
    waste_level: Stat,
    last_time_lower_food: i64,
    last_time_lower_joy: i64,
    last_time_lower_energy: i64,
    last_time_increase_waste: i64,
    shape: CreatureShapes,
    asleep: bool,
    alive: bool,
}

impl Friend {
    pub fn new(name: &str, shape: CreatureShapes) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            name: String::from(name),
            food: Stat::new(50).unwrap(),
            joy: Stat::new(50).unwrap(),
            energy: Stat::new(50).unwrap(),
            waste_level: Stat::new(50).unwrap(),
            last_time_lower_food: now,
            last_time_lower_joy: now,
            last_time_lower_energy: now,
            last_time_increase_waste: now,
            shape,
            asleep: false,
            alive: true,
        }
    }

    /// Updates this Friend's state for each minute passed since last update <br>
    pub fn update_state(&mut self) {
        let now = Utc::now().timestamp_millis();
        let minute_millis = 1000 * 60;
        
        // Use while loops instead of if statements to account for loading from file, 
        // when we might have been away for more than a single minute.
        while now - self.last_time_lower_food >= minute_millis {
            self.food.subtract(1);
            self.last_time_lower_food += minute_millis;
        }

        while now - self.last_time_lower_energy >= minute_millis {
            match self.asleep {
                true => self.energy.add(1),
                false => self.energy.subtract(1),
            }
            self.last_time_lower_energy += minute_millis;
        }
        
        while now - self.last_time_lower_joy >= minute_millis {
            self.joy.subtract(1);
            self.last_time_lower_joy += minute_millis;
        }
        
        while now - self.last_time_increase_waste >= minute_millis {
            self.waste_level.add(1);
            self.last_time_increase_waste += minute_millis;
        }
        
        self.update_alive_status();
    }

    fn update_alive_status(&mut self) {
        let stats_sum = self.food.value() + self.joy.value() + self.energy.value();
        if stats_sum < 20 {
            self.alive = false;
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn eat(&mut self, food: Food) {
        self.food.add(food.points());
    }

    pub fn sleep(&mut self) {
        self.asleep = !self.asleep;
    }
    
    pub fn is_asleep(&self) -> bool {
        self.asleep
    }

    pub fn play(&mut self) {
        self.joy.add(20);
    }

    pub fn poop(&mut self) {
        self.waste_level.subtract(50);
    }

    pub fn food(&self) -> &Stat {
        &self.food
    }

    pub fn joy(&self) -> &Stat {
        &self.joy
    }

    pub fn energy(&self) -> &Stat {
        &self.energy
    }

    pub fn waste_level(&self) -> &Stat {
        &self.waste_level
    }
    
    pub fn shape(&self) -> &CreatureShapes {
        &self.shape
    }
    
    pub fn alive(&self) -> &bool {
        &self.alive
    }
}