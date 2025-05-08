use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{poll, Event, KeyCode};
use ratatui::DefaultTerminal;
use crate::game_state::GameState;

/// This function renders the *"Raindrop minigame"* onto the terminal and handles the user input to 
/// allow the user to actually play it.
/// ## parameters:
/// * `terminal` - The terminal to render the raindrop game onto.
/// * `state` - A mutable reference to the current game's state.
pub fn raindrop_minigame_layout(terminal: &mut DefaultTerminal, state: &mut GameState) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let frame_area = frame.area();
            // TODO: Actually render and play the game.
        })?;
        
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }
    }
    
    Ok(())
}
