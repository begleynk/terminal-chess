use board::Coordinate;
use piece::Piece;

#[derive(PartialEq, Clone, Copy, Debug)]
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
