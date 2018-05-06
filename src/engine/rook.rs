use game::{GameState};
use action::Action;
use board::{Coordinate};
use engine::{find_moves_in_direction, find_opposing_piece_in_direction};


pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    actions.append(&mut possible_captures(from, state));

    actions
}

pub fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
    let mut moves = vec![];
    // North
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north()));
    // East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.east()));
    // South
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south()));
    // West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.west()));

    moves
        .into_iter()
        .map(|c| {
            Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c)
        })
        .collect()
}

pub fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
    let piece_n =
        find_opposing_piece_in_direction(from, side, state.board(), |mover| mover.north());
    let piece_e =
        find_opposing_piece_in_direction(from, side, state.board(), |mover| mover.east());
    let piece_s =
        find_opposing_piece_in_direction(from, side, state.board(), |mover| mover.south());
    let piece_w =
        find_opposing_piece_in_direction(from, side, state.board(), |mover| mover.west());

    vec![piece_n, piece_e, piece_s, piece_w]
        .into_iter()
        .filter_map(|c| c)
        .map(|c| {
            Action::Capture(
                state.piece_at(*from).unwrap().clone(),
                state.piece_at(c).unwrap().clone(),
                from.clone(),
                c,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;
    use Side;
    use piece::{Piece, Rank};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn moves_in_straight_lines_until_it_hits_something() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        board.update(&coord!("g4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // In the way
        let state = GameState::with_board(board);


        let valid_moves = possible_moves(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d5")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d6")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d7")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d8")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("e4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("f4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d3")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d2")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("d1")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("c4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("b4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Rook), coord!("d4"), coord!("a4")),
        ]);
    }

    #[test]
    fn captures_in_straight_lines() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();

        board.update(&coord!("b4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // In the way of the piece below
        board.update(&coord!("a4"), Some(Piece::pack(Side::Black, Rank::Bishop))).unwrap();

        board.update(&coord!("d1"), Some(Piece::pack(Side::Black, Rank::Bishop))).unwrap();
        board.update(&coord!("d6"), Some(Piece::pack(Side::Black, Rank::Bishop))).unwrap();
        board.update(&coord!("g4"), Some(Piece::pack(Side::Black, Rank::Bishop))).unwrap();

        let state = GameState::with_board(board);

        let captures = possible_captures(&coord!("d4"), &state);

        assert_eq!(captures,vec![
            Action::Capture(Piece::pack(Side::White, Rank::Rook), Piece::pack(Side::Black, Rank::Bishop), coord!("d4"), coord!("d6")),
            Action::Capture(Piece::pack(Side::White, Rank::Rook), Piece::pack(Side::Black, Rank::Bishop), coord!("d4"), coord!("g4")),
            Action::Capture(Piece::pack(Side::White, Rank::Rook), Piece::pack(Side::Black, Rank::Bishop), coord!("d4"), coord!("d1")),
        ]);
    }
}
