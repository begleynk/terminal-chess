use piece::{Piece};
use Side;
use board::{Board, Coordinate};
use engine;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    MovePiece(Piece, Coordinate, Coordinate),
    Capture(Piece, Piece, Coordinate),
    Promotion(Piece, Piece, Coordinate),
}

#[derive(PartialEq, Clone, Debug)]
pub struct GameState {
    next_to_move: Side,
    history: Vec<Action>,
    board: Board
}

#[allow(dead_code)] // TODO: Remove
impl GameState {
    pub fn new() -> GameState {
        GameState {
            next_to_move: Side::White,
            history: vec![],
            board: Board::default()
        }
    }

    pub fn history(&self) -> &Vec<Action> {
        &self.history
    }

    pub fn next_to_move(&self) -> Side {
        self.next_to_move.clone()
    }

    pub fn add_action_to_history(&mut self, action: Action) {
        self.history.push(action);
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn update_board(&mut self, coordinate: &Coordinate, piece: Option<Piece>) -> Result<(), String> {
       self.board.update(coordinate, piece) 
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn toggle_side(&mut self) {
        match self.next_to_move {
            Side::White => self.next_to_move = Side::Black,
            Side::Black => self.next_to_move = Side::White,
        }
    }
}

#[derive(PartialEq)]
pub struct Game {
    current_state: GameState,
}

#[allow(dead_code)] // TODO: Remove
impl Game {
    fn new() -> Game {
        Game {
            current_state: GameState::new(),
        }
    }

    fn current_turn(&self) -> Side {
        self.current_state.next_to_move
    }

    pub fn history(&self) -> &Vec<Action> {
        self.current_state.history()
    }

    fn board(&self) -> &Board {
        &self.current_state.board()
    }

    fn advance(&mut self, action: Action) -> Result<(), String> {
        engine::apply_action(&action, &mut self.current_state)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Rank;

    #[test]
    fn it_initializes_the_game() {
        let game = Game::new();

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

    //#[test] TODO: Enable eventually
    #[allow(dead_code)]
    fn game_updates_its_state_with_moves() {
        let mut game = Game::new();
        let action = Action::MovePiece(
            Piece::pack(Side::White, Rank::Pawn),
            Coordinate::from_human("a2".to_owned()).unwrap(),
            Coordinate::from_human("a4".to_owned()).unwrap(),
        );

        game.advance(action.clone()).unwrap();

        assert_eq!(game.history(), &vec![action]);
        assert_eq!(game.current_turn(), Side::Black);

        let mut expected_board = Board::default();
        expected_board.update(&Coordinate::new(1,1), None).unwrap();
        expected_board.update(&Coordinate::new(1,3), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();

        assert_eq!(*game.board(), expected_board);
    }
}
