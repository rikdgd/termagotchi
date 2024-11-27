use std::time::Duration;
use ratatui::{Frame, DefaultTerminal};
use ratatui::widgets::ListState;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Rect;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
use crate::friend::GrowthStage;
use crate::game_state::GameState;
use crate::movements::{Movement, MovementWrapper, EggHopMovement, SmallStepsMovement, DvdBounceMovement};
use crate::friend::Friend;
use crate::widgets::{stats_widget, FriendWidget, actions_widget};
use crate::movements::Location;
use crate::layouts;
use crate::Food;
use crate::shapes::creatures::CreatureShapes;
use crate::utils::ColorWrapper;


pub struct App {
    game_state: GameState,
    actions_widget_state: ListState,
    previous_growth_stage: GrowthStage,
    friend_movement: MovementWrapper,
}

impl App {
    pub fn new(terminal: &mut DefaultTerminal) -> std::io::Result<Self> {
        let actions_widget_state = ListState::default();

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
            get_friend_boundaries(terminal),
        );
        
        Ok(Self {
            game_state,
            actions_widget_state,
            previous_growth_stage,
            friend_movement,
        })
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            self.game_state.update();
            if !self.game_state.friend().alive() {
                layouts::draw_new_friend_layout(terminal, &mut self.game_state)?;
            }

            if self.previous_growth_stage != self.game_state.friend().growth_stage() {
                self.previous_growth_stage = self.game_state.friend().growth_stage();

                let friend_boundaries = get_friend_boundaries(terminal);
                update_friend_movement(&mut self.friend_movement, self.game_state.friend(), friend_boundaries);
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
                                        "Sleep" => self.game_state.friend_mut().sleep(),
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
            FriendWidget::new(self.game_state.friend(), self.friend_movement.next_position())
        } else {
            let friend_area = get_main_areas(frame.area())[1]; // index 1, because the center area is where our friend 'lives'.
            let location = Location::new(
                friend_area.width as u32 / 2,
                friend_area.height as u32 / 2,
            );
            FriendWidget::new(self.game_state.friend(), location)
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
fn update_friend_movement(movement: &mut MovementWrapper, friend: &Friend, max_xy: (u32, u32)) {
    *movement = get_movement_wrapper(&friend.growth_stage(), max_xy);
}

/// Gets the width and height of the area the friend will move around in. The friend should always
/// stay within these boundaries.
///
/// ## Parameters:
/// * `terminal` - The `ratatui::DefaultTerminal` that should be used to draw the game to.
///
/// ## Returns:
/// A `(u32, u32)` tuple where the width and height are ordered as: (width, height)
fn get_friend_boundaries(terminal: &mut DefaultTerminal) -> (u32, u32) {
    let frame_area = terminal.get_frame().area();
    let [_, middle_area, _] = get_main_areas(frame_area);
    (middle_area.width as u32, middle_area.height as u32)
}

fn get_movement_wrapper(growth_stage: &GrowthStage, max_xy: (u32, u32)) -> MovementWrapper {
    match growth_stage {
        GrowthStage::Egg => MovementWrapper::EggHop(EggHopMovement::new(Location::new(40, 20))),
        GrowthStage::Baby => MovementWrapper::SmallSteps(SmallStepsMovement::new(Location::new(40, 20))),
        _ => MovementWrapper::DvdBounce(DvdBounceMovement::new(Location::new(23, 11), max_xy.0, max_xy.1)),
    }
}
