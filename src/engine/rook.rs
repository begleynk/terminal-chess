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
    assert_eq!(piece.rank(), Rank::Rook);

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
    // North
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.north()));
    // East
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.east()));
    // South
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.south()));
    // West
    moves.append(&mut find_moves_in_direction(from, side, state.board(),|mover| mover.west()));
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
    fn moves_in_straight_lines_until_it_hits_something() {
        let mut state = GameState::new();
        state.set_board(Board::empty());

        state.update_board(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        state.update_board(&coord!("g4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // In the way

        let valid_moves = determine_valid_moves(&coord!("d4"), &state);

        assert_eq!(valid_moves,vec![
            coord!("d5"),
            coord!("d6"),
            coord!("d7"),
            coord!("d8"),
            coord!("e4"),
            coord!("f4"),
            coord!("d3"),
            coord!("d2"),
            coord!("d1"),
            coord!("c4"),
            coord!("b4"),
            coord!("a4"),
        ]);
    }
}
