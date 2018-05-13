#![cfg_attr(feature = "strict", deny(warnings))]

extern crate termion;
extern crate tokio;
extern crate bytes;
#[macro_use]
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod piece;
mod game;
mod board;
mod engine;
mod ui;
mod session;
mod action;
mod ai;
mod protocol;
pub mod server;

use std::fmt;
use session::Session;

use std::ops::Not;

pub fn new_session() -> Session {
    Session::new()
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
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
