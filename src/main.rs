mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;
mod widgets;

use std::time::Duration;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Layout},
    Frame,
};
use ratatui::widgets::ListState;
use shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::utils::ColorWrapper;
use widgets::{stats_widget, actions_widget};
use crate::food::Food;
use crate::widgets::FriendWidget;

fn main() -> std::io::Result<()> {
    let mut game_state = match GameState::file_exists() {
        true => {
            GameState::read_from_file()?
        },
        false => {
            // TODO: Randomize shape and color.
            let friend = Friend::new(
                "Wally", 
                CreatureShapes::Egg(ColorWrapper::Red), 
            );
            GameState::new(friend)
        }
    };
    
    let mut terminal = ratatui::init();
    let mut actions_widget_state = ListState::default();

    loop {
        game_state.update();
        
        terminal.draw(|frame| {
            draw(frame, game_state.friend_mut(), &mut actions_widget_state);
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
                                    "Sleep" => game_state.friend_mut().sleep(), // TODO: change background to night
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

fn draw(frame: &mut Frame, friend: &Friend, actions_widget_state: &mut ListState) 
{
    let frame_area = frame.area();
    let [left_area, middle_area, right_area] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(frame_area);
    
    let stats_widget = stats_widget(&left_area, friend);
    for gauge in stats_widget {
        frame.render_widget(gauge.0, gauge.1);
    }
    
    let friend_widget = FriendWidget::new(friend, (0, 0));
    frame.render_widget(friend_widget.get_widget(), middle_area);
    frame.render_stateful_widget(actions_widget(), right_area, actions_widget_state);
}
