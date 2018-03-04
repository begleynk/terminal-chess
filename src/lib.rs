#![cfg_attr(feature = "strict", deny(warnings))]

extern crate termion;
mod piece;
mod game;
mod board;
mod engine;
mod ui;
mod session;
mod action;
mod ai;

use std::fmt;
use session::Session;

use std::ops::Not;

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

impl Not for Side {
    type Output = Side;

    fn not(self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
