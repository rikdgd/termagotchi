use crate::food::Food;
use crate::utils::Stat;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::shapes::creatures::CreatureShapes;
use crate::shapes::{GrowthStageShapes, PixelVectorShape};

const MINUTE_MILLIS: i64 = 1000 * 60;

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
    health: Stat,
    last_time_lower_food: i64,
    last_time_lower_joy: i64,
    last_time_lower_energy: i64,
    last_time_lower_health: i64,
    health_decrease_time_left: i64,
    shape: CreatureShapes,
    growth_stage: GrowthStage,
    asleep: bool,
    asleep_since: Option<i64>,
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
            health: Stat::new(50).unwrap(),
            last_time_lower_food: now,
            last_time_lower_joy: now,
            last_time_lower_energy: now,
            last_time_lower_health: now,
            health_decrease_time_left: 0,
            shape,
            growth_stage: GrowthStage::Egg,
            asleep: false,
            asleep_since: None,
            alive: true,
            time_created: now,
        }
    }

    /// Updates this Friend's state for each minute passed since last update <br>
    pub fn update_state(&mut self, now: i64) {
        self.update_growth_stage(now);
        
        if self.growth_stage != GrowthStage::Egg {
            self.update_stats(now);
            // self.update_asleep_status(now);
            self.update_alive_status();
        }
    }
    
    fn update_stats(&mut self, now: i64) {
        let food_offset_minutes = 16 * MINUTE_MILLIS;
        let energy_offset_minutes = 14 * MINUTE_MILLIS;
        let joy_offset_minutes = 18 * MINUTE_MILLIS;
        let health_offset_minutes = MINUTE_MILLIS;

        // Use while loops instead of if statements to account for loading from file
        // when we might have been away for more than a single minute.
        while now - self.last_time_lower_food >= food_offset_minutes {
            self.food.subtract(1);
            self.last_time_lower_food += food_offset_minutes;
        }

        while now - self.last_time_lower_energy >= energy_offset_minutes {
            match self.asleep {
                true => self.energy.add(3),
                false => self.energy.subtract(1),
            }
            self.last_time_lower_energy += energy_offset_minutes;
            
            self.update_asleep_status(now);
        }

        while now - self.last_time_lower_joy >= joy_offset_minutes {
            self.joy.subtract(1);
            self.last_time_lower_joy += joy_offset_minutes;
        }
        
        while now - self.last_time_lower_health >= health_offset_minutes {
            if self.health_decrease_time_left >= health_offset_minutes {
                self.health.subtract(1);
                self.health_decrease_time_left -= health_offset_minutes;
            }
            self.last_time_lower_health += health_offset_minutes;
        }
    }
    
    /// Updates the sleeping state of the Friend. This will wake the friend up after it has been asleep
    /// for to long so the player cannot just let it sleep forever.
    /// <br>
    /// ## parameters:
    /// * `now` - The current utc time in millis, used to determine the time elapsed since the friend fell asleep.
    fn update_asleep_status(&mut self, now: i64) {
        if !self.asleep {
            return;
        }
        
        if let Some(start_sleeping) = self.asleep_since {
            if now - start_sleeping > MINUTE_MILLIS * 60 * 12 {
                self.asleep = false;
                self.asleep_since = None;
            }
        }
    }

    fn update_alive_status(&mut self) {
        let stats_sum = self.food.value() + self.joy.value() + self.health.value();
        if stats_sum < 15 {
            self.alive = false;
        }
        
        let mut counter: u8 = 0;
        for stat in [self.food.value(), self.joy.value(), self.energy.value(), self.health.value()] {
            if stat == 0 {
                counter += 1;
            }
        }
        if counter >= 2 {
            self.alive = false;
        }
        
        if self.health.value() == 0 {
            self.alive = false;
        }
    }

    fn update_growth_stage(&mut self, now: i64) {
        let growth_delay = match self.growth_stage {
            GrowthStage::Egg => Some(300000),     // 5 minutes
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

    pub fn eat(&mut self, food: Food) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }
        
        self.food.add(food.points());
        self.health_decrease_time_left += (food.points() / 3) as i64 * MINUTE_MILLIS;
    }

    pub fn toggle_sleep(&mut self) {
        if self.growth_stage == GrowthStage::Egg {
            return;
        }

        self.asleep = !self.asleep;
        
        if self.asleep {
            let now = Utc::now().timestamp_millis();
            self.asleep_since = Some(now);
        } else {
            self.asleep_since = None;
        }
    }
    
    pub fn play(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.joy.add(30);
            self.health_decrease_time_left += 10 * MINUTE_MILLIS;
        }
    }

    pub fn take_medicine(&mut self) {
        if self.growth_stage != GrowthStage::Egg {
            self.health.add(40);
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

    pub fn health(&self) -> &Stat {
        &self.health
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


#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::friend::{Friend, GrowthStage, MINUTE_MILLIS};
    use crate::shapes::creatures::CreatureShapes;
    use crate::utils::{ColorWrapper, Stat};

    #[test]
    fn friend_auto_wakeup_test() {
        let max_sleep_time = MINUTE_MILLIS * 60 * 12;
        let now = Utc::now().timestamp_millis();
        let mut friend = Friend::new(
            "test-friend", 
            CreatureShapes::Squid(ColorWrapper::Green)
        );
        
        friend.growth_stage = GrowthStage::Adult;
        friend.food = Stat::new(100).unwrap();
        friend.joy = Stat::new(100).unwrap();
        friend.energy = Stat::new(100).unwrap();
        friend.health = Stat::new(100).unwrap();
        friend.toggle_sleep();
        
        friend.update_state(now + max_sleep_time + 100);
        
        assert!(!friend.asleep);
        assert_eq!(None, friend.asleep_since);
    }
    
    #[test]
    fn lower_energy_after_auto_wakeup() {
        let max_sleep_time = MINUTE_MILLIS * 60 * 12;
        let now = Utc::now().timestamp_millis();
        let mut friend = Friend::new(
            "test-friend",
            CreatureShapes::Squid(ColorWrapper::Green)
        );

        friend.food = Stat::new(100).unwrap();
        friend.joy = Stat::new(100).unwrap();
        friend.energy = Stat::new(100).unwrap();
        friend.health = Stat::new(100).unwrap();
        friend.toggle_sleep();
        
        friend.update_state(now + 10 * max_sleep_time); // Make sure the termagotchi energy is at zero.
        
        assert!(!friend.asleep);
        assert_eq!(None, friend.asleep_since);
        assert_eq!(0, friend.energy.value());
    }
}