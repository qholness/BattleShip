use super::grid::Grid;
use super::player;
use super::ship::Ship;
use std::error::Error;
use std::{fmt, io};
use std::io::{Write};


#[derive(fmt::Debug)]
pub struct GameError;

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game Error.")
    }
}
pub struct ShipSet {
    pub battle_ship: Ship
}

pub struct GameState {
    pub id: String,
    pub grid: Grid,
    pub players: player::PlayerVec,
    pub _continue: bool,
    pub _cur_player: u8,
    pub ships: Vec<ShipSet> // [0, 1] selector
}

impl GameState {
    // TODO: Ship placement
    
    pub fn update_current_player(&mut self) {
        match self._cur_player {
            0 => self._cur_player = 1,
            1 => self._cur_player = 0,
            _ => ()
        }
    }

    pub fn place_ship(ship: &mut Ship) {
        use super::game_loop::GameLoop;
        // Placement function for ship
        // Enter starting point
        let starting_coordinates = 
            GameLoop::get_number_pair(
                format!("Enter starting coordinate points for {}: ", &ship).as_bytes());
        println!("Placing {} at {:?}", &ship, &starting_coordinates);
        //TODO: Validate starting point
        // Get valid ending points
        // Cannot place diagonally
        let mut _valid_ending_coordinates: Vec<Vec<i32>> = Vec::new();
        _valid_ending_coordinates.push(vec![starting_coordinates.0 , starting_coordinates.1 + ship.length]);
        _valid_ending_coordinates.push(vec![starting_coordinates.0 + ship.length, starting_coordinates.1]);
         
        println!("Possible coordinate points for: {}", &ship);
        for _point in &_valid_ending_coordinates {
            println!("{:?}", _point)
        }
        let ending_coordinates = 
            GameLoop::get_number_pair(
                format!("Enter ending coordinate points for {}: ", &ship).as_bytes());
        // Select end point (offer up potential input coordinates)
        let mut _hit_vector: Vec<Vec<i32>> = Vec::new();
        io::stdout().write_all(format!("End coordinates: {:?}\n", _valid_ending_coordinates[1]).as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        if starting_coordinates.0 == ending_coordinates.0 {
            //  Ending point is second pair of coordinates
            for a in starting_coordinates.1..ending_coordinates.1 {
               _hit_vector.push(vec![starting_coordinates.0, a]) 
            }
            ship.update_hit_vector( _hit_vector );
        } else {
            for a in starting_coordinates.0..ending_coordinates.0 {
                _hit_vector.push(vec![a, starting_coordinates.1]) 
             }
            ship.update_hit_vector( _hit_vector );
        }
        println!("Ship hit vector: {:?}", ship.hit_vector);
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameState: {{\n\tid: {},\n\tdimensions: ({}, {})\n}}",
            &self.id,
            &self.grid.n,
            &self.grid.m,
        )
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            id: "DefaultGame".into(),
            grid: Grid::new(8i32, 8i32),
            players: player::PlayerVec {
                players: vec![
                    player::Player{name: "P1".into()},
                    player::Player{name: "P2".into()}
                ]
            },
            _continue: true,
            _cur_player: 0,
            ships: vec![]
        }
    }
}




// impl GameState {
//     fn 
// }