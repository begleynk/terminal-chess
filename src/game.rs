use piece::{Piece, Rank};
use Side;
use board::{Board, Coordinate};
use engine;
use action::Action;

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
            board: Board::default(),
        }
    }

    pub fn with_board(board: Board) -> GameState {
        GameState {
            next_to_move: Side::White,
            history: vec![],
            board: board,
        }
    }

    pub fn history(&self) -> &Vec<Action> {
        &self.history
    }

    pub fn next_to_move(&self) -> Side {
        self.next_to_move
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn piece_at(&self, coord: Coordinate) -> &Option<Piece> {
        &self.board.piece_at(coord)
    }

    pub fn actions_at(&mut self, coordinate: Coordinate) -> Vec<Action> {
        engine::possible_actions(&coordinate, self)
    }

    pub fn has_completed(&mut self) -> bool {
        let side = self.next_to_move();
        engine::is_in_checkmate(self, side)
    }

    pub fn peek_into_the_future(&self) -> GameState {
        GameState {
            next_to_move: !self.next_to_move,
            history: self.history.clone(),
            board: self.board.clone()
        }
    }

    pub fn advance(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::MovePiece(piece, from, to) => {
                self
                    .update_board(&to, Some(piece.clone()))
                    .expect("Bad move found. Bug");
                self
                    .update_board(&from, None)
                    .expect("Bad move found. Bug");
                self.add_action_to_history(
                    Action::MovePiece(piece.clone(), from.clone(), to.clone()),
                    );
                self.toggle_side();

                Ok(())
            }
            Action::Capture(capturer, target, from, to) => {
                self
                    .update_board(&to, Some(capturer.clone()))
                    .expect("Bad move found. Bug");
                self
                    .update_board(&from, None)
                    .expect("Bad move found. Bug");
                self.add_action_to_history(Action::Capture(
                        capturer.clone(),
                        target.clone(),
                        from.clone(),
                        to.clone(),
                        ));
                self.toggle_side();

                Ok(())
            }
            _ => unimplemented!(),
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        let last_action = self.history.pop();
        match last_action {
            Some(Action::MovePiece(piece, from, to)) => {
                self
                    .update_board(&from, Some(piece.clone()))
                    .expect("Bad move found. Bug");
                self
                    .update_board(&to, None)
                    .expect("Bad move found. Bug");
                self.toggle_side();

                Ok(())
            },
            Some(Action::Capture(capturer, target, from, to)) => {
                self
                    .update_board(&from, Some(capturer.clone()))
                    .expect("Bad move found. Bug");
                self
                    .update_board(&to, Some(target.clone()))
                    .expect("Bad move found. Bug");
                self.toggle_side();

                Ok(())
            }
            None => Ok(()),
            _ => unimplemented!(),

        }
    }

    pub fn evaluate_with_action<F, T>(&mut self, action: Action, evaluation_fn: F) -> T where F: Fn(&mut GameState) -> T {
        self.advance(action);
        let evaluation_result = evaluation_fn(self);
        self.undo();
        evaluation_result
    }

    fn update_board(&mut self, coordinate: &Coordinate, piece: Option<Piece>) -> Result<(), String> {
       self.board.update(coordinate, piece)
    }

    fn toggle_side(&mut self) {
        match self.next_to_move {
            Side::White => self.next_to_move = Side::Black,
            Side::Black => self.next_to_move = Side::White,
        }
    }

    fn add_action_to_history(&mut self, action: Action) {
        self.history.push(action);
    }
}

#[derive(PartialEq, Debug)]
pub struct Game {
    current_state: GameState,
}

#[allow(dead_code)] // TODO: Remove
impl Game {
    pub fn new() -> Game {
        Game {
            current_state: GameState::new(),
        }
    }

    pub fn current_turn(&self) -> Side {
        self.current_state.next_to_move
    }

    pub fn history(&self) -> &Vec<Action> {
        self.current_state.history()
    }

    pub fn board(&self) -> &Board {
        &self.current_state.board()
    }

    pub fn state(&self) -> &GameState {
        &self.current_state
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.current_state
    }

    pub fn advance(&mut self, action: Action) -> Result<(), String> {
        self.current_state.advance(action)
    }

    pub fn has_completed(&mut self) -> bool {
        self.current_state.has_completed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Rank;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

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

    #[test]
    fn can_undo_moves() {
        let mut state = GameState::new();

        state.advance(Action::MovePiece(
            Piece::pack(Side::White, Rank::Pawn),
            Coordinate::from_human("e2".to_owned()).unwrap(),
            Coordinate::from_human("e3".to_owned()).unwrap(),
        ));

        state.undo();

        assert_eq!(state, GameState::new());
    }

    #[test]
    fn can_undo_captures() {
        let mut board = Board::empty();
        board.update(&coord!("a2"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("h1"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("h8"), Some(Piece::pack(Side::White, Rank::Queen))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        let state = GameState::with_board(board);

        let mut new_state = state.clone();

        new_state.advance(Action::Capture(
            Piece::pack(Side::White, Rank::Queen),
            Piece::pack(Side::Black, Rank::Rook),
            Coordinate::from_human("h8".to_owned()).unwrap(),
            Coordinate::from_human("a1".to_owned()).unwrap(),
        ));

        new_state.undo();

        assert_eq!(new_state, state);
    }

    #[test]
    fn undoing_without_a_history_is_a_noop() {
        let mut state = GameState::new();

        state.undo();

        assert_eq!(state, GameState::new());
    }
}
