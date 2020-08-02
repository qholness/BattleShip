use super::game_state::{GameState, ShipSet};
use super::ship::BattleShip;
use super::ship::Ship;
use super::grid::Grid;
use super::player::{Player, PlayerVec};
use std::io::{self, Write};


pub struct GameLoop {
    state: GameState
}

impl GameLoop {
    pub fn turn(&mut self) {
        // Player takes their turn.
        // Player can
            // Guess a position
            // Check the board
            // Quit
        // TODO:
            // Get user decision input
            // Present player with list of options
            // Numeric identification of choice
        let mut turn_taken = false;
        let options: Vec<&[u8]> = vec![
            "\t1) Guess a coordinate\n".as_bytes(),
            "\t2) Display the board\n".as_bytes(),
            "\t3) Quit\n".as_bytes()
        ];
        let cur_player = &self.state.players.players[usize::from(self.state._cur_player)];
        io::stdout().write(format!("What would you like to do: {}?\n", cur_player).as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdout().write(b"What would you like to do?\n").unwrap();
        io::stdout().flush().unwrap();
        for opt in options {
            io::stdout().write(opt).unwrap();
            io::stdout().flush().unwrap();
        }
        let s: i32 = GameLoop::get_user_input(b"Enter a number: ").parse().unwrap();
        match s {
            1 => {
                let target = 
                    GameLoop::get_number_pair(b"Enter desired target coordinates (separate by as \" \"): ");
                // TODO: Check if hit other player's ship
                match &self.state._cur_player {
                    0 => {
                        match self.state.ships[1].battle_ship.check_hit(vec![target.0, target.1]) {
                            true => {
                                &self.state.grid.update_grid("hit".into(), target.0, target.1);
                                self.state.update_current_player()
                            },
                            false => {
                                &self.state.grid.update_grid("miss".into(), target.0, target.1);
                                self.state.update_current_player()
                            }
                        }
                    }
                    1 => {
                        match self.state.ships[0].battle_ship.check_hit(vec![target.0, target.1]) {
                            true => {
                                &self.state.grid.update_grid("hit".into(), target.0, target.1);
                                self.state.update_current_player()
                            },
                            false => {
                                &self.state.grid.update_grid("miss".into(), target.0, target.1);
                                self.state.update_current_player()
                            }
                        }
                    },
                    _ => ()
                }
            },
            2 => {
                self.state.grid.print_grid();
            },
            3 => {
                std::process::exit(0)
            }
            _ => {
                self.turn()
            }
        }
    }
    pub fn get_user_input(request: &[u8]) -> String {
        // Get and validate input from player
        fn validate(s: &String) -> bool {
            // String validation
            if s.trim().len() > 0 {
                false
            } else {
                true
            }
        }
        let mut s = String::new();
        while validate(&s) {
            // It's here but it's not printing out the initial data?
            io::stdout().write(request).unwrap();
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut s).unwrap();
            // TODO: Validate game name
        }
        s.trim().to_string()
    }

    pub fn get_number_pair(request: &[u8]) -> (i32, i32) {
        let mut s: String;
        let mut _dimensions: Vec<i32>;
        let mut n = 0_i32;
        let mut m = 0_i32;
        fn parse_to_i32(s: &mut String, tgt: &mut Vec<i32>) {
            for _item in s.split(" ") {
                if _item.is_empty() {
                    continue
                }
                let _val: i32 = _item.trim().parse().unwrap();
                tgt.push(_val);
            }
        }
        loop {
            // Reinitialize these every loop
            s = String::new();
            _dimensions = vec![];
            
            io::stdout().write(request).unwrap();
            io::stdout().flush().unwrap();
            
            // Get user input 
            io::stdin().read_line(&mut s).unwrap();

            // Convert to trimmed string
            s = s.to_string().trim().to_string();

            // Split by space and check for i32 values
            parse_to_i32(&mut s, &mut _dimensions);

            match _dimensions.len() {
                2 => {
                    n = _dimensions[0];
                    m = _dimensions[1];
                    break
                },
                x if x > 2 => {
                    io::stdout().write(b"\nOops, we're only playing 2d battleship!\n").unwrap();
                    io::stdout().flush().unwrap();
                },
                x if x < 2  => {
                    io::stdout().write(b"\nNot enough data. Please provide a coordinate like: \"8 0\"!\n").unwrap();
                    io::stdout().flush().unwrap();
                },
                _ => {
                    ()
                }
            }
        }
        (n, m)
    }

    pub fn new() -> Self {
        // Get player input from command line
        println!("Let's setup the game!");
        let _players = vec![
            Player{name: "P1".into()},
            Player{name: "P2".into()}
        ];
        // let mut state = GameState {
        //     id: GameLoop::get_user_input(),
        //     grid: {
        //         let _dims = GameLoop::get_number_pair(b"Enter grid dimensions (separate by as \" \"): ");
        //         Grid::new(_dims.0, _dims.1)
        //     },
        //     players: PlayerVec {
        //         players: _players
        //     },
        //     _continue: true,
        //     _cur_player: 0
        // };

        // Test setup
        let mut state = GameState {
            id: "TestGame".into(),
            grid: Grid::new(10i32, 10i32),
            players: PlayerVec {
                players: _players
            },
            _continue: true,
            _cur_player: 0,
            ships: vec![
                ShipSet {
                    battle_ship: BattleShip::new(),
                },
                ShipSet {
                    battle_ship: BattleShip::new(),
                }
            ]
        };

        io::stdout().write_all(b"Let's place your ships\n").unwrap();
        io::stdout().flush().unwrap();

        GameState::place_ship(&mut state.ships[0].battle_ship);
        state.ships[0].battle_ship.check_hit(vec![0, 0]);

        // Place battleship
        GameLoop {
            state: state
        }
    }

    pub fn game_loop(&mut self) {
        while self.state._continue {
            // Continue game loop
            self.turn();
            // TODO: define win conditions
        }
    }
}
