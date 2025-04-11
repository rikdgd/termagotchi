use std::time::Duration;
use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, poll}, layout::{Constraint, Layout}, DefaultTerminal};
use crate::shapes::creatures::CreatureShapes;
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::widgets::new_game_widget::{new_game_dialog, new_game_name_input};

/// Draws the widget that allows the user to create a new `GameState`, used when there is no save file to be found.
/// <br>
/// The widget will tell the user some basics about the application, and prompt them for a name for their first pet.
/// <br>
/// ## parameters:
/// * `terminal` - The `ratatui::DefaultTerminal` to draw the layout onto.
/// 
/// <br>
/// ## returns:
/// A newly generated `GameState`.
pub fn draw_new_game_layout(terminal: &mut DefaultTerminal) -> std::io::Result<GameState> {
    let mut new_name_input = String::new();

    loop {
        terminal.draw(|frame| {
            let frame_area = frame.area();

            let [dialog_area, input_area] = Layout::vertical([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
                .areas(frame_area);

            frame.render_widget(new_game_dialog(), dialog_area);
            frame.render_widget(new_game_name_input(&new_name_input), input_area);
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

    Ok(GameState::new(
        Friend::new(
            &new_name_input,
            CreatureShapes::new_random()
        )
    ))
}
