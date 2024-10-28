use std::time::Duration;
use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, poll}, layout::{Constraint, Layout}, DefaultTerminal};
use crate::shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::widgets::new_friend_widget::{new_friend_dialog, new_friend_name_input};

/// Draws the widget that allows the user to create a new GameState, for example when their friend has died. <br>
/// Updates the old GameState to the new one using a mutable reference _old_state_.
pub fn draw_new_friend_layout(terminal: &mut DefaultTerminal, old_state: &mut GameState) -> std::io::Result<()> {
    let mut new_name_input = String::new();

    loop {
        terminal.draw(|frame| {
            let frame_area = frame.area();

            let [dialog_area, input_area] = Layout::vertical([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
                .areas(frame_area);

            frame.render_widget(new_friend_dialog(), dialog_area);
            frame.render_widget(new_friend_name_input(&mut new_name_input), input_area);
        })?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(input) => new_name_input.push(input),
                        KeyCode::Backspace => {
                            let _ = new_name_input.remove(new_name_input.len() -1);
                        },

                        KeyCode::Enter => break,
                        _ => (),
                    }
                }
            }
        }
    }

    // Generate a random new friend shape and set the new GameState
    let new_friend_shape = CreatureShapes::new_random();    
    *old_state = GameState::new(
        Friend::new(
            &new_name_input,
            new_friend_shape
        )
    );

    Ok(())
}
