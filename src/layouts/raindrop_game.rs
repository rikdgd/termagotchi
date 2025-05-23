use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{poll, Event, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Layout};
use crate::game_state::GameState;
use crate::raindrop_game::game_widget::GameWidgetManager;

/// This function renders the **Raindrop minigame** onto the terminal and handles the user input to 
/// allow the user to actually play it.
/// ## parameters:
/// * `terminal` - The terminal to render the raindrop game onto.
/// * `state` - A mutable reference to the current game's state.
pub fn raindrop_minigame_layout(terminal: &mut DefaultTerminal, state: &mut GameState) -> std::io::Result<()> {
    let mut game_manager: Option<GameWidgetManager> = None;
    
    loop {
        terminal.draw(|frame| {
            let frame_area = frame.area();
            let [game_area, controls_area] = Layout::vertical([
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
                .areas(frame_area);

            if game_manager.is_none() {
                game_manager = Some(GameWidgetManager::new(state.friend(), game_area));
            }
            
            let manager = game_manager.as_mut().unwrap();
            frame.render_widget(manager.get_widget(), game_area);
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
