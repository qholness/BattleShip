// use druid::{Data, Lens};
use std::fmt;
use std::cmp::PartialEq;



#[derive(fmt::Debug, PartialEq)]
pub struct Player {
    // Struct for attributes of the player
    pub name: String,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".into(),
        }
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


// #[derive(Copy)]
pub struct PlayerVec {
    // Tuple containing players of the game for easy access
    pub players: Vec<Player>,
}

impl fmt::Display for PlayerVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", &self.players[0], &self.players[1])
    }
}