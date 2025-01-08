use ratatui::DefaultTerminal;
use crate::game_state::GameState;

/// This layout is used whenever the users pet has died. It will display a short death message<br>
/// and allow the user to create a new friend. This method will do this by modifying the old<br>
/// game state.
/// 
/// ## paramters:
/// * `terminal` - The `ratatui::DefaultTerminal` to draw the layout onto.
/// * `game_state` - The current `GameState` where the pet has died.
pub fn friend_death_layout(terminal: &mut DefaultTerminal, game_state: &mut GameState) {
    
    
    todo!()
}