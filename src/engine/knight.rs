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
    assert_eq!(piece.rank(), Rank::Knight);

    let valid_moves = determine_valid_moves(piece, from, board, state.next_to_move());

    if valid_moves.contains(to) {
        board.update(to, Some(piece.clone()));
        state.add_action_to_history(Action::MovePiece(piece.clone(), from.clone(), to.clone()));
        state.toggle_side();

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

    // North moves
    moves.push(Mover::new(side).move_to(from).fw().fw().left().make());
    moves.push(Mover::new(side).move_to(from).fw().fw().right().make());
    // East
    moves.push(Mover::new(side).move_to(from).right().right().fw().make());
    moves.push(Mover::new(side).move_to(from).right().right().bw().make());
    // South
    moves.push(Mover::new(side).move_to(from).bw().bw().right().make());
    moves.push(Mover::new(side).move_to(from).bw().bw().left().make());
    // West
    moves.push(Mover::new(side).move_to(from).left().left().bw().make());
    moves.push(Mover::new(side).move_to(from).left().left().fw().make());

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
    fn in_a_shocking_turn_of_events_moves_like_a_knight() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Knight)));

        let piece = board.piece_at(coord!("d4")).unwrap();

        let from = Coordinate::from_human("d4".to_string()).unwrap();
        let to = Coordinate::from_human("e6".to_string()).unwrap();

        assert_eq!(
            apply_move(&piece, &from, &to, &mut board, &mut state),
            Ok(())
        );
        assert_eq!(
            board.piece_at(coord!("e6")),
            &Some(piece)
        );
        assert_eq!(state.history().len(), 1, "History not updated");
        assert_eq!(state.next_to_move(), Side::Black, "Side not updated");
    }

    #[test]
    fn cannot_move_to_taken_squares() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Knight)));
        board.update(&coord!("e6"), Some(Piece::pack(Side::White, Rank::Rook)));

        let piece = board.piece_at(coord!("d4")).unwrap();

        let from = Coordinate::from_human("d4".to_string()).unwrap();
        let to = Coordinate::from_human("e6".to_string()).unwrap();

        assert_eq!(
            apply_move(&piece, &from, &to, &mut board, &mut state),
            Err("Invalid move".to_string())
        );
        assert_eq!(state.history().len(), 0, "History updated");
        assert_eq!(state.next_to_move(), Side::White, "Side updated");
    }

    #[test]
    fn cannot_jump_off_the_board() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        assert_eq!(board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Knight))), Ok(()));

        let valid_moves = determine_valid_moves(
            &board.piece_at(coord!("a1")).unwrap(),
            &coord!("a1"),
            &board,
            Side::White
        );

        assert_eq!(valid_moves, vec![
            coord!("b3"), coord!("c2")
        ]);
    }
}
