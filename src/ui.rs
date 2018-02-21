extern crate termion;

use {Session, Side};
use std::io::{Write};
use board::Coordinate;
use piece::{Piece, Rank};

use termion::color;

type Out = termion::raw::RawTerminal<::std::io::Stdout>;

static SIDE_BUFFER : &str = "     ";

pub fn draw(session: &Session, out: &mut Out) {
    write!(out, "{}", termion::cursor::Goto(1, 1));

    write!(out, "\n\n{}", SIDE_BUFFER);
    write!(out, "Cursor at: {}, {} | Piece: {:?}\n\r", session.cursor().row(), session.cursor().column(), session.game().state().piece_at(Coordinate::new(session.cursor().row(), session.cursor().column())));
    draw_table(session, out);
    write!(out, "\n\n\r");
    write!(out, "{}", termion::cursor::Goto(1, 1));
}

fn draw_table(session: &Session, out: &mut Out) {
    for (index, row) in session.game().board().rows().into_iter().rev().enumerate() {
        draw_row(row, index, session, out);
    }
}

pub fn clear(out: &mut Out) {
    write!(out, "{}", termion::clear::All);
}

fn draw_row(row: &[Option<Piece>; 8], row_index: usize, session: &Session, out: &mut Out) {
    // First row
    write!(out, "{}", SIDE_BUFFER);
    for (col_index, square) in row.into_iter().enumerate() {
        draw_square_padding(out, row_index, col_index, session);
    }
    write!(out, "\n\r");

    // Second row with the character
    write!(out, "{}", SIDE_BUFFER);
    for (col_index, square) in row.into_iter().enumerate() {
        draw_square_with_piece(square, out, row_index, col_index, session);
    }
    write!(out, "\n\r");

    // Third row, same as above (ish)
    write!(out, "{}", SIDE_BUFFER);
    for (col_index, square) in row.into_iter().enumerate() {
        draw_square_padding(out, row_index, col_index, session);
    }
    write!(out, "\n\r");
}

fn cursor_on_square(session: &Session, row_index: usize, col_index: usize) -> bool {
    match session.player_as() {
        Side::Black => session.cursor().row() == row_index && session.cursor().column() == col_index,
        Side::White => session.cursor().row() == 7 - row_index && session.cursor().column() == col_index,
    }
}

fn draw_square_padding(out: &mut Out, row_index: usize, col_index: usize, session: &Session) {
    if cursor_on_square(session, row_index, col_index) {
        write!(out, "{}       {}", color::Bg(color::Rgb(0,100,100)), color::Bg(color::Reset));
    } else {
        if (col_index + row_index) % 2 == 0 {
            write!(out, "{}       {}", color::Bg(color::Rgb(226,226,226)), color::Bg(color::Reset));
        } else {
            write!(out, "{}       {}", color::Bg(color::Rgb(190,190,190)), color::Bg(color::Reset));
        }
    }
}

fn draw_square_with_piece(square: &Option<Piece>, out: &mut Out, row_index: usize, col_index: usize, session: &Session) {
    if cursor_on_square(session, row_index, col_index) {
        write!(out, "{}   {}   {}", color::Bg(color::Rgb(0,100,100)), format_piece(square), color::Bg(color::Reset));
    } else {
        if (col_index + row_index) % 2 == 0 {
            write!(out, "{}   {}   {}", color::Bg(color::Rgb(226,226,226)), format_piece(square), color::Bg(color::Reset));
        } else {
            write!(out, "{}   {}   {}", color::Bg(color::Rgb(190,190,190)), format_piece(square), color::Bg(color::Reset));
        }
    }
}

fn format_piece(piece: &Option<Piece>) -> &str {
    if let &Some(p) = piece {
        match (p.side(), p.rank()) {
            (Side::White, Rank::Pawn) => "♙",
            (Side::White, Rank::Knight) => "♘",
            (Side::White, Rank::Bishop) => "♗",
            (Side::White, Rank::Rook) => "♖",
            (Side::White, Rank::Queen) => "♕",
            (Side::White, Rank::King) => "♔",
            (Side::Black, Rank::Pawn) => "♟",
            (Side::Black, Rank::Knight) => "♞",
            (Side::Black, Rank::Bishop) => "♝",
            (Side::Black, Rank::Rook) => "♜",
            (Side::Black, Rank::Queen) => "♛",
            (Side::Black, Rank::King) => "♚",
        }
    } else {
        " "
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