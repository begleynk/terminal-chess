use piece::{Piece, Rank};
use Side;
use std::fmt;
use board::Board;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn from_human(string: String) -> Result<Coordinate, String> {
        assert!(string.len() == 2);
        let mut chars = string.chars();

        let column = match chars.next().unwrap().to_string().as_ref() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => return Err(format!("Bad coordinate {}", string)),
        };

        let row = match chars.next().unwrap().to_string().parse::<usize>() {
            Ok(r) => r - 1,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Coordinate { row, column })
    }

    fn row(&self) -> usize {
        self.row
    }

    fn column(&self) -> usize {
        self.column
    }

    fn to_human(&self) -> String {
        let col = match self.column {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!(format!("Bad column index in coordinates: {}", self.column)),
        };

        format!("{}{}", col, self.row + 1)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    MovePiece(Side, Piece, Coordinate, Coordinate),
    Capture(Side, Piece, Coordinate),
    Promote(Side, Piece, Piece, Coordinate),
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
            board: Board::default(),
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

    fn advance(&mut self, the_move: Move) -> Result<(), String> {
        Ok(())
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

    #[test]
    fn coordinates_can_be_created_from_human_readable_strings() {
        // Note we store 0-indexed

        let coord = Coordinate::from_human("b5".to_owned()).unwrap();
        assert_eq!(coord.row(), 4, "Incorrect row index parserd");
        assert_eq!(coord.column(), 1, "Incorrect column index parserd");

        let coord = Coordinate::from_human("a1".to_owned()).unwrap();
        assert_eq!(coord.row(), 0, "Incorrect row index parserd");
        assert_eq!(coord.column(), 0, "Incorrect column index parserd");

        let coord = Coordinate::from_human("h8".to_owned()).unwrap();
        assert_eq!(coord.row(), 7, "Incorrect row index parserd");
        assert_eq!(coord.column(), 7, "Incorrect column index parserd");
    }

    #[test]
    fn game_updates_its_state_with_moves() {
        let mut game = Game::new();
        let the_move = Move::MovePiece(
            Side::White,
            Piece::pack(Side::White, Rank::Pawn),
            Coordinate::from_human("a2".to_owned()).unwrap(),
            Coordinate::from_human("a4".to_owned()).unwrap(),
        );

        game.advance(the_move.clone());

        assert_eq!(game.history(), &vec![the_move]);
        assert_eq!(game.current_turn(), Side::Black);

        let mut expected_board = Board::default();
        expected_board.update(1,1, None);
        expected_board.update(1,3, Some(Piece::pack(Side::White, Rank::Pawn)));

        assert_eq!(*game.board(), expected_board);
    }
}
