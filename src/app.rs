use std::time::Duration;
use ratatui::{Frame, DefaultTerminal};
use ratatui::widgets::ListState;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Rect;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
use crate::game_state::GameState;
use crate::movements::{Movement, MovementWrapper, EggHopMovement, SmallStepsMovement, DvdBounceMovement};
use crate::friend::{Friend, GrowthStage};
use crate::widgets::{FriendWidget, actions_widget, StatsWidget};
use crate::utils::location::Location;
use crate::layouts;
use crate::food::Food;
use crate::shapes::creatures::CreatureShapes;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;
use crate::animations::PopupAnimation;
use crate::animations::food_animation::{FoodAnimation, FoodAnimationFrames};
use crate::animations::{HealthAnimation, JoyAnimation};

/// This struct holds most logic for actually running the app. It is able to run the Termagotchi app
/// using a `ratatui::DefaultTerminal` and keeps track of: game state, widget states, movements and animations.
/// 
/// ## example:
/// ```
/// fn main() -> std::io::Result<()> {
///     let mut terminal = ratatui::init();
///     let mut app = App::new(&mut terminal)?;
///     
///     app.run(&mut terminal)?;
///     app.save_game()?;
///     ratatui::restore();
///     
///     Ok(())
/// }
/// ```
pub struct App {
    game_state: GameState,
    actions_widget_state: ListState,
    previous_growth_stage: GrowthStage,
    friend_movement: MovementWrapper,
    playground: Rect,
    popup_animation: Option<PopupAnimation>,
    allow_inputs: bool,
    is_running: bool,
}

impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> std::io::Result<Self> {
        let actions_widget_state = ListState::default();
        let playground = Rect::new(0, 0, 150, 100);

        let mut game_state: GameState;
        if let Ok(state) = GameState::read_from_file() {
            game_state = state;

        } else {
            game_state = layouts::draw_new_friend_layout(terminal)?;
        }

        let previous_growth_stage = game_state.friend().growth_stage();

        let friend_movement = get_movement_wrapper(
            &game_state.friend().growth_stage(),
            playground,
            game_state.friend().get_pixel_vector(),
        );
        
        Ok(Self {
            game_state,
            actions_widget_state,
            previous_growth_stage,
            friend_movement,
            playground,
            popup_animation: None,
            allow_inputs: true,
            is_running: true,
        })
    }
    
    /// Starts the main application loop and handles user input.
    /// 
    /// ## parameters:
    /// * `terminal` - The ratatui terminal to draw the application on.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.is_running {
            self.game_state.update();
            if !self.game_state.friend().alive() {
                layouts::friend_death_layout(terminal, &mut self.game_state);
            }

            if self.previous_growth_stage != self.game_state.friend().growth_stage() {
                self.previous_growth_stage = self.game_state.friend().growth_stage();

                update_friend_movement(&mut self.friend_movement, self.game_state.friend(), self.playground);
            }

            terminal.draw(|frame| {
                self.draw_main(frame);

                // Check if a popup animation should be displayed, or hidden when it has finished.
                if let Some(popup_animation) = &mut self.popup_animation {
                    self.allow_inputs = false;
                    if popup_animation.is_running() {
                        popup_animation.render(frame);
                    } else {
                        self.popup_animation = None;
                        self.allow_inputs = true;
                    }
                }
            })?;

            self.handle_inputs()?;
        }
        
        Ok(())
    }
    
    /// Saves the game's state to a file by calling `GameState::store_to_file()`.
    pub fn save_game(&mut self) -> std::io::Result<()> {
        self.game_state.store_to_file()
    }

    /// Draws the main screen of the application, which allows for users to interact with their friend.
    fn draw_main(&mut self, frame: &mut Frame) {
        let frame_area = frame.area();
        let [left_area, middle_area, right_area] = get_main_areas(frame_area);
        
        let canvas_stats = StatsWidget::new(self.game_state.friend());
        
        let friend_widget = if !self.game_state.friend().is_asleep() {
            FriendWidget::new(self.game_state.friend(), self.friend_movement.next_position(), self.playground)
        } else {
            FriendWidget::new(self.game_state.friend(), self.sleep_drawing_location(), self.playground)
        };
        
        
        frame.render_widget(canvas_stats.get_widget(), left_area);
        frame.render_widget(friend_widget.get_widget(), middle_area);
        frame.render_stateful_widget(actions_widget(), right_area, &mut self.actions_widget_state);
    }

    fn handle_inputs(&mut self) -> std::io::Result<()> {
        if self.allow_inputs && poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {

                    match key.code {
                        KeyCode::Char('q') => self.is_running = false,

                        KeyCode::Up => self.actions_widget_state.select_previous(),
                        KeyCode::Down => self.actions_widget_state.select_next(),
                        KeyCode::Enter => {
                            if let Some(action) = self.actions_widget_state.selected() {
                                let action = actions_widget::ITEMS[action];
                                let is_awake = !self.game_state.friend().is_asleep();
                                let is_not_egg = self.game_state.friend().growth_stage() != GrowthStage::Egg;
                                match action {
                                    "Eat" => {
                                        if is_awake && is_not_egg {
                                            let food = Food::new_random();
                                            self.set_food_animation(food);
                                            self.game_state.friend_mut().eat(food);
                                        }
                                    },
                                    "Play" => {
                                        if is_awake && is_not_egg {
                                            self.set_joy_animation();
                                            self.game_state.friend_mut().play();
                                        }
                                    },
                                    "Sleep" => {
                                        if is_not_egg {
                                            self.game_state.friend_mut().toggle_sleep()
                                        }
                                    },
                                    "Medicine" => {
                                        if is_awake && is_not_egg {
                                            self.set_health_animation();
                                            self.game_state.friend_mut().take_medicine();
                                        }
                                    },
                                    _ => ()
                                }
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
        Ok(())
    }

    fn set_food_animation(&mut self, food: Food) {
        if self.game_state.friend().food().is_max() {
            return;
        }
        
        let frames = match food {
            Food::Soup => FoodAnimationFrames::Soup,
            Food::Fries => FoodAnimationFrames::Fries,
            Food::Burger => FoodAnimationFrames::Burger,
        };

        self.popup_animation = Some(PopupAnimation::new(
            Box::new(FoodAnimation::new(frames)),
            (15, 15),
        ))
    }
    
    fn set_health_animation(&mut self) {
        if self.game_state.friend().health().is_max() {
            return;
        }
        
        self.popup_animation = Some(PopupAnimation::new(
            Box::new(HealthAnimation::new()),
            (15, 15)
        ))
    }
    
    fn set_joy_animation(&mut self) {
        if self.game_state.friend().joy().is_max() {
            return;
        }
        
        self.popup_animation = Some(PopupAnimation::new(
            Box::new(JoyAnimation::new()),
            (15, 15)
        ))
    }
    
    fn sleep_drawing_location(&self) -> Location {
        let mut center = Location {
            x: self.playground.width as u32 / 2,
            y: self.playground.height as u32 / 2,
        };

        let sprite_width = match self.game_state.friend().growth_stage() {
            GrowthStage::Egg => 10,
            GrowthStage::Baby => 10,
            GrowthStage::Kid => 15,
            GrowthStage::Adult => 25,
        };

        center.x -= sprite_width / 2;
        center.y -= sprite_width / 2;
        center
    }
}



/// Returns the 3 areas used on the main screen in an array.
///
/// ## Parameters
/// * `area` - The area of the `ratatui::Frame` the widgets will be drawn to.
///
/// ## Example usage:
/// ```
/// let frame_area = frame.area();
/// let [left_area, middle_area, right_area] = get_main_areas(frame_area);
/// ```
fn get_main_areas(area: Rect) -> [Rect; 3] {
    Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
        .areas(area)
}


/// Updates the movement of the creature based on its growth stage.
/// <br>
/// ## parameters:
/// * `movement` - The movement that should be modified.
/// * `friend` - The friend that will be used to check the growth stage.
/// * `area` - The area where the creature walks around in, used to set movement boundaries.
fn update_friend_movement(movement: &mut MovementWrapper, friend: &Friend, area: Rect) {
    let shape = friend.get_pixel_vector();
    *movement = get_movement_wrapper(&friend.growth_stage(), area, shape);
}

fn get_movement_wrapper(growth_stage: &GrowthStage, area: Rect, friend_shape: PixelVectorShape) -> MovementWrapper {
    let center = Location::new(area.width as u32 / 2, area.height as u32 / 2);
    match growth_stage {
        GrowthStage::Egg => MovementWrapper::EggHop(EggHopMovement::new(center)),
        GrowthStage::Baby => MovementWrapper::SmallSteps(SmallStepsMovement::new(center)),
        _ => MovementWrapper::DvdBounce(DvdBounceMovement::new(center, area, friend_shape)),
    }
}
