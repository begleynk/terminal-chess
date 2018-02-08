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
    assert_eq!(piece.rank(), Rank::Bishop);

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
    // North East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.north().east() )));
    // South East
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.south().east() )));
    // South West
    moves.append(&mut find_moves_in_direction(from, side, board,(|mover| mover.south().west() )));
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
    fn moves_north_east_until_it_hits_something() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::Bishop)));
        board.update(&coord!("g7"), Some(Piece::pack(Side::White, Rank::Bishop)));

        let valid_moves = determine_valid_moves(&coord!("a1"), &board, Side::White);

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

        let mut board = Board::empty();
        board.update(&coord!("a8"), Some(Piece::pack(Side::White, Rank::Bishop)));
        board.update(&coord!("g2"), Some(Piece::pack(Side::White, Rank::Bishop)));

        let valid_moves = determine_valid_moves(&coord!("a8"), &board, Side::White);

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

        let mut board = Board::empty();
        board.update(&coord!("h8"), Some(Piece::pack(Side::White, Rank::Bishop)));
        board.update(&coord!("f6"), Some(Piece::pack(Side::White, Rank::Bishop)));

        let valid_moves = determine_valid_moves(&coord!("h8"), &board, Side::White);

        assert_eq!(valid_moves,vec![
            coord!("g7")
        ]);
    }

    #[test]
    fn moves_north_west_until_it_hits_something() {
        let mut state = GameState::new();

        let mut board = Board::empty();
        board.update(&coord!("h1"), Some(Piece::pack(Side::White, Rank::Bishop)));

        let valid_moves = determine_valid_moves(&coord!("h1"), &board, Side::White);

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
