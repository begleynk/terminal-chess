use game::{Action, GameState};
use board::Coordinate;
use piece::{Piece, Rank};
use engine::find_moves_in_direction;

pub fn possible_actions(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let mut actions = vec![];
    actions.append(&mut possible_moves(from, state));
    //actions.append(&mut possible_captures(from, state));

    actions
}

pub fn possible_moves(from: &Coordinate, state: &GameState) -> Vec<Action> {
    let side = state.next_to_move();
    let mut moves = vec![];
    // North East
    moves.append(&mut find_moves_in_direction(
        from,
        side,
        state.board(),
        |mover| mover.north().east(),
    ));
    // South East
    moves.append(&mut find_moves_in_direction(
        from,
        side,
        state.board(),
        |mover| mover.south().east(),
    ));
    // South West
    moves.append(&mut find_moves_in_direction(
        from,
        side,
        state.board(),
        |mover| mover.south().west(),
    ));
    // North West
    moves.append(&mut find_moves_in_direction(
        from,
        side,
        state.board(),
        |mover| mover.north().west(),
    ));


    moves
        .into_iter()
        .map(|c| {
            Action::MovePiece(state.piece_at(*from).unwrap().clone(), from.clone(), c)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;
    use Side;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn moves_north_east_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state
            .update_board(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();
        state
            .update_board(&coord!("g7"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();

        let valid_moves = possible_moves(&coord!("a1"), &state);

        assert_eq!(
            valid_moves,
            vec![
                Action::MovePiece(state.piece_at(coord!("a1")).unwrap().clone(), coord!("a1"), coord!("b2")),
                Action::MovePiece(state.piece_at(coord!("a1")).unwrap().clone(), coord!("a1"), coord!("c3")),
                Action::MovePiece(state.piece_at(coord!("a1")).unwrap().clone(), coord!("a1"), coord!("d4")),
                Action::MovePiece(state.piece_at(coord!("a1")).unwrap().clone(), coord!("a1"), coord!("e5")),
                Action::MovePiece(state.piece_at(coord!("a1")).unwrap().clone(), coord!("a1"), coord!("f6")),
            ]
        );
    }

    #[test]
    fn moves_south_east_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state
            .update_board(&coord!("a8"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();
        state
            .update_board(&coord!("g2"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();

        let valid_moves = possible_moves(&coord!("a8"), &state);

        assert_eq!(
            valid_moves,
            vec![
                Action::MovePiece(state.piece_at(coord!("a8")).unwrap().clone(), coord!("a8"), coord!("b7")),
                Action::MovePiece(state.piece_at(coord!("a8")).unwrap().clone(), coord!("a8"), coord!("c6")),
                Action::MovePiece(state.piece_at(coord!("a8")).unwrap().clone(), coord!("a8"), coord!("d5")),
                Action::MovePiece(state.piece_at(coord!("a8")).unwrap().clone(), coord!("a8"), coord!("e4")),
                Action::MovePiece(state.piece_at(coord!("a8")).unwrap().clone(), coord!("a8"), coord!("f3")),
            ]
        );
    }

    #[test]
    fn moves_south_west_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state
            .update_board(&coord!("h8"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();
        state
            .update_board(&coord!("f6"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();

        let valid_moves = possible_moves(&coord!("h8"), &state);

        assert_eq!(valid_moves, vec![
                Action::MovePiece(state.piece_at(coord!("h8")).unwrap().clone(), coord!("h8"), coord!("g7")),
        ])
    }

    #[test]
    fn moves_north_west_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state
            .update_board(&coord!("h1"), Some(Piece::pack(Side::White, Rank::Bishop)))
            .unwrap();

        let valid_moves = possible_moves(&coord!("h1"), &state);

        assert_eq!(
            valid_moves,
            vec![
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("g2")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("f3")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("e4")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("d5")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("c6")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("b7")),
                Action::MovePiece(state.piece_at(coord!("h1")).unwrap().clone(), coord!("h1"), coord!("a8")),
            ]
        );
    }
}
