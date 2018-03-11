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

pub fn possible_actions(
    from: &Coordinate,
    state: &mut GameState,
) -> Vec<Action> {
    enumerate_all_actions(from, state).into_iter()
        .filter(|action| leads_out_of_check(action, state))
        .collect()
}

pub fn enumerate_all_actions(
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

fn leads_out_of_check(action: &Action, state: &mut GameState) -> bool {
    let next_to_move = state.next_to_move();
    state.evaluate_with_action(action.clone(), |new_state| !is_in_check(&new_state, next_to_move))
}

pub fn opponent_can_capture(coord: &Coordinate, my_side: Side, state: &GameState) -> bool {
    state.board().pieces_with_coordinates()
        .into_iter()
        .filter(|&(_coordinate, piece)| piece.side() != my_side)
        .flat_map(|(coordinate, _piece)| enumerate_all_actions(&coordinate, &state))
        .any(|action| action_matches_coordinate(&action, coord))
}

pub fn is_in_check(state: &GameState, side: Side) -> bool {
    let my_king = Piece::pack(side, Rank::King);
    let all_my_king_coordinates = state.board().find_pieces(my_king);

    let king_coordinate = all_my_king_coordinates.get(0).expect("No king on the board");

    opponent_can_capture(&king_coordinate, side, state)
}

pub fn is_in_checkmate(state: &mut GameState, side: Side) -> bool {
    let my_coords_and_pieces = state.board().pieces_with_coordinates()
                                .into_iter()
                                .filter(|&(_coordinate, piece)| piece.side() == side)
                                .collect::<Vec<(Coordinate, Piece)>>();

    my_coords_and_pieces
        .into_iter()
        .all(|(coordinate, _piece)| possible_actions(&coordinate, state).is_empty())
}

fn action_matches_coordinate(action: &Action, coord: &Coordinate) -> bool {
    match *action {
        Action::MovePiece(_, __, to) => *coord == to,
        Action::Capture(_, _, _, to) => *coord == to,
        _ => false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_detect_check() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("e4"), Some(Piece::pack(Side::Black, Rank::Queen))).unwrap();
        let mut state = GameState::with_board(board);

        assert!(is_in_check(&mut state, Side::White));
    }

    #[test]
    fn can_detect_when_not_in_check() {
        let mut board = Board::empty();

        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();

        let mut state = GameState::with_board(board);

        assert!(!is_in_check(&mut state, Side::White));
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

        assert!(is_in_check(game.state(), Side::Black));
    }

    #[test]
    fn prunes_moves_that_do_not_get_the_player_out_of_check() {
        let mut board = Board::empty();

        board.update(&coord!("a2"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("h1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("h8"), Some(Piece::pack(Side::Black, Rank::Queen))).unwrap();
        board.update(&coord!("g1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();

        let mut state = GameState::with_board(board);

        assert_eq!(possible_actions(&coord!("h1"), &mut state), vec![
          Action::MovePiece(
            Piece::pack(Side::White, Rank::King),
            Coordinate::from_human("h1".to_owned()).unwrap(),
            Coordinate::from_human("g2".to_owned()).unwrap()
          )
        ]);
    }

    #[test]
    fn detects_checkmate() {
        let mut board = Board::empty();

        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("h1"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("b2"), Some(Piece::pack(Side::Black, Rank::Queen))).unwrap();
        board.update(&coord!("d2"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();

        let mut state = GameState::with_board(board);

        assert!(is_in_checkmate(&mut state, Side::White));
    }
}
