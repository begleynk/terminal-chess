use game::{Action, GameState};
use board::{Board, Coordinate};
use piece::{Piece, Rank};
use Side;
use engine::Mover;

pub fn apply_move(
    piece: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    board: &mut Board,
    state: &mut GameState,
) -> Result<(), String> {
    assert_eq!(piece.rank(), Rank::Pawn);

    let valid_moves = determine_valid_moves(piece, from, board, state.next_to_move());

    if valid_moves.contains(to) {
        board.update(to, Some(piece.clone()));
        state.add_action_to_history(Action::MovePiece(piece.clone(), from.clone(), to.clone()));

        Ok(())
    } else {
        Err("Invalid move".to_string())
    }
}

pub fn determine_valid_moves(
    piece: &Piece,
    from: &Coordinate,
    board: &Board,
    side: Side,
) -> Vec<Coordinate> {
    let mut moves: Vec<Result<Coordinate, String>> = Vec::new();

    // Moves forward
    let mut mover = Mover::new(side);
    mover.move_to(from);
    mover.fw();
    moves.push(mover.make());

    // Moves forward twice
    let mut mover = Mover::new(side);
    mover.move_to(from);
    mover.fw();
    mover.fw();
    moves.push(mover.make());

    moves
        .into_iter()
        .filter_map(|m| m.ok()) // Filter valid coordinates
        .filter(|c| board.is_empty(*c)) // Filter moves to positions that are taken
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_move_pawns_one_step_ahead() {
        let mut state = GameState::new();

        let mut board = Board::default();
        let piece = board
            .piece_at(Coordinate::from_human("e2".to_string()).unwrap())
            .unwrap();
        let from = Coordinate::from_human("e2".to_string()).unwrap();
        let to = Coordinate::from_human("e3".to_string()).unwrap();

        assert_eq!(
            apply_move(&piece, &from, &to, &mut board, &mut state),
            Ok(())
        );
        assert_eq!(
            board.piece_at(coord!("e3")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
    }

    #[test]
    fn can_move_pawns_two_step_ahead() {
        let mut state = GameState::new();

        let mut board = Board::default();
        let piece = board
            .piece_at(coord!("e2"))
            .unwrap();
        let from = coord!("e2");
        let to = coord!("e4");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut board, &mut state),
            Ok(())
        );
        assert_eq!(
            board.piece_at(coord!("e4")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
    }

    #[test]
    fn cannot_move_ahead_if_the_pawn_is_blocked() {
        let mut state = GameState::new();

        let mut board = Board::default();
        board.update(
            &coord!("e3"),
            Some(Piece::pack(Side::Black, Rank::Bishop)),
        ); // Bishop blocking on e3

        let piece = board.piece_at(coord!("e2")).unwrap();
        let from = coord!("e2");
        let to = coord!("e3");

        assert_eq!(
            apply_move(&piece, &from, &to, &mut board, &mut state),
            Err(("Invalid move".to_string()))
        );
        assert_eq!(
            board.piece_at(coord!("e3")),
            &Some(Piece::pack(Side::Black, Rank::Bishop))
        );
        assert_eq!(
            state.history().len(),
            0,
            "History updated when it should not have"
        );
    }
}
