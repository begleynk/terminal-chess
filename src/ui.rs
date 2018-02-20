extern crate termion;

use {Session, Side};
use std::io::{Write};
use board::Coordinate;

type Out = termion::raw::RawTerminal<::std::io::Stdout>;

pub fn draw(session: &Session, out: &mut Out) {
    write!(out, "{}", termion::clear::All);
    write!(out, "{}", termion::cursor::Goto(1, 1));

    write!(out, "Cursor at: {}, {} | Piece: {:?}\n\r", session.cursor().row(), session.cursor().column(), session.game().state().piece_at(Coordinate::new(session.cursor().row(), session.cursor().column())));
    draw_table(session, out);
}

pub fn draw_table(session: &Session, out: &mut Out) {
    for row in session.game().board().rows().into_iter().rev() {
        write!(out, "{:?}\n\r", row);
    }
}

#[derive(PartialEq, Debug)]
pub struct Cursor {
    row: usize,
    column: usize,
    side: Side,
}

impl Cursor {
    pub fn new(side: Side) -> Cursor {
        Cursor {
            row: 0,
            column: 0,
            side: side,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn up(&mut self) {
        match self.side {
            Side::White => { if self.row < 7 { self.row += 1 } },
            Side::Black => { if self.row > 0 { self.row -= 1 } }
        }
    }

    pub fn down(&mut self) {
        match self.side {
            Side::White => { if self.row > 0 { self.row -= 1 } },
            Side::Black => { if self.row < 7 { self.row += 1 } }
        }
    }

    pub fn right(&mut self) {
        match self.side {
            Side::White => { if self.column < 7 { self.column += 1 } },
            Side::Black => { if self.column > 0 { self.column -= 1 } }
        }
    }

    pub fn left(&mut self) {
        match self.side {
            Side::White => { if self.column > 0 { self.column -= 1 } },
            Side::Black => { if self.column < 7 { self.column += 1 } }
        }
    }

    pub fn to_coord(&self) -> Coordinate {
        Coordinate::new(self.row, self.column)
    }
}