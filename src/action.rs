use std::fmt;

use board::Coordinate;
use piece::Piece;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Action {
    MovePiece(Piece, Coordinate, Coordinate),
    Capture(Piece, Piece, Coordinate, Coordinate),
    Promotion(Piece, Piece, Coordinate, Coordinate),
}

pub fn to_coordinate_for(action: &Action) -> &Coordinate {
    match *action {
        Action::MovePiece(_,_,ref to) => to,
        Action::Capture(_,_,_,ref to) => to,
        _ => unimplemented!()
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::MovePiece(piece, from, to) =>
                write!(f, "{:?} ::{}->{}", piece, from.to_human(), to.to_human()),
            Action::Capture(piece1, piece2, from, to) =>
                write!(f, "CAPTURE {:?} {:?} :: {}-{}", piece1, piece2, from.to_human(), to.to_human()),
            _ => unimplemented!()
        }
    }
}
