extern crate termion;

use {Side};
use session::{Session, SessionState};
use std::io::{Write};
use board::Coordinate;
use piece::{Piece, Rank};
use action::Action;

use termion::{color, style};
use ::std::io::Result;

type Out = termion::raw::RawTerminal<::std::io::Stdout>;

static SIDE_BUFFER : &str = "     ";

pub fn draw(session: &Session, out: &mut Out) -> Result<()> {
    write!(out, "{}", termion::cursor::Goto(1, 1))?;
    clear(out)?;

    write!(out, "\n\n{}", SIDE_BUFFER)?;
    if session.game().current_turn() == Side::White {
        write!(out, "To Act: {}White{}\n\r", style::Bold, style::Reset)?;
    } else {
        write!(out, "To Act: {}Black{}\n\r", style::Bold, style::Reset)?;
    }
    write!(out, "{}", SIDE_BUFFER)?;
    write!(out, "Cursor at: {}, {} | Piece: {:?}\n\r", session.cursor().row(), session.cursor().column(), session.game().state().piece_at(Coordinate::new(session.cursor().row(), session.cursor().column())))?;
    draw_table(session, out)?;
    write!(out, "\n\n\r")?;
    write!(out, "{}", termion::cursor::Goto(1, 1))?;

    Ok(())
}

fn draw_table(session: &Session, out: &mut Out) -> Result<()> {
    for (index, row) in row_iterator(session).enumerate() {
        draw_row(row, index, session, out)?;
    }
    Ok(())
}

pub fn clear(out: &mut Out) -> Result<()> {
    write!(out, "{}", termion::clear::All)
}

fn draw_row(row: &[Option<Piece>; 8], row_index: usize, session: &Session, out: &mut Out) -> Result<()> {
    // First row
    write!(out, "{}", SIDE_BUFFER)?;
    for (col_index, _square) in column_iterator(session, row).enumerate() {
        draw_square_padding(out, row_index, col_index, session)?;
    }
    write!(out, "\n\r")?;

    // Second row with the character
    write!(out, "{}", SIDE_BUFFER)?;
    for (col_index, square) in column_iterator(session, row).enumerate() {
        draw_square_with_piece(square, out, row_index, col_index, session)?;
    }
    write!(out, "\n\r")?;

    // Third row, same as above (ish)
    write!(out, "{}", SIDE_BUFFER)?;
    for (col_index, _square) in column_iterator(session, row).enumerate() {
        draw_square_padding(out, row_index, col_index, session)?;
    }
    write!(out, "\n\r")
}

fn draw_square_padding(out: &mut Out, row_index: usize, col_index: usize, session: &Session) -> Result<()> {
    if is_under_cursor(session, row_index, col_index) {
        write!(out, "{}       {}", color::Bg(color::Rgb(0,100,100)), color::Bg(color::Reset))
    } else if is_possible_action(session, row_index, col_index) {
        write!(out, "{}       {}", color::Bg(color::Rgb(010,000,100)), color::Bg(color::Reset))
    } else if is_chosen_square(session, row_index, col_index) {
        write!(out, "{}       {}", color::Bg(color::Rgb(100,000,000)), color::Bg(color::Reset))
    } else {
        if (col_index + row_index) % 2 == 0 {
            write!(out, "{}       {}", color::Bg(color::Rgb(226,226,226)), color::Bg(color::Reset))
        } else {
            write!(out, "{}       {}", color::Bg(color::Rgb(190,190,190)), color::Bg(color::Reset))
        }
    }
}

fn draw_square_with_piece(square: &Option<Piece>, out: &mut Out, row_index: usize, col_index: usize, session: &Session) -> Result<()> {
    if is_under_cursor(session, row_index, col_index) {
        write!(out, "{}   {}   {}", color::Bg(color::Rgb(0,100,100)), format_piece(square), color::Bg(color::Reset))
    } else if is_possible_action(session, row_index, col_index) {
        write!(out, "{}   {}   {}", color::Bg(color::Rgb(010,000,100)), format_piece(square), color::Bg(color::Reset))
    } else if is_chosen_square(session, row_index, col_index) {
        write!(out, "{}   {}   {}", color::Bg(color::Rgb(100,000,000)), format_piece(square), color::Bg(color::Reset))
    } else {
        if (col_index + row_index) % 2 == 0 {
            write!(out, "{}   {}   {}", color::Bg(color::Rgb(226,226,226)), format_piece(square), color::Bg(color::Reset))
        } else {
            write!(out, "{}   {}   {}", color::Bg(color::Rgb(190,190,190)), format_piece(square), color::Bg(color::Reset))
        }
    }
}

fn is_under_cursor(session: &Session, row_index: usize, col_index: usize) -> bool {
    matches_coordinate(session, &session.cursor().to_coord(), row_index, col_index)
}

fn is_possible_action(session: &Session, row_index: usize, col_index: usize) -> bool {
    match session.state() {
        &SessionState::CoordinateSelected(_, ref actions) => actions.into_iter().any(|action|
            match *action {
                Action::MovePiece(_,_,ref to) => matches_coordinate(session, to, row_index, col_index),
                Action::Capture(_,_,_,ref to) => matches_coordinate(session, to, row_index, col_index),
                _ => false
            }
        ),
        _ => false
    }
}

fn is_chosen_square(session: &Session, row_index: usize, col_index: usize) -> bool {
    match *session.state() {
        SessionState::CoordinateSelected(ref coord, _) => matches_coordinate(session, coord, row_index, col_index),
        _ => false
    }
}

fn matches_coordinate(session: &Session, coord: &Coordinate, row_index: usize, col_index: usize) -> bool {
    match session.player_as() {
        Side::Black => coord.row() == row_index && coord.column() == 7 - col_index,
        Side::White => coord.row() == 7 - row_index && coord.column() == col_index,
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

fn row_iterator<'a>(session: &'a Session) -> Box<Iterator<Item = &'a[Option<Piece>; 8]> + 'a> {
    match session.player_as() {
        Side::White => Box::new(session.game().board().rows().into_iter().rev()),
        Side::Black => Box::new(session.game().board().rows().into_iter())
    }
}

fn column_iterator<'a>(session: &Session, row: &'a [Option<Piece>; 8]) -> Box<Iterator<Item = &'a Option<Piece>> + 'a> {
    match session.player_as() {
        Side::White => Box::new(row.into_iter()),
        Side::Black => Box::new(row.into_iter().rev())
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