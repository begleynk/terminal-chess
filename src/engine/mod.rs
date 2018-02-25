use game::{GameState};
use board::{Board, Coordinate};
use piece::{Rank};
use Side;
use action::Action;
use piece::Piece;

mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;

pub fn apply_action(action: &Action, state: &mut GameState) -> Result<(), String> {
    match *action {
        Action::MovePiece(piece, from, to) => {
            let possible_moves: Vec<Action> = possible_actions(&from, &state)
                .into_iter()
                .filter(|a| match *a {
                    Action::MovePiece(_, _, _) => true,
                    _ => false,
                })
                .collect();

            // Check this is indeed a valid move
            if let Some(_) = possible_moves
                .into_iter()
                .find(|a| action_matches_coordinate(&a, &to))
            {
                state
                    .update_board(&to, Some(piece.clone()))
                    .expect("Bad move found. Bug");
                state
                    .update_board(&from, None)
                    .expect("Bad move found. Bug");
                state.add_action_to_history(
                    Action::MovePiece(piece.clone(), from.clone(), to.clone()),
                );
                state.toggle_side();

                Ok(())
            } else {
                Err("Invalid move".to_string())
            }
        }
        Action::Capture(capturer, target, from, to) => {
            let possible_captures: Vec<Action> =
                possible_actions(&from, &state)
                    .into_iter()
                    .filter(|a| match *a {
                        Action::Capture(_, _, _, _) => true,
                        _ => false,
                    })
                    .collect();

            if let Some(_) = possible_captures
                .into_iter()
                .find(|a| action_matches_coordinate(&a, &to))
            {
                state
                    .update_board(&to, Some(capturer.clone()))
                    .expect("Bad move found. Bug");
                state
                    .update_board(&from, None)
                    .expect("Bad move found. Bug");
                state.add_piece_to_capture_list(target.clone());
                state.add_action_to_history(Action::Capture(
                    capturer.clone(),
                    target.clone(),
                    from.clone(),
                    to.clone(),
                ));
                state.toggle_side();

                Ok(())
            } else {
                Err("Invalid capture".to_string())
            }
        }
        _ => unimplemented!(),
    }
}

pub fn possible_actions(
    from: &Coordinate,
    state: &GameState,
) -> Vec<Action> {
    if let &Some(piece) = state.piece_at(*from) {
        match piece.rank() {
            Rank::Pawn => pawn::possible_actions(from, state),
            Rank::Knight => knight::possible_actions(from, state),
            Rank::Bishop => bishop::possible_actions(from, state),
            Rank::Rook => rook::possible_actions(from, state),
            Rank::Queen => queen::possible_actions(from, state),
            Rank::King => king::possible_actions(from, state)
        }
    } else {
        vec![]
    }
}

fn action_matches_coordinate(action: &Action, coord: &Coordinate) -> bool {
    match *action {
        Action::MovePiece(_, __, to) => *coord == to,
        Action::Capture(_, _, _, to) => *coord == to,
        Action::Promotion(_, _, _, to) => *coord == to,
    }
}

struct Mover {
    side: Side,
    current_row: i8,
    current_column: i8,
}

impl Mover {
    fn new(side: Side) -> Mover {
        Mover {
            side: side,
            current_row: 0,
            current_column: 0,
        }
    }

    fn north(mut self) -> Self {
        match self.side {
            Side::White => self.current_row += 1,
            Side::Black => self.current_row -= 1,
        }

        self
    }

    fn south(mut self) -> Self {
        match self.side {
            Side::White => self.current_row -= 1,
            Side::Black => self.current_row += 1,
        }

        self
    }

    fn west(mut self) -> Self {
        match self.side {
            Side::White => self.current_column -= 1,
            Side::Black => self.current_column += 1,
        }

        self
    }

    fn east(mut self) -> Self {
        match self.side {
            Side::White => self.current_column += 1,
            Side::Black => self.current_column -= 1,
        }

        self
    }

    fn move_to(mut self, coordinate: &Coordinate) -> Self {
        self.current_column = coordinate.column() as i8;
        self.current_row = coordinate.row() as i8;

        self
    }

    fn make(self) -> Result<Coordinate, String> {
        Coordinate::new_safe(self.current_row as usize, self.current_column as usize)
    }
}

// Takes a closure that drives a mover, moving in a particular direction
// until it hits another piece, or the edge of the board.
fn find_moves_in_direction<F>(
    starting_coordinate: &Coordinate,
    side: Side,
    board: &Board,
    closure: F,
) -> Vec<Coordinate>
where
    F: Fn(Mover) -> Mover,
{
    let mut accumulator = vec![];
    let mut current_coord = starting_coordinate.clone();
    for _ in 0..7 {
        let mut mover = Mover::new(side).move_to(&current_coord);
        mover = closure(mover);

        if let Ok(next) = mover.make() {
            if board.is_empty(next) {
                accumulator.push(next);
                current_coord = next;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    accumulator
}

fn find_opposing_piece_in_direction<F>(
    starting_coordinate: &Coordinate,
    side: Side,
    board: &Board,
    closure: F,
) -> Option<Coordinate>
where
    F: Fn(Mover) -> Mover,
{
    let mut current_coord = starting_coordinate.clone();
    let mut result = None;
    for _ in 0..7 {
        let mut mover = Mover::new(side).move_to(&current_coord);
        mover = closure(mover);

        if let Ok(next) = mover.make() {
            if let &Some(piece) = board.piece_at(next) {
                if piece.side() != side {
                    result = Some(next);
                    break;
                } else {
                    break;
                }
            }
            current_coord = next;
        } else {
            result = None;
        }
    }

    result
}

pub fn is_in_check(state: &GameState) -> bool {
    let my_king = Piece::pack(state.next_to_move(), Rank::King);
    let all_my_king_coordinates = state.board().find_pieces(my_king);

    let king_coordinate = all_my_king_coordinates.get(0).expect("No king on the board");

    state.board().pieces_with_coordinates()
        .into_iter()
        .filter(|&(_coordinate, piece)| piece.side() != state.next_to_move())
        .flat_map(|(coordinate, _piece)| possible_actions(&coordinate, &state.peek_into_the_future()))
        .any(|action| action_matches_coordinate(&action, &king_coordinate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_detect_check() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        state.update_board(&coord!("e4"), Some(Piece::pack(Side::Black, Rank::Queen))).unwrap();

        assert!(is_in_check(&state));
    }

    #[test]
    fn can_detect_when_not_in_check() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        state.update_board(&coord!("a1"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();

        assert!(!is_in_check(&state));
    }

    #[test]
    fn dis_bug() {
        use game::Game;

        let mut game = Game::new();

        game.advance(Action::MovePiece(
            Piece::pack(Side::White, Rank::Pawn),
            Coordinate::from_human("e2".to_owned()).unwrap(),
            Coordinate::from_human("e3".to_owned()).unwrap(),
        )).unwrap();

        game.advance(Action::MovePiece(
            Piece::pack(Side::Black, Rank::Pawn),
            Coordinate::from_human("d7".to_owned()).unwrap(),
            Coordinate::from_human("d6".to_owned()).unwrap(),
        )).unwrap();

        game.advance(Action::MovePiece(
            Piece::pack(Side::White, Rank::Bishop),
            Coordinate::from_human("f1".to_owned()).unwrap(),
            Coordinate::from_human("b5".to_owned()).unwrap(),
        )).unwrap();

        assert!(is_in_check(game.state()));
    }
}
