mod game_lib;
mod music;
use game_lib::{game_loop, game_state};


fn main() -> Result<(), game_state::GameError> {    
    let mut game_loop = game_loop::GameLoop::new();
    game_loop.game_loop();
    Ok(())
}
