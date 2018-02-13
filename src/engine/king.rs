use game::{Action, GameState};
use board::{Coordinate};
use piece::{Piece, Rank};
use engine::{Mover};

pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    actions.append(&mut possible_captures(from, state));

    actions
}

pub fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.next_to_move();
    let mut moves = vec![];
    // North
    moves.push(Mover::new(side).move_to(from).north().make());
    // North East
    moves.push(Mover::new(side).move_to(from).north().east().make());
    // East
    moves.push(Mover::new(side).move_to(from).east().make());
    // South East
    moves.push(Mover::new(side).move_to(from).south().east().make());
    // South
    moves.push(Mover::new(side).move_to(from).south().make());
    // South West
    moves.push(Mover::new(side).move_to(from).south().west().make());
    // West
    moves.push(Mover::new(side).move_to(from).west().make());
    // North West
    moves.push(Mover::new(side).move_to(from).north().west().make());

    moves
    .into_iter()
    .filter_map(|m| m.ok()) // Filter valid coordinates
    .filter(|c| state.board().is_empty(*c)) // Filter moves to positions that are taken
    .map(|c| {
        Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c)
    })
    .collect()
}

pub fn possible_captures(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.next_to_move();
    let mut moves = vec![];
    // North
    moves.push(Mover::new(side).move_to(from).north().make());
    // North East
    moves.push(Mover::new(side).move_to(from).north().east().make());
    // East
    moves.push(Mover::new(side).move_to(from).east().make());
    // South East
    moves.push(Mover::new(side).move_to(from).south().east().make());
    // South
    moves.push(Mover::new(side).move_to(from).south().make());
    // South West
    moves.push(Mover::new(side).move_to(from).south().west().make());
    // West
    moves.push(Mover::new(side).move_to(from).west().make());
    // North West
    moves.push(Mover::new(side).move_to(from).north().west().make());

    moves
    .into_iter()
    .filter_map(|c| c.ok() )
    .filter(|c| match *state.piece_at(*c) {
        Some(p) => p.side() != state.piece_at(*from).unwrap().side(),
        None => false,
    })
    .map(|c| Action::Capture(state.piece_at(*from).unwrap().clone(), state.piece_at(c).unwrap().clone(), from.clone(), c))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Side;
    use board::Board;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_move_one_step_in_every_direction() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        state.update_board(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // Next to the king
        state.update_board(&coord!("d5"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Above the king

        let valid_moves = possible_moves(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            // North West
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("e5")),

            // South West
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("e3")),
            // South
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("d3")),
            // South East
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("c3")),
            // East
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("c4")),
            // North East
            Action::MovePiece(Piece::pack(Side::White, Rank::King), coord!("d4"), coord!("c5")),
        ]);
    }

    #[test]
    fn can_capture_one_step_in_every_direction() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        state.update_board(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // Next to the king
        state.update_board(&coord!("d5"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Above the king
        state.update_board(&coord!("d2"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Far away from the king

        let valid_moves = possible_captures(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            // North
            Action::Capture(Piece::pack(Side::White, Rank::King), Piece::pack(Side::Black, Rank::Pawn), coord!("d4"), coord!("d5")),
        ]);
    }
}
