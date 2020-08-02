// use super::super::*;
use super::super::grid::Grid;
use super::super::game_state::GameState;
use super::super::player::{Player, PlayerVec};

pub fn test_game() -> GameState {
    let _players = vec![
        Player{name: "P1".into()},
        Player{name: "P2".into()}
    ];
    GameState {
        id: "Test Game".into(),
        grid: Grid::new(5, 5),
        players: PlayerVec {
            players: _players
        },
        _continue: true,
        _cur_player: 0,
        ships: vec![]
    }
}