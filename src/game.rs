use piece::{Piece, Rank};
use Side;
use std::fmt;
use board::Board;

#[derive(PartialEq)]
struct Coordinate {
    row: u8,
    column: u8,
}

#[derive(PartialEq)]
enum Move {
    MovePiece(Piece, Coordinate, Coordinate),
    Capture(Piece, Coordinate),
    Promote(Piece, Piece, Coordinate),
}

#[derive(PartialEq)]
struct Game {
    board: Board,
    current_turn: Side,
    history: Vec<Move>,
}

impl Game {
    fn new() -> Game {
        Game {
            current_turn: Side::White,
            history: vec![],
            board: Board::default()
        }
    }

    fn history(&self) -> &Vec<Move> {
        &self.history
    }

    fn current_turn(&self) -> Side {
        self.current_turn
    }

    fn board(&self) -> &Board {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_the_game() {
        let mut game = Game::new();

        assert_eq!(game.history().len(), 0);
        assert_eq!(game.current_turn(), Side::White);
        assert_eq!(*game.board(), Board::default());
    }
}
