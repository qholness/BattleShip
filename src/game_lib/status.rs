use std::fmt;
use std::cmp::PartialEq;

#[derive(fmt::Debug, PartialEq)]
pub enum ElementStatus {
    X, //HIT
    M, // MISS
    O, // EMPTY
}

impl fmt::Display for ElementStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}