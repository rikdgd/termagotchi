use std::time::Duration;
use ratatui::{Frame, DefaultTerminal};
use ratatui::widgets::ListState;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Rect;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
use crate::game_state::GameState;
use crate::movements::{Movement, MovementWrapper, EggHopMovement, SmallStepsMovement, DvdBounceMovement};
use crate::friend::{Friend, GrowthStage};
use crate::widgets::{stats_widget, FriendWidget, actions_widget};
use crate::movements::Location;
use crate::layouts;
use crate::food::Food;
use crate::shapes::creatures::CreatureShapes;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;

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
}

impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> std::io::Result<Self> {
        let actions_widget_state = ListState::default();
        let playground = Rect::new(0, 0, 150, 100);

        let mut game_state: GameState;
        if let Ok(state) = GameState::read_from_file() {
            game_state = state;

        } else {
            let friend = Friend::new("temp friend", CreatureShapes::Duck(ColorWrapper::Red));
            game_state = GameState::new(friend);    // Create a temporary GameState, this will never be used.
            layouts::draw_new_friend_layout(terminal, &mut game_state)?;
        }

        let previous_growth_stage = game_state.friend_clone().growth_stage();

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
        })
    }
    
    /// Starts the main application loop and handles user input.
    /// 
    /// ## parameters:
    /// * `terminal` - The ratatui terminal to draw the application on.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            self.game_state.update();
            if !self.game_state.friend().alive() {
                layouts::draw_new_friend_layout(terminal, &mut self.game_state)?;
            }

            if self.previous_growth_stage != self.game_state.friend().growth_stage() {
                self.previous_growth_stage = self.game_state.friend().growth_stage();

                update_friend_movement(&mut self.friend_movement, self.game_state.friend(), self.playground);
            }

            terminal.draw(|frame| {
                self.draw_main(frame);
            })?;

            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,

                            KeyCode::Up => self.actions_widget_state.select_previous(),
                            KeyCode::Down => self.actions_widget_state.select_next(),
                            KeyCode::Enter => {
                                if let Some(action) = self.actions_widget_state.selected() {
                                    let action = actions_widget::ITEMS[action];
                                    match action {
                                        "Eat" => {
                                            if !self.game_state.friend().is_asleep() {
                                                let food = Food::new_random();
                                                self.game_state.friend_mut().eat(&food);
                                            }
                                        },
                                        "Play" => {
                                            if !self.game_state.friend().is_asleep() {
                                                self.game_state.friend_mut().play();
                                            }
                                        },
                                        "Sleep" => self.game_state.friend_mut().toggle_sleep(),
                                        "Poop" => {
                                            if !self.game_state.friend().is_asleep() {
                                                self.game_state.friend_mut().poop();
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

        let stats_widget = stats_widget(&left_area, self.game_state.friend());
        for gauge in stats_widget {
            frame.render_widget(gauge.0, gauge.1);
        }

        let friend_widget = if !self.game_state.friend().is_asleep() {
            FriendWidget::new(self.game_state.friend(), self.friend_movement.next_position(), self.playground)
        } else {
            let friend_area = get_main_areas(frame.area())[1]; // index 1, because the center area is where our friend 'lives'.
            let location = Location::new(
                self.playground.width as u32 / 2,
                self.playground.height as u32 / 2,
            );
            FriendWidget::new(self.game_state.friend(), location, self.playground)
        };


        frame.render_widget(friend_widget.get_widget(), middle_area);
        frame.render_stateful_widget(actions_widget(), right_area, &mut self.actions_widget_state);
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


/// updates the movement of the creature based on its growth stage
///
/// ## parameters
/// * `movement` - The movement that should be modified.
/// * `friend` - The friend that will be used to check the growth stage.
fn update_friend_movement(movement: &mut MovementWrapper, friend: &Friend, area: Rect) {
    let shape = friend.get_pixel_vector();
    *movement = get_movement_wrapper(&friend.growth_stage(), area, shape);
}

fn get_movement_wrapper(growth_stage: &GrowthStage, area: Rect, friend_shape: PixelVectorShape) -> MovementWrapper {
    match growth_stage {
        GrowthStage::Egg => MovementWrapper::EggHop(EggHopMovement::new(Location::new(60, 35))),
        GrowthStage::Baby => MovementWrapper::SmallSteps(SmallStepsMovement::new(Location::new(40, 20))),
        _ => MovementWrapper::DvdBounce(DvdBounceMovement::new(Location::new(23, 11), area, friend_shape)),
    }
}
