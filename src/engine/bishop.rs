use game::{Action, GameState};
use board::{Coordinate};
use piece::{Piece, Rank};
use engine::{find_moves_in_direction};

pub fn apply_move(
    piece: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    state: &mut GameState,
) -> Result<(), String> {
    assert_eq!(piece.rank(), Rank::Bishop);

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

pub fn determine_valid_moves(
    from: &Coordinate,
    state: &GameState,
) -> Vec<Coordinate> {

    let side = state.next_to_move();
    let mut moves = vec![];
    // North East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north().east()));
    // South East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south().east()));
    // South West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south().west()));
    // North West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north().west()));
    moves

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

        state.update_board(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();
        state.update_board(&coord!("g7"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();

        let valid_moves = determine_valid_moves(&coord!("a1"), &state);

        assert_eq!(valid_moves,vec![
            coord!("b2"),
            coord!("c3"),
            coord!("d4"),
            coord!("e5"),
            coord!("f6"),
        ]);
    }

    #[test]
    fn moves_south_east_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("a8"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();
        state.update_board(&coord!("g2"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();

        let valid_moves = determine_valid_moves(&coord!("a8"), &state);

        assert_eq!(valid_moves,vec![
            coord!("b7"),
            coord!("c6"),
            coord!("d5"),
            coord!("e4"),
            coord!("f3"),
        ]);
    }

    #[test]
    fn moves_south_west_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("h8"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();
        state.update_board(&coord!("f6"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();

        let valid_moves = determine_valid_moves(&coord!("h8"), &state);

        assert_eq!(valid_moves,vec![
            coord!("g7")
        ]);
    }

    #[test]
    fn moves_north_west_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("h1"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();

        let valid_moves = determine_valid_moves(&coord!("h1"), &state);

        assert_eq!(valid_moves,vec![
            coord!("g2"),
            coord!("f3"),
            coord!("e4"),
            coord!("d5"),
            coord!("c6"),
            coord!("b7"),
            coord!("a8"),
        ]);
    }
}
