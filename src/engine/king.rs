use game::{Action, GameState};
use board::{Board, Coordinate};
use piece::{Piece, Rank};
use Side;
use engine::{Mover};

pub fn apply_move(
    piece: &Piece,
    from: &Coordinate,
    to: &Coordinate,
    board: &mut Board,
    state: &mut GameState,
) -> Result<(), String> {
    assert_eq!(piece.rank(), Rank::King);

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
    fn can_move_one_step_in_ever_direction() {
        let mut board = Board::empty();
        board.update(&coord!("d4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap(); // Next to the king

        let valid_moves = determine_valid_moves(&coord!("d4"), &board, Side::White);

        assert_eq!(valid_moves,vec![
            // North
            coord!("d5"),
            // North West
            coord!("e5"),
            // West is blocked
            // coord!("e4"),

            // South West
            coord!("e3"),
            // South
            coord!("d3"),
            // South East
            coord!("c3"),
            // East
            coord!("c4"),
            // North East
            coord!("c5"),
        ]);
    }
}
