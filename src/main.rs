mod food;
mod friend;
mod game_state;
mod shapes;
mod utils;
mod widgets;
mod layouts;
mod animations;
mod movements;
mod app;

use crate::app::App;



fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(&mut terminal)?;
    
    app.run(&mut terminal)?;
    app.save_game()?;
    ratatui::restore();
    
    Ok(())
}
