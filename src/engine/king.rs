use game::{GameState};
use action::{Action, CastleSide};
use board::{Coordinate};
use engine::{Mover, is_in_check, opponent_can_capture};
use piece::{Piece, Rank};

pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    actions.append(&mut possible_captures(from, state));

    if can_castle_queen_side(state) {
        actions.push(Action::Castle(CastleSide::QueenSide))
    }

    actions
}

pub fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.piece_at(*from).unwrap().side();
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
    let side = state.piece_at(*from).unwrap().side();
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

fn can_castle_queen_side(state: &GameState) -> bool {
    let my_side = state.next_to_move();
    let actions = state.history().into_iter().filter(|action| {
        match **action {
            Action::MovePiece(piece, _, _) => {
                (piece.rank() == Rank::King || piece.rank() == Rank::Rook) &&
                    piece.side() == my_side
            },
            _ => false,
        }
    }).collect::<Vec<&Action>>();

    let blocking_squares = [
        Coordinate::from_human("b1".to_owned()).unwrap(),
        Coordinate::from_human("c1".to_owned()).unwrap(),
        Coordinate::from_human("d1".to_owned()).unwrap(),
    ]; 

    let in_flight_squares = [
        Coordinate::from_human("c1".to_owned()).unwrap(),
        Coordinate::from_human("d1".to_owned()).unwrap(),
    ]; 

    !is_in_check(state, my_side)
        && actions.is_empty()
        && blocking_squares.into_iter().all(|coord| state.board().is_empty(*coord))
        && !in_flight_squares.into_iter().any(|coord| opponent_can_capture(coord, my_side, state))
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
    fn can_move_one_step_in_every_direction() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // Next to the king
        board.update(&coord!("d5"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Above the king

        let state = GameState::with_board(board);

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
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // Next to the king
        board.update(&coord!("d5"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Above the king
        board.update(&coord!("d2"), Some(Piece::pack(Side::Black, Rank::Pawn))).unwrap(); // Far away from the king

        let state = GameState::with_board(board);

        let valid_moves = possible_captures(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            // North
            Action::Capture(Piece::pack(Side::White, Rank::King), Piece::pack(Side::Black, Rank::Pawn), coord!("d4"), coord!("d5")),
        ]);
    }

    #[test]
    fn cannot_castle_when_in_check() {
        let mut board = Board::empty();
        board.update(&coord!("e1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        board.update(&coord!("e8"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();

        let state = GameState::with_board(board);
        assert!(!can_castle_queen_side(&state))
     
    }

    #[test]
    fn can_castle_if_king_and_rook_havent_moved_before() {
        let mut board = Board::empty();
        let white_king = Piece::pack(Side::White, Rank::King);
        let black_king = Piece::pack(Side::Black, Rank::King);
        board.update(&coord!("e1"), Some(white_king)).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        board.update(&coord!("e8"), Some(black_king)).unwrap();

        let mut state = GameState::with_board(board);
        assert!(can_castle_queen_side(&state));

        state.advance(Action::MovePiece(white_king, coord!("e1"), coord!("e2")));
        state.advance(Action::MovePiece(black_king, coord!("e8"), coord!("e7")));
     
        assert!(!can_castle_queen_side(&state));
    }

    #[test]
    fn cannot_castle_if_nonempty_squares_in_between() {
        let mut board = Board::empty();
        let white_king = Piece::pack(Side::White, Rank::King);
        board.update(&coord!("e1"), Some(white_king)).unwrap();
        board.update(&coord!("b1"), Some(Piece::pack(Side::White, Rank::Knight))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();

        let state = GameState::with_board(board);

        assert!(!can_castle_queen_side(&state));
    }


    #[test]
    fn cannot_castle_if_opponent_can_capture_king_in_flight() {
        let mut board = Board::empty();
        let white_king = Piece::pack(Side::White, Rank::King);
        board.update(&coord!("e1"), Some(white_king)).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        board.update(&coord!("d8"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();

        let state = GameState::with_board(board);

        assert!(!can_castle_queen_side(&state));
    }

    #[test]
    fn includes_possible_castle_actions() {
        let mut board = Board::empty();
        let white_king = Piece::pack(Side::White, Rank::King);
        let black_king = Piece::pack(Side::Black, Rank::King);
        board.update(&coord!("e1"), Some(white_king)).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        //board.update(&coord!("e8"), Some(black_king)).unwrap();

        let state = GameState::with_board(board);

        let actions = possible_actions(&coord!("e1"), &state);

        assert!(actions.contains(&Action::Castle(CastleSide::QueenSide)))
    }
}
