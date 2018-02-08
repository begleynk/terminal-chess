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
    assert_eq!(piece.rank(), Rank::Knight);

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
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.fw() )));
    // East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.right() )));
    // South
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.bw() )));
    // West
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.left() )));
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn moves_in_straight_lines_until_it_hits_something() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::Rook)));
        board.update(&coord!("g4"), Some(Piece::pack(Side::White, Rank::Bishop))); // In the way

        let valid_moves = determine_valid_moves(&coord!("d4"), &board, Side::White);

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
