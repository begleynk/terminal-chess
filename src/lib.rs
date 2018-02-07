pub mod piece;
pub mod game;
pub mod board;
pub mod engine;

use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Side {
    White,
    Black
}

impl fmt::Debug for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::White => write!(f, "{}", "W"),
            Side::Black => write!(f, "{}", "B")
        }
    }
}