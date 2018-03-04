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
    // North East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north().east()));
    // East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.east()));
    // South East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south().east()));
    // South
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south()));
    // South West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south().west()));
    // West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.west()));
    // North West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north().west()));

    moves
        .into_iter()
        .map(|c| {
            Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c)
        })
        .collect()
}

pub fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
    let mut moves = vec![];
    // North
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.north()));
    // North East
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.north().east()));
    // East
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.east()));
    // South East
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.south().east()));
    // South
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.south()));
    // South West
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.south().west()));
    // West
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.west()));
    // North West
    moves.push(find_opposing_piece_in_direction(from, side, state.board(),|mover| mover.north().west()));

    moves
        .into_iter()
        .filter_map(|c| c )
        .map(|c| {
            Action::Capture(state.piece_at(*from).unwrap().clone(), state.piece_at(c).unwrap().clone(), from.clone(), c)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Side;
    use board::Board;
    use piece::{Piece, Rank};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn moves_in_straight_lines_and_diagonally_until_it_hits_something() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Queen))).unwrap();
        board.update(&coord!("g4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // In the way

        let state = GameState::with_board(board);

        let valid_moves = possible_moves(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            // North
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d5")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d6")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d7")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d8")),

            // North West
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("e5")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("f6")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("g7")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("h8")),

            // West
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("e4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("f4")),

            // South West
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("e3")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("f2")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("g1")),

            // South
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d3")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d2")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("d1")),

            // South East
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("c3")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("b2")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("a1")),

            // East
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("c4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("b4")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("a4")),

            // North East
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("c5")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("b6")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Queen), coord!("d4"), coord!("a7")),
        ]);
    }

    #[test]
    fn captures_in_straight_lines_and_diagonally() {
        let mut board = Board::empty();
        board
            .update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Queen)))
            .unwrap();
        board
            .update(&coord!("g7"), Some(Piece::pack(Side::Black, Rank::Pawn)))
            .unwrap();
        board
            .update(&coord!("b6"), Some(Piece::pack(Side::Black, Rank::Pawn)))
            .unwrap();
        board
            .update(&coord!("c3"), Some(Piece::pack(Side::White, Rank::Pawn)))
            .unwrap();
        board
            .update(&coord!("b2"), Some(Piece::pack(Side::Black, Rank::Pawn)))
            .unwrap();
        board
            .update(&coord!("d6"), Some(Piece::pack(Side::Black, Rank::Bishop)))
            .unwrap();

        let state = GameState::with_board(board);

        let valid_moves = possible_captures(&coord!("d4"), &state);

        assert_eq!(
            valid_moves,
            vec![
                Action::Capture(
                    state.piece_at(coord!("d4")).unwrap().clone(),
                    state.piece_at(coord!("d6")).unwrap().clone(),
                    coord!("d4"),
                    coord!("d6"),
                ),
                Action::Capture(
                    state.piece_at(coord!("d4")).unwrap().clone(),
                    state.piece_at(coord!("g7")).unwrap().clone(),
                    coord!("d4"),
                    coord!("g7"),
                ),
                Action::Capture(
                    state.piece_at(coord!("d4")).unwrap().clone(),
                    state.piece_at(coord!("b6")).unwrap().clone(),
                    coord!("d4"),
                    coord!("b6"),
                ),
            ]
        )
    }
}
