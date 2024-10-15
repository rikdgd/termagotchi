mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;
mod widgets;

use ratatui::widgets::canvas::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::utils::ColorWrapper;
use widgets::{friend_widget, stats_widget};

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

    loop {
        game_state.update();
        
        terminal.draw(|mut frame| {
            draw(&mut frame, game_state.friend());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    game_state.store_to_file()?;
    
    Ok(())
}

fn draw<T>(frame: &mut Frame, friend: &Friend<T>) 
    where T: Shape 
{
    let frame_area = frame.area();
    let [left_area, middle_area, right_area] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(frame_area);
    
    let stats_widget = stats_widget(&frame_area, friend);
    for gauge in stats_widget {
        frame.render_widget(gauge.0, gauge.1);
    }
    
    frame.render_widget(friend_widget(), middle_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
