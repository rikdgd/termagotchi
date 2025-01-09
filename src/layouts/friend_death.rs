use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{poll, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Layout};
use crate::friend::Friend;
use crate::game_state::GameState;
use crate::shapes::creatures::CreatureShapes;
use crate::widgets::friend_death_widget::death_canvas;
use crate::widgets::new_friend_widget::new_friend_name_input;

/// This layout is used whenever the users pet has died. It will display a short death message<br>
/// and allow the user to create a new pet. This method will do this by modifying the old<br>
/// game state.
/// <br>
/// ## paramters:
/// * `terminal` - The `ratatui::DefaultTerminal` to draw the layout onto.
/// * `game_state` - The current `GameState` where the pet has died.
pub fn friend_death_layout(terminal: &mut DefaultTerminal, game_state: &mut GameState) -> std::io::Result<()> {
    let mut name_buffer = String::new();
    loop {
        terminal.draw(|frame| {
            let frame_area = frame.area();

            let [canvas_area, input_area] = Layout::vertical([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
                .areas(frame_area);
            
            
            frame.render_widget(death_canvas(), canvas_area);
            frame.render_widget(new_friend_name_input(&mut name_buffer), input_area);
        })?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(input) => name_buffer.push(input),
                        KeyCode::Backspace => {
                            let _ = name_buffer.remove(name_buffer.len() -1);
                        },

                        KeyCode::Enter => break,
                        _ => (),
                    }
                }
            }
        }
    }
    
    // Adjust the GameState
    *game_state = GameState::new(Friend::new(&name_buffer, CreatureShapes::new_random()));
    
    Ok(())
}
