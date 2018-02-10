use game::{Action, GameState};
use board::{Board, Coordinate};
use piece::{Piece, Rank};
use Side;

mod pawn;
mod knight;
mod bishop;
mod rook;
mod queen;
mod king;

pub fn apply_action(action: &Action, state: &mut GameState) -> Result<(), String> {
    match *action {
        Action::MovePiece(piece, from, to) => apply_move(&piece, &from, &to, state),
        Action::Capture(_, _, _) => apply_capture(action, state),
        Action::Promotion(_, _, _) => apply_promotion(action, state),
    }
}

fn apply_move(piece: &Piece, from: &Coordinate, to: &Coordinate, state: &mut GameState) -> Result<(), String> {
    // Assert the "from" coordinate has the piece we are expecting
    assert_eq!(state.board().piece_at(*from), &Some(*piece));

    match piece.rank() {
        Rank::Pawn => pawn::apply_move(piece, from, to, state),
        Rank::Knight => knight::apply_move(piece, from, to, state),
        Rank::Bishop => bishop::apply_move(piece, from, to, state),
        Rank::Rook => rook::apply_move(piece, from, to, state),
        Rank::Queen => queen::apply_move(piece, from, to, state),
        Rank::King => king::apply_move(piece, from, to, state)
    }
}

#[allow(unused_variables)]
fn apply_capture(action: &Action, state: &mut GameState) -> Result<(), String> {
    unimplemented!()
}

#[allow(unused_variables)]
fn apply_promotion(action: &Action, state: &mut GameState) -> Result<(), String> {
    unimplemented!()
}


struct Mover {
    side: Side,
    current_row: i8,
    current_column: i8
}

impl Mover {
    fn new(side: Side) -> Mover {
        Mover { side: side, current_row: 0, current_column: 0}
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
fn find_moves_in_direction<F>(starting_coordinate: &Coordinate, side: Side, board: &Board, closure: F) -> Vec<Coordinate> where F: Fn(Mover) -> Mover {
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
                break
            }
        } else {
            break;
        }
    }
    accumulator
} 