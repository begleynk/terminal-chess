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
    let moves = knight_moves(from, state.next_to_move());

    moves
        .into_iter()
        .filter(|c| state.board().is_empty(*c)) // Filter moves to positions that are taken
        .map(|c| Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c))
        .collect()
}

fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let moves = knight_moves(from, state.next_to_move());

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
    use engine;
    use piece::{Piece, Rank};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn in_a_shocking_turn_of_events_moves_like_a_knight() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();

        let piece = state.board().piece_at(coord!("d4")).unwrap();

        let from = Coordinate::from_human("d4".to_string()).unwrap();
        let to = Coordinate::from_human("e6".to_string()).unwrap();

        assert_eq!(
            engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state),
            Ok(())
        );
        assert_eq!(
            state.board().piece_at(coord!("e6")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
        assert_eq!(state.next_to_move(), Side::Black, "Side not updated");
    }

    #[test]
    fn cannot_move_to_taken_squares() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();
        state.update_board(&coord!("e6"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();

        let piece = state.board().piece_at(coord!("d4")).unwrap();

        let from = Coordinate::from_human("d4".to_string()).unwrap();
        let to = Coordinate::from_human("e6".to_string()).unwrap();

        assert_eq!(
            engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state),
            Err("Invalid move".to_string())
        );
        assert_eq!(state.history().len(), 0, "History updated");
        assert_eq!(state.next_to_move(), Side::White, "Side updated");
    }

    #[test]
    fn cannot_jump_off_the_board() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        assert_eq!(state.update_board(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))), Ok(()));

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
        let mut state = GameState::new();
        state.set_board(Board::empty());

        assert_eq!(state.update_board(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))), Ok(()));
        assert_eq!(state.update_board(&coord!("b3"), Some(Piece::pack(Side::Black, Rank::Pawn))), Ok(()));
        assert_eq!(state.update_board(&coord!("c2"), Some(Piece::pack(Side::Black, Rank::Pawn))), Ok(()));

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
