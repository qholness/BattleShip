pub mod game_loop;
pub mod game_state;
pub mod grid;
pub mod player;
pub mod ship;
pub mod status;
pub mod tests;

pub mod default {
    use std::fmt;

    #[derive(fmt::Debug)]
    pub struct DefaultDisplay;

    impl fmt::Display for DefaultDisplay {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}

