use super::game_loop::GameLoop;
use super::player::Player;
use std::fmt::{Debug, Display, Formatter, Result};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub struct Ship {
    pub name: String,
    pub length: i32,
    pub hit_vector: Vec<Vec<i32>> // Vector of coordinate pairs
}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.name)
    }
}

impl Ship {
    pub fn update_hit_vector(&mut self, vector: Vec<Vec<i32>>) {
        self.hit_vector = vector;
    }
    pub fn check_hit(&self, hit_vector: Vec<i32>) -> bool {
        // Check if coordinates hit a BattleShip
        for _item in &self.hit_vector {
            if _item[0] == hit_vector[0] {
                if _item[1] == hit_vector[1] {
                    println!("HIT: {}!", &self);
                    return true
                }
            }
        }
        println!("MISS!");
        false
    }
}

// impl Default for Ship<'_> {
//     fn default() -> Self {
//         Ship {
//             name: "Null".to_string(),
//             length: 4,
//             hit_vector: vec![
//                 vec![0, 0],
//                 vec![0, 0],
//                 vec![0, 0],
//                 vec![0, 0]
//             ]
//         }
//     }
// }


pub trait BattleShip {
    fn new() -> Self;
}


impl BattleShip for Ship {
    fn new() -> Self {
        Ship {
            name: "Battleship".to_string(),
            length: 8,
            hit_vector: vec![]}
    }
}
