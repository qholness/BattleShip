extern crate colour;

use colour::*;
use super::status::ElementStatus;
use std::fmt;
use std::cmp::PartialEq;
use std::io::{self, Write};
use std::convert::TryInto;


#[derive(fmt::Debug, PartialEq)]
pub struct GridElement {
    pub status: ElementStatus
}


#[derive(fmt::Debug)]
pub struct Grid {
    pub n: i32,   
    pub m: i32,
    pub grid: Vec<Vec<GridElement>>
}

impl Grid {
    fn __initialize_grid__(n: &i32, m: &i32) -> Vec<Vec<GridElement>> {
        let mut _vec = Vec::new();
        for _n in 0..*n {
            let mut _sub_vec = Vec::new();
            for _m in 0..*m {
                _sub_vec.push(GridElement{status: ElementStatus::O});
            }
            _vec.push(_sub_vec);
        }
        _vec
    }
    pub fn new(n: i32, m: i32) -> Grid {
        // Initialize grid
        Grid {
            n: n,
            m: m,
            grid: Grid::__initialize_grid__(&n, &m)
        }
    }

    pub fn update_grid(&mut self, _how: String, x: i32, y: i32) {
        match _how.as_str() {
            "hit" => {
                for i in 0..self.grid.len() {
                    if i == x.try_into().unwrap() {
                        for k in 0..self.grid[i].len() {
                            if k == y.try_into().unwrap() {
                                self.hit(i, k);
                                self.print_grid();
                            }
                        }
                    }
                }
            },
            "miss" => {
                for i in 0..self.grid.len() {
                    if i == x.try_into().unwrap() {
                        for k in 0..self.grid[i].len() {
                            if k == y.try_into().unwrap() {
                                self.miss(i, k);
                                self.print_grid();
                            }
                        }
                    }
                }
            }
            _ => ()
        };
        // match self.validate_coordinates(&x, &y) { 
        //     true => {
        //     },
        //     false => println!("Invalid reference coordinates")
        // }
    }

    pub fn fetch_position_value(&mut self, x: usize, y: usize) -> &GridElement {
        &self.grid[x][y]
    }

    fn fetch_position(&mut self, x: usize, y: usize) -> &mut GridElement {
        // Fetch position from grid
        &mut self.grid[x][y]
        // &mut self.grid.iter().position(|&r| ) [usize::from(y)]
    }

    fn hit(&mut self, x: usize, y: usize) {
        // Update Grid coordinate as HIT
        let _grid_item = self.fetch_position(x, y);
        _grid_item.status = ElementStatus::X;
    }

    fn miss(&mut self, x: usize, y: usize) {
        // Update Grid coordinate as MISS
        let _grid_item = self.fetch_position(x, y);
        _grid_item.status = ElementStatus::M;
    }

    pub fn display_grid(&self) -> String {
        // Display the grid with X, M, O in a (n x m) matrix representation
        let mut _output = String::new();
        for _n in 0..self.grid.len() {
            for _m in 0..self.grid[_n].len() {
                _output.push_str(&format!(" {} ", &self.grid[_n][_m].status.to_string()))
            }
            _output.push('\n');
        }
        _output
    }

    pub fn print_grid(&self) {
        for elem in self.display_grid().chars() {
            match elem {
                'X' => {
                    green!("{}", ElementStatus::X);
                },
                'M' => {
                    red!("{}", ElementStatus::M);
                },
                'O' => {
                    print!("{}", ElementStatus::O);
                },
                _ => {
                    print!("{}", elem);
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_grid())
    }
}