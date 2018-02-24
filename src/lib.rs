extern crate termion;
mod piece;
mod game;
mod board;
mod engine;
mod ui;
mod session;
mod action;

use std::fmt;
use session::Session;

pub fn new_session() -> Session {
    Session::new()
}

#[derive(PartialEq, Copy, Clone)]
pub enum Side {
    White,
    Black,
}

impl fmt::Debug for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::White => write!(f, "{}", "W"),
            Side::Black => write!(f, "{}", "B"),
        }
    }
}
