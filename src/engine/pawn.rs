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
    let side = state.piece_at(*from).unwrap().side();

    // TODO: Consider refactoring this mess
    let one_square_forward = Mover::new(side)
                                .move_to(from)
                                .north()
                                .make();
    // Moves forward
    moves.push(one_square_forward.clone());

    if is_starting_coordinate(from, side) && state.board().is_empty(one_square_forward.unwrap()) {
        // Moves forward twice
        moves.push(
            Mover::new(side)
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
    let side = state.piece_at(*from).unwrap().side();

    moves.push(
        Mover::new(side)
            .move_to(from)
            .north()
            .east()
            .make(),
    );
    moves.push(
        Mover::new(side)
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
    use piece::{Piece, Rank};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn can_move_pawns_one_step_ahead() {
        let mut board = Board::empty();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();

        let state = GameState::with_board(board);

        assert_eq!(
            possible_moves(&coord!("e4"), &state),
            vec![
                Action::MovePiece(Piece::pack(Side::White, Rank::Pawn), coord!("e4"), coord!("e5"))
            ]
        );

    }

    #[test]
    fn can_move_pawns_two_step_ahead_if_on_starting_row() {
        let mut board = Board::empty();
        board.update(&coord!("e2"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();

        let state = GameState::with_board(board);

        assert_eq!(
            possible_moves(&coord!("e2"), &state),
            vec![
                Action::MovePiece(Piece::pack(Side::White, Rank::Pawn), coord!("e2"), coord!("e3")),
                Action::MovePiece(Piece::pack(Side::White, Rank::Pawn), coord!("e2"), coord!("e4"))
            ]
        );
    }

    #[test]
    fn cannot_move_ahead_if_the_pawn_is_blocked() {
        let mut board = Board::empty();
        board.update(&coord!("e2"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();
        board.update(&coord!("e3"), Some(Piece::pack(Side::Black, Rank::Bishop))).unwrap(); // Bishop blocking on e3

        let state = GameState::with_board(board);

        assert_eq!(
            possible_moves(&coord!("e2"), &state),
            vec![]
        );
    }

    #[test]
    fn finds_correct_captures() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Pawn))).unwrap();
        board.update(&coord!("e5"), Some(Piece::pack(Side::Black, Rank::Knight))).unwrap(); // In the way

        let state = GameState::with_board(board);

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
