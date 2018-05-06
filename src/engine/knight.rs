use game::{GameState};
use action::Action;
use board::{Coordinate};
use engine::Mover;
use Side;

pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    actions.append(&mut possible_captures(from, state));

    actions
}

fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
    let moves = knight_moves(from, side);

    moves
        .into_iter()
        .filter(|c| state.board().is_empty(*c)) // Filter moves to positions that are taken
        .map(|c| Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c))
        .collect()
}

pub fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
    let moves = knight_moves(from, side);

    moves
        .into_iter()
        .filter(|c| match *state.piece_at(*c) {
            Some(p) => p.side() != state.next_to_move(),
            None => false
        })
        .map(|c| Action::Capture(state.piece_at(*from).unwrap().clone(), state.piece_at(c).unwrap().clone(), from.clone(), c))
        .collect()
}

fn knight_moves(from: &Coordinate, side: Side) -> Vec<Coordinate> {
    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    // North moves
    moves.push(Mover::new(side).move_to(from).north().north().west().make());
    moves.push(Mover::new(side).move_to(from).north().north().east().make());
    // East
    moves.push(Mover::new(side).move_to(from).east().east().north().make());
    moves.push(Mover::new(side).move_to(from).east().east().south().make());
    // South
    moves.push(Mover::new(side).move_to(from).south().south().east().make());
    moves.push(Mover::new(side).move_to(from).south().south().west().make());
    // West
    moves.push(Mover::new(side).move_to(from).west().west().south().make());
    moves.push(Mover::new(side).move_to(from).west().west().north().make());

    moves
        .into_iter()
        .filter_map(|m| m.ok()) // Filter valid coordinates
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
    fn in_a_shocking_turn_of_events_moves_like_a_knight() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();

        let state = GameState::with_board(board);

        let piece = state.board().piece_at(coord!("d4")).unwrap();
        let from = Coordinate::from_human("d4".to_string()).unwrap();

        assert_eq!(
            possible_actions(&from, &state),
            vec![
                Action::MovePiece(piece.clone(), from.clone(), coord!("c6")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("e6")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("f5")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("f3")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("e2")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("c2")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("b3")),
                Action::MovePiece(piece.clone(), from.clone(), coord!("b5")),
            ]
        );
    }

    #[test]
    fn cannot_move_to_taken_squares() {
        let mut board = Board::empty();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();
        board.update(&coord!("b3"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();

        let state = GameState::with_board(board);

        let from = Coordinate::from_human("a1".to_string()).unwrap();

        assert_eq!(
            possible_moves(&from, &state),
            vec![Action::MovePiece(Piece::pack(Side::White, Rank::Knight), from, coord!("c2"))]
        );
    }

    #[test]
    fn cannot_jump_off_the_board() {
        let mut board = Board::empty();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();

        let state = GameState::with_board(board);

        let valid_moves = possible_moves(
            &coord!("a1"),
            &state
        );

        assert_eq!(valid_moves, vec![
            Action::MovePiece(Piece::pack(Side::White, Rank::Knight), coord!("a1"), coord!("b3")),
            Action::MovePiece(Piece::pack(Side::White, Rank::Knight), coord!("a1"), coord!("c2"))
        ]);
    }

    #[test]
    fn can_capture_pieces() {
        let mut board = Board::empty();;
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();
        board.update(&coord!("b3"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap();
        board.update(&coord!("c2"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap();

        let state = GameState::with_board(board);

        let valid_moves = possible_captures(
            &coord!("a1"),
            &state
        );

        assert_eq!(valid_moves, vec![
            Action::Capture(Piece::pack(Side::White, Rank::Knight), Piece::pack(Side::Black, Rank::Pawn), coord!("a1"), coord!("b3")),
            Action::Capture(Piece::pack(Side::White, Rank::Knight), Piece::pack(Side::Black, Rank::Pawn), coord!("a1"), coord!("c2"))
        ]);
    }
}
