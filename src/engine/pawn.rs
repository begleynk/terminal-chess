use game::{GameState};
use action::Action;
use board::Coordinate;
use Side;
use engine::Mover;

pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    actions.append(&mut possible_captures(from, state));

    actions
}

fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    // Moves forward
    moves.push(
        Mover::new(state.next_to_move())
            .move_to(from)
            .north()
            .make(),
    );

    if is_starting_coordinate(from, state.next_to_move()) {
        // Moves forward twice
        moves.push(
            Mover::new(state.next_to_move())
                .move_to(from)
                .north()
                .north()
                .make(),
        );
    }

    moves
        .into_iter()
        .filter_map(|m| m.ok()) // Filter valid coordinates
        .filter(|c| state.board().is_empty(*c)) // Filter moves to positions that are taken
        .map(|c| Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c))
        .collect()
}

fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    moves.push(
        Mover::new(state.next_to_move())
            .move_to(from)
            .north()
            .east()
            .make(),
    );
    moves.push(
        Mover::new(state.next_to_move())
            .move_to(from)
            .north()
            .west()
            .make(),
    );
    // TODO: Ampasant

    moves
        .into_iter()
        .filter_map(|c| c.ok())
        .filter(|c| match *state.piece_at(*c) {
            Some(p) => p.side() != state.piece_at(*from).unwrap().side(),
            None => false,
        })
        .map(|c| Action::Capture(state.piece_at(*from).unwrap().clone(), state.piece_at(c).unwrap().clone(), from.clone(), c))
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
    use engine;
    use piece::{Piece, Rank};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_move_pawns_one_step_ahead() {
        let mut state = GameState::new();

        let piece = state.board().piece_at(coord!("e2")).unwrap();
        let from = coord!("e2");
        let to = coord!("e3");

        assert_eq!(engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state), Ok(()));
        assert_eq!(state.board().piece_at(coord!("e3")), &Some(piece));
        assert_eq!(state.history().len(), 1, "History not updated");
        assert_eq!(state.next_to_move(), Side::Black, "Side not updated");
    }

    #[test]
    fn can_move_pawns_two_step_ahead() {
        let mut state = GameState::new();

        let piece = state.board().piece_at(coord!("e2")).unwrap();
        let from = coord!("e2");
        let to = coord!("e4");

        assert_eq!(engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state), Ok(()));
        assert_eq!(state.board().piece_at(coord!("e4")), &Some(piece));
        assert_eq!(state.history().len(), 1, "History not updated");
    }

    #[test]
    fn cannot_move_ahead_if_the_pawn_is_blocked() {
        let mut state = GameState::new();

        assert_eq!(
            state.update_board(&coord!("e3"), Some(Piece::pack(Side::Black, Rank::Bishop)),),
            Ok(())
        ); // Bishop blocking on e3

        let piece = state.board().piece_at(coord!("e2")).unwrap();
        let from = coord!("e2");
        let to = coord!("e3");

        assert_eq!(
            engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state),
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
        assert_eq!(
            engine::apply_action(&Action::MovePiece(state.piece_at(coord!("e2")).unwrap().clone(), coord!("e2"), coord!("e3")), &mut state),
            Ok(())
        );
        // Random black move
        assert_eq!(
            engine::apply_action(&Action::MovePiece(state.piece_at(coord!("b7")).unwrap().clone(), coord!("b7"), coord!("b6")), &mut state),
            Ok(())
        );

        // Try to move e3 to e5
        let piece = state.board().piece_at(coord!("e3")).unwrap();
        let from = coord!("e3");
        let to = coord!("e5");

        assert_eq!(
            engine::apply_action(&Action::MovePiece(piece.clone(), from.clone(), to.clone()), &mut state),
            Err("Invalid move".to_string())
        );
        assert_eq!(state.board().piece_at(coord!("e4")), &None);
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

        state
            .update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Pawn)))
            .unwrap();
        state
            .update_board(&coord!("e5"), Some(Piece::pack(Side::Black, Rank::Knight)))
            .unwrap(); // In the way

        assert_eq!(
            engine::apply_action(
                &Action::Capture(
                    Piece::pack(Side::White, Rank::Pawn),
                    Piece::pack(Side::Black, Rank::Knight),
                    coord!("d4"),
                    coord!("e5"),
                ),
                &mut state
            ),
            Ok(())
        );

        assert_eq!(
            state.piece_at(coord!("e5")),
            &Some(Piece::pack(Side::White, Rank::Pawn))
        );
        assert_eq!(state.piece_at(coord!("d4")), &None);
        assert_eq!(state.history().len(), 1);
        assert_eq!(
            state.captures(),
            &vec![Piece::pack(Side::Black, Rank::Knight)]
        )
    }

    #[test]
    fn finds_correct_captures() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state
            .update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Pawn)))
            .unwrap();
        state
            .update_board(&coord!("e5"), Some(Piece::pack(Side::Black, Rank::Knight)))
            .unwrap(); // In the way

        let captures = possible_captures(&coord!("d4"), &state);

         assert_eq!(captures, vec![
             Action::Capture(
                 Piece::pack(Side::White, Rank::Pawn),
                 Piece::pack(Side::Black, Rank::Knight),
                 coord!("d4"),
                 coord!("e5")
             )
         ])
    }
}
