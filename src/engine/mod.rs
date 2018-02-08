use game::{Action, GameState};
use board::{Board, Coordinate};
use piece::{Piece, Rank};
use Side;

mod pawn;
mod knight;
mod bishop;
mod rook;

pub fn apply_action(action: &Action, board: &mut Board, state: &mut GameState) -> Result<(), String> {
    match *action {
        Action::MovePiece(piece, from, to) => apply_move(&piece, &from, &to, board, state),
        Action::Capture(_, _, _) => apply_capture(action, board, state),
        Action::Promotion(_, _, _) => apply_promotion(action, board, state),
    }
}

fn apply_move(piece: &Piece, from: &Coordinate, to: &Coordinate, board: &mut Board, state: &mut GameState) -> Result<(), String> {
    // Assert the "from" coordinate has the piece we are expecting
    assert_eq!(board.piece_at(*from), &Some(*piece));

    match piece.rank() {
        Rank::Pawn => pawn::apply_move(piece, from, to, board, state),
        Rank::Knight => knight::apply_move(piece, from, to, board, state),
        Rank::Bishop => bishop::apply_move(piece, from, to, board, state),
        Rank::Rook => rook::apply_move(piece, from, to, board, state),
        Rank::Queen => unimplemented!(),
        Rank::King => unimplemented!(),
    }
}

fn apply_capture(action: &Action, board: &mut Board, state: &mut GameState) -> Result<(), String> {
    unimplemented!()
}

fn apply_promotion(action: &Action, board: &mut Board, state: &mut GameState) -> Result<(), String> {
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

    fn fw(mut self) -> Self {
        match self.side {
            Side::White => self.current_row += 1,
            Side::Black => self.current_row -= 1,
        }

        self
    }

    fn bw(mut self) -> Self {
        match self.side {
            Side::White => self.current_row -= 1,
            Side::Black => self.current_row += 1,
        }

        self
    }

    fn left(mut self) -> Self {
        match self.side {
            Side::White => self.current_column -= 1,
            Side::Black => self.current_column += 1,
        }

        self
    }

    fn right(mut self) -> Self {
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