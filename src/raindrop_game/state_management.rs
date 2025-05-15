use crate::utils::location::Location;
use std::time::SystemTime;
use ratatui::layout::Rect;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct RaindropGameState {
    pub score: u32,
    pub health: u8,
    pub player_x: u32,
    pub drop_locations: Vec<Location>,
    pub game_area: Rect,
    last_update_time: SystemTime,
}
impl RaindropGameState {
    pub fn new(game_area: Rect) -> Self {
        Self {
            score: 0,
            health: 5,
            player_x: 50,   // TODO: Should be center of the screen/game-area
            drop_locations: Vec::new(),
            game_area,
            last_update_time: SystemTime::now(),
        }
    }
    
    /// Progresses `self` to the next frame. It will only actually do so if at least
    /// x amount of time has passed since the last state update to limit the framerate.
    pub fn update_state(&mut self) {
        let millis_past = self.last_update_time
            .elapsed()
            .expect("Failed to calculate the elapsed time since last (raindrop)game state update.")
            .as_millis();
        
        // Only update the game state at max 4 times a second
        if millis_past >= 250 {
            self.update_drop_locations();
            self.update_player_health();
            self.clean_drops();
        } 
    }
    
    /// Updates `self.drop_locations` by moving all existing drops down, and generating a random amount
    /// of new drops. The drops are generated at random locations at the top of the screen.
    fn update_drop_locations(&mut self) {
        for drop in &mut self.drop_locations {
            drop.y -= 10;
        }
        
        let mut rng = rand::thread_rng();
        
        let new_drop_count = rng.gen_range(1..=5);
        for _ in 0..new_drop_count {
            let drop_x = rng.gen_range(0..self.game_area.width);
            self.drop_locations.push(Location {
                x: drop_x as u32,
                y: self.game_area.height as u32,
            });
        }
    }
    
    /// Checks if the player is colliding with a drop, and if so, removes some health points.
    fn update_player_health(&mut self) {
        for drop in &self.drop_locations {
            if self.collides_with_player(drop) {
                if self.health > 0 {
                    self.health -= 1;
                }
            }
        }
    }
    
    /// Checks if a drop at the given location would collide with the player.
    /// ## parameters:
    /// * `drop_location` - The location of the drop that should be tested for player collision.
    /// ## returns:
    /// `true` when the drop at the given location collides with the player, `false` otherwise.
    fn collides_with_player(&self, drop_location: &Location) -> bool {
        let in_x_range = 
            drop_location.x > self.player_x - 12 && 
            drop_location.x < self.player_x + 12;
        let in_y_range = drop_location.y <= 25;
        
        in_x_range && in_y_range
    }
    
    /// Removes all the raindrops from `self.drop_locations` that are either outside of the screen,
    /// or colliding with the player.
    fn clean_drops(&mut self) {
        let filter_res: Vec<&Location> = self.drop_locations.iter().filter(|drop| {
            self.collides_with_player(drop) || drop.y == 0
        }).collect();
        
        let mut new_drops = Vec::new();
        for drop in filter_res {
            new_drops.push(drop.clone());
        }
        
        self.drop_locations = new_drops;
    }
}
