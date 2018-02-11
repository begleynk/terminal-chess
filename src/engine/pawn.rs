use game::{Action, GameState};
use board::{Coordinate};
use piece::{Piece, Rank};
use Side;
use engine::Mover;

pub fn apply_move(
    piece: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    state: &mut GameState,
) -> Result<(), String> {
    assert_eq!(piece.rank(), Rank::Pawn);

    let valid_moves = determine_valid_moves(from, state);

    if valid_moves.contains(to) {
        state.update_board(to, Some(piece.clone())).expect("Bad move found. Bug");
        state.add_action_to_history(Action::MovePiece(piece.clone(), from.clone(), to.clone()));
        state.toggle_side();

        Ok(())
    } else {
        Err("Invalid move".to_string())
    }
}

pub fn apply_capture(
    capturer: &Piece,
    target: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    state: &mut GameState,
) -> Result<(), String> {
    let valid_moves = determine_valid_captures(from, state);

    if valid_moves.contains(to) {
        state.update_board(to, Some(capturer.clone())).expect("Bad move found. Bug");
        state.update_board(from, None).expect("Bad move found. Bug");
        state.add_piece_to_capture_list(target.clone());
        state.add_action_to_history(Action::Capture(capturer.clone(), target.clone(), from.clone(), to.clone()));
        state.toggle_side();

        Ok(())
    } else {
        Err("Invalid capture".to_string())
    }
}

pub fn determine_valid_moves(
    from: &Coordinate,
    state: &GameState
) -> Vec<Coordinate> {
    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    // Moves forward
    moves.push(Mover::new(state.next_to_move()).move_to(from).north().make());

    if is_starting_coordinate(from, state.next_to_move()) {
        // Moves forward twice
        moves.push(Mover::new(state.next_to_move()).move_to(from).north().north().make());
    }

    moves
        .into_iter()
        .filter_map(|m| m.ok()) // Filter valid coordinates
        .filter(|c| state.board().is_empty(*c)) // Filter moves to positions that are taken
        .collect()
}

pub fn determine_valid_captures(from: &Coordinate, state: &GameState) -> Vec<Coordinate> {

    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    moves.push(Mover::new(state.next_to_move()).move_to(from).north().east().make());
    moves.push(Mover::new(state.next_to_move()).move_to(from).north().west().make());
    // TODO: Ampasant

    moves
    .into_iter()
    .filter_map(|c| c.ok() )
    .filter(|c| match *state.piece_at(*c) {
        Some(p) => p.side() != state.piece_at(*from).unwrap().side(),
        None => false
    })
    .collect()
}

fn is_starting_coordinate(coordinate: &Coordinate, side: Side) -> bool {
    match side {
        Side::White => coordinate.row() == 1,
        Side::Black => coordinate.row() == 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_move_pawns_one_step_ahead() {
        let mut state = GameState::new();

        let piece = state.board()
            .piece_at(coord!("e2"))
            .unwrap();
        let from = coord!("e2");
        let to = coord!("e3");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut state),
            Ok(())
        );
        assert_eq!(
            state.board().piece_at(coord!("e3")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
        assert_eq!(state.next_to_move(), Side::Black, "Side not updated");
    }

    #[test]
    fn can_move_pawns_two_step_ahead() {
        let mut state = GameState::new();

        let piece = state.board()
            .piece_at(coord!("e2"))
            .unwrap();
        let from = coord!("e2");
        let to = coord!("e4");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut state),
            Ok(())
        );
        assert_eq!(
            state.board().piece_at(coord!("e4")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
    }

    #[test]
    fn cannot_move_ahead_if_the_pawn_is_blocked() {
        let mut state = GameState::new();

        assert_eq!(state.update_board(
            &coord!("e3"),
            Some(Piece::pack(Side::Black, Rank::Bishop)),
        ), Ok(())); // Bishop blocking on e3

        let piece = state.board().piece_at(coord!("e2")).unwrap();
        let from = coord!("e2");
        let to = coord!("e3");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut state),
            Err("Invalid move".to_string())
        );
        assert_eq!(
            state.board().piece_at(coord!("e3")),
            &Some(Piece::pack(Side::Black, Rank::Bishop))
        );
        assert_eq!(
            state.history().len(),
            0,
            "History updated when it should not have"
        );
    }

    #[test]
    fn cannot_move_two_places_if_not_on_starting_row() {
        let mut state = GameState::new();

        // Move pawn to e3
        assert_eq!(apply_move(&state.board().piece_at(coord!("e2")).unwrap(), &coord!("e2"), &coord!("e3"), &mut state), Ok(()));
        // Random black move
        assert_eq!(apply_move(&state.board().piece_at(coord!("b7")).unwrap(), &coord!("b7"), &coord!("b6"), &mut state), Ok(()));

        // Try to move e3 to e5
        let piece = state.board().piece_at(coord!("e3")).unwrap();
        let from = coord!("e3");
        let to = coord!("e5");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut state),
            Err("Invalid move".to_string())
        );
        assert_eq!(
            state.board().piece_at(coord!("e4")),
            &None
        );
        assert_eq!(
            state.history().len(),
            2,
            "History updated when it should not have"
        );
    }

    #[test]
    fn can_capture_diagonally() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();
        state.update_board(&coord!("e5"), Some(Piece::pack(Side::Black, Rank::Knight))).unwrap(); // In the way

        assert_eq!(apply_capture(
            &Piece::pack(Side::White, Rank::Pawn),
            &Piece::pack(Side::Black, Rank::Knight),
            &coord!("d4"),
            &coord!("e5"),
            &mut state
        ), Ok(()));

        assert_eq!(state.piece_at(coord!("e5")), &Some(Piece::pack(Side::White, Rank::Pawn)));
        assert_eq!(state.piece_at(coord!("d4")), &None);
        assert_eq!(state.history().len(), 1);
        assert_eq!(state.captures(), &vec![Piece::pack(Side::Black, Rank::Knight)])
    }

    #[test]
    fn finds_correct_captures() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();
        state.update_board(&coord!("e5"), Some(Piece::pack(Side::Black, Rank::Knight))).unwrap(); // In the way

        let captures = determine_valid_captures(&coord!("d4"), &state);

        assert_eq!(captures, vec![coord!("e5")]);

        // assert_eq!(captures, vec![
        //     Action::Capture(
        //         Piece::pack(Side::White, Rank::Pawn),
        //         Piece::pack(Side::Black, Rank::Knight),
        //         coord!("d4"),
        //         coord!("d5")
        //     )
        // ])
    }
}
