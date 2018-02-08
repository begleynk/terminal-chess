use game::{Action, GameState};
use board::{Board, Coordinate};
use piece::{Piece, Rank};
use Side;
use engine::{Mover, find_moves_in_direction};

pub fn apply_move(
    piece: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    board: &mut Board,
    state: &mut GameState,
) -> Result<(), String> {
    assert_eq!(piece.rank(), Rank::Queen);

    let valid_moves = determine_valid_moves(from, board, state.next_to_move());

    if valid_moves.contains(to) {
        board.update(to, Some(piece.clone())).expect("Bad move found. Bug");
        state.add_action_to_history(Action::MovePiece(piece.clone(), from.clone(), to.clone()));
        state.toggle_side();

        Ok(())
    } else {
        Err("Invalid move".to_string())
    }
}

pub fn determine_valid_moves(
    from: &Coordinate,
    board: &Board,
    side: Side,
) -> Vec<Coordinate> {

    let mut moves = vec![];
    // North
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.north() )));
    // North East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.north().east() )));
    // East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.east() )));
    // South East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.south().east() )));
    // South
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.south() )));
    // South West
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.south().west() )));
    // West
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.west() )));
    // North West
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.north().west() )));
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn moves_in_straight_lines_and_diagonally_until_it_hits_something() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Queen)));
        board.update(&coord!("g4"), Some(Piece::pack(Side::White, Rank::Bishop))); // In the way

        let valid_moves = determine_valid_moves(&coord!("d4"), &board, Side::White);

        assert_eq!(valid_moves,vec![
            // North
            coord!("d5"),
            coord!("d6"),
            coord!("d7"),
            coord!("d8"),

            // North West
            coord!("e5"),
            coord!("f6"),
            coord!("g7"),
            coord!("h8"),

            // West
            coord!("e4"),
            coord!("f4"),

            // South West
            coord!("e3"),
            coord!("f2"),
            coord!("g1"),

            // South
            coord!("d3"),
            coord!("d2"),
            coord!("d1"),

            // South East
            coord!("c3"),
            coord!("b2"),
            coord!("a1"),

            // East
            coord!("c4"),
            coord!("b4"),
            coord!("a4"),

            // North East
            coord!("c5"),
            coord!("b6"),
            coord!("a7"),
        ]);
    }
}
