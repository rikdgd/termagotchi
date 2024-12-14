use crate::food::Food;
use crate::utils::Stat;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::shapes::creatures::CreatureShapes;
use crate::shapes::{GrowthStageShapes, PixelVectorShape};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrowthStage {
    Egg,
    Baby,
    Kid,
    Adult,
}

impl GrowthStage {
    /// Upgrades self to the next logical growth stage.
    pub fn next_stage(&mut self) {
        match self {
            GrowthStage::Egg => *self = GrowthStage::Baby,
            GrowthStage::Baby => *self = GrowthStage::Kid,
            GrowthStage::Kid => *self = GrowthStage::Adult,

            GrowthStage::Adult => (),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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
    full_waste_since: Option<i64>,
    shape: CreatureShapes,
    growth_stage: GrowthStage,
    asleep: bool,
    alive: bool,
    time_created: i64,
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
            full_waste_since: None,
            shape,
            growth_stage: GrowthStage::Egg,
            asleep: false,
            alive: true,
            time_created: now,
        }
    }

    /// Updates this Friend's state for each minute passed since last update <br>
    pub fn update_state(&mut self) {
        let now = Utc::now().timestamp_millis();

        self.update_growth_stage(now);
        
        if self.growth_stage != GrowthStage::Egg {
            self.update_stats(now);
            self.update_alive_status(now);
        }
    }
    
    fn update_stats(&mut self, now: i64) {
        let minute_millis = 1000 * 60;

        // Use while loops instead of if statements to account for loading from file
        // when we might have been away for more than a single minute.
        while now - self.last_time_lower_food >= 7 * minute_millis {
            self.food.subtract(1);
            self.last_time_lower_food += 7 * minute_millis;
        }

        while now - self.last_time_lower_energy >= 6 * minute_millis {
            match self.asleep {
                true => self.energy.add(3),
                false => self.energy.subtract(1),
            }
            self.last_time_lower_energy += 6 * minute_millis;
        }

        while now - self.last_time_lower_joy >= 8 * minute_millis {
            self.joy.subtract(1);
            self.last_time_lower_joy += 8 * minute_millis;
        }

        if self.waste_level.value() >= 100 && self.full_waste_since == None {
            self.full_waste_since = Some(now);
        }
        if self.waste_level.value() < 100 && self.full_waste_since != None {
            self.full_waste_since = None;
        }
    }

    fn update_alive_status(&mut self, now: i64) {
        let stats_sum = self.food.value() + self.joy.value() + self.energy.value();
        if stats_sum < 15 {
            self.alive = false;
        }
        
        let mut counter: u8 = 0;
        for stat in [self.food.value(), self.joy.value(), self.energy.value()] {
            if stat == 0 {
                counter += 1;
            }
        }
        if counter >= 2 {
            self.alive = false;
        }
        
        if let Some(time) = self.full_waste_since {
            // If the waste level has been maxed out for 2 hours.
            if self.waste_level.value() >= 100 && now - time > 1000 * 60 * 60 * 2 {
                self.alive = false;
            }
        }
    }

    fn update_growth_stage(&mut self, now: i64) {
        let growth_delay = match self.growth_stage {
            GrowthStage::Egg => Some(300000),    // 5 minutes
            GrowthStage::Baby => Some(18000000),  // 5 hours
            GrowthStage::Kid => Some(86400000),   // 24 hours
            GrowthStage::Adult => None,
        };
        
        if let Some(growth_delay) = growth_delay {
            if now - self.time_created > growth_delay {
                self.growth_stage.next_stage();
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn eat(&mut self, food: &Food) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }
        
        self.food.add(food.points());
        self.waste_level.add(food.points() / 2);
    }

    pub fn toggle_sleep(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.asleep = !self.asleep;
        }
    }
    
    pub fn play(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.joy.add(30);
        }
    }

    pub fn poop(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.waste_level.subtract(50);
        }
    }

    pub fn is_asleep(&self) -> bool {
        self.asleep
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
    
    pub fn growth_stage(&self) -> GrowthStage {
        self.growth_stage
    }
    
    pub fn get_shape_wrapper(&self) -> ShapeWrapper {
        let color = self.shape.get_color();
        match self.growth_stage {
            GrowthStage::Egg => ShapeWrapper::Growing(GrowthStageShapes::Egg(color)),
            GrowthStage::Baby => ShapeWrapper::Growing(GrowthStageShapes::Baby(color)),
            GrowthStage::Kid => ShapeWrapper::Growing(GrowthStageShapes::Kid(color)),

            GrowthStage::Adult => ShapeWrapper::Adult(self.shape.clone()),
        }
    }
    
    pub fn get_pixel_vector(&self) -> PixelVectorShape {
        match self.get_shape_wrapper() {
            ShapeWrapper::Growing(shape) => PixelVectorShape::from_pixel_image(&shape),
            ShapeWrapper::Adult(shape) => PixelVectorShape::from_pixel_image(&shape),
        }
    }
    
    pub fn alive(&self) -> &bool {
        &self.alive
    }
    
    pub fn time_created(&self) -> i64 { self.time_created }
}

#[derive(Debug, Clone)]
pub enum ShapeWrapper {
    Growing(GrowthStageShapes),
    Adult(CreatureShapes),
}
