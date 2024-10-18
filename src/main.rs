mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;
mod widgets;

use std::time::Duration;
use ratatui::widgets::canvas::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, poll},
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use ratatui::widgets::ListState;
use shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::utils::ColorWrapper;
use widgets::{friend_widget, stats_widget, actions_widget};

fn main() -> std::io::Result<()> {
    let mut game_state = match GameState::file_exists() {
        true => {
            GameState::read_from_file()?
        },
        false => {
            // TODO: Randomize shape and color.
            let friend = Friend::new(
                "Waldo", 
                CreatureShapes::Duck, 
                ColorWrapper::Cyan
            );
            GameState::new(friend)
        }
    };
    
    let mut terminal = ratatui::init();
    let mut actions_widget_state = ListState::default();

    loop {
        game_state.update();
        
        terminal.draw(|frame| {
            draw(frame, game_state.friend(), &mut actions_widget_state);
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
                                // TODO: update friends state accordingly
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

fn draw<T>(frame: &mut Frame, friend: &Friend<T>, actions_widget_state: &mut ListState) 
    where T: Shape 
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
        
    frame.render_widget(friend_widget(), middle_area);
    frame.render_stateful_widget(actions_widget(), right_area, actions_widget_state);
}
