mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;
mod widgets;
mod layouts;
mod animations;
mod movements;

use std::time::Duration;
use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, poll}, layout::{Constraint, Layout}, DefaultTerminal, Frame, Terminal};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use shapes::creatures::CreatureShapes;
use crate::friend::{Friend, GrowthStage};
use crate::game_state::GameState;
use crate::utils::ColorWrapper;
use widgets::{stats_widget, actions_widget};
use crate::food::Food;
use crate::movements::{EggHopMovement, SmallStepsMovement, Location, Movement, MovementWrapper, DvdBounceMovement};
use crate::widgets::FriendWidget;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut actions_widget_state = ListState::default();
    
    let mut game_state: GameState;
    if let Ok(state) = GameState::read_from_file() {
        game_state = state;
        
    } else {
        let friend = Friend::new("temp friend", CreatureShapes::Duck(ColorWrapper::Red));
        game_state = GameState::new(friend);    // Create a temporary GameState, this will never be used.
        layouts::draw_new_friend_layout(&mut terminal, &mut game_state)?;
    }
    
    let mut previous_growth_stage = game_state.friend_clone().growth_stage();
    let mut friend_movement = get_movement_wrapper(
        &game_state.friend().growth_stage(), 
        get_friend_boundaries(&mut terminal),
    );
    
    loop {
        game_state.update();
        if !game_state.friend().alive() {
            layouts::draw_new_friend_layout(&mut terminal, &mut game_state)?;
        }
        
        if previous_growth_stage != game_state.friend().growth_stage() {
            previous_growth_stage = game_state.friend_clone().growth_stage();
            
            let friend_constraints = get_friend_boundaries(&mut terminal);
            update_friend_movement(&mut friend_movement, game_state.friend(), friend_constraints);
        }
        
        terminal.draw(|frame| {
            draw_main(frame, game_state.friend_mut(), &mut friend_movement, &mut actions_widget_state);
        })?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        
                        KeyCode::Up => actions_widget_state.select_previous(),
                        KeyCode::Down => actions_widget_state.select_next(),
                        KeyCode::Enter => {
                            if let Some(action) = actions_widget_state.selected() {
                                let action = actions_widget::ITEMS[action];
                                match action {
                                    // TODO: Eat the user provided food, instead of defaulting to Burger
                                    "Eat" => game_state.friend_mut().eat(Food::Burger),
                                    "Play" => game_state.friend_mut().play(),
                                    "Sleep" => game_state.friend_mut().sleep(),
                                    "Poop" => game_state.friend_mut().poop(),
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

    ratatui::restore();
    game_state.store_to_file()?;
    
    Ok(())
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


/// Draws the main screen of the application, which allows for users to interact with their friend.
fn draw_main<T: Movement>(frame: &mut Frame, friend: &Friend, friend_movement: &mut T, actions_widget_state: &mut ListState) {
    let frame_area = frame.area();
    let [left_area, middle_area, right_area] = get_main_areas(frame_area);
    
    let stats_widget = stats_widget(&left_area, friend);
    for gauge in stats_widget {
        frame.render_widget(gauge.0, gauge.1);
    }
    
    let friend_widget = FriendWidget::new(friend, friend_movement.next_position());
    frame.render_widget(friend_widget.get_widget(), middle_area);
    frame.render_stateful_widget(actions_widget(), right_area, actions_widget_state);
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
