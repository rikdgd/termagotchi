mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;

use ratatui::widgets::canvas::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::Color,
    widgets::Block,
    Frame,
};
use shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;

fn main() -> std::io::Result<()> {
    let game_state = match GameState::file_exists() {
        true => {
            GameState::read_from_file()?
        },
        false => {
            // TODO: 'Randomly' select the creatures shape and color.
            let friend = Friend::new("waldo", CreatureShapes::Duck);
            GameState::new(friend)
        }
    };
    
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|mut frame| {
            draw(&mut frame);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let [left_area, middle_area, right_area] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(frame.area());

    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(friend_widget(), middle_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}

fn friend_widget() -> Canvas<'static, fn(&mut Context)> {
    let friend_widget_x_bounds = [-180.0, 180.0];
    let friend_widget_y_bounds = [-90.0, 90.0];

    Canvas::default()
        .block(Block::bordered().title("Friend"))
        .x_bounds(friend_widget_x_bounds)
        .y_bounds(friend_widget_y_bounds)
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();
            ctx.draw(&Line {
                x1: 0.0,
                y1: 10.0,
                x2: 10.0,
                y2: 10.0,
                color: Color::White,
            });
            ctx.draw(&Rectangle {
                x: 10.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
                color: Color::Red,
            });
            ctx.draw(&CreatureShapes::Duck)
        })
}
