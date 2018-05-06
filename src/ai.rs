use Side;
use game::{GameState};
use action::Action;
use piece::{Piece, Rank};
use engine;


pub fn make_move(state: &mut GameState) -> Option<Action> {
    root_alpha_beta(state)
}

pub fn root_alpha_beta(state: &mut GameState) -> Option<Action> {
    let side = state.next_to_move();
    let action = possible_actions(state, side).into_iter().max_by_key(|action| {
        state.evaluate_with_action(action.clone(), |mut new_state| {
            -alpha_beta(4, &mut new_state, side, <i32>::min_value()+1, <i32>::max_value())
        })
    });
    action
}

fn possible_actions(state: &mut GameState, side: Side) -> Vec<Action> {
    state.board().pieces_with_coordinates()
        .into_iter()
        .filter(|&(_coordinate, piece)| piece.side() == side)
        .map(|(coordinate, _piece)| coordinate)
        .flat_map(|coordinate| engine::enumerate_all_actions(&coordinate, state))
        .collect()
}


fn alpha_beta(depth: u8, state: &mut GameState, my_side: Side, mut alpha: i32, beta: i32) -> i32 {
    let next_to_move = state.next_to_move();
    let mut actions = possible_actions(state, next_to_move);
    actions.sort_by_key(|&a| { match a { Action::Capture(_, _, _, _) => 1, _ => 2 } });
    if depth == 0 || actions.is_empty() {
        return evaluate_board(state, next_to_move) - depth as i32; // penalty for games that end early
    }

    let mut score = <i32>::min_value()+1;

    for action in actions {
        let value = state.evaluate_with_action(action.clone(), |mut new_state| {
            -alpha_beta(depth - 1, &mut new_state, my_side, -beta, -alpha)
        });
        if value > score { score = value; }
        if score > alpha { alpha = score; }
        if score >= beta { break; }
    }
    score
}


fn evaluate_piece(piece: Piece, my_side: Side) -> i32 {
    let score = match piece.rank() {
        Rank::Pawn => 1,
        Rank::Knight => 3,
        Rank::Bishop => 3,
        Rank::Rook => 5,
        Rank::Queen => 9,
        Rank::King => 1000,
    };
    if piece.side() == my_side { score } else { -score }
}

fn evaluate_board(state: &mut GameState, my_side: Side) -> i32 {
    let material_score: i32 = state.board().pieces_with_coordinates()
        .into_iter()
        .map(|(_coordinate, piece)| evaluate_piece(piece, my_side))
        .sum();
    let mobility_score = possible_actions(state, my_side).len() as i32;
    10 * material_score + mobility_score
}


#[cfg(test)]
mod tests {
    use super::*;
    use board::{Board, Coordinate};

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    #[test]
    fn makes_the_obvious_move() {
        let mut board = Board::empty();
        board.update(&coord!("b5"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();
        board.update(&coord!("e8"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        board.update(&coord!("h8"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        let mut state = GameState::with_board(board);
        let action = make_move(&mut state);

        assert!(matches!(action, Some(Action::Capture(_, _, _, _))));
    }

    #[test]
    fn queen_wins_with_two_moves() {
        let mut board = Board::empty();
        board.update(&coord!("a5"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        board.update(&coord!("g8"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        board.update(&coord!("h1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("a1"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        let mut state = GameState::with_board(board);
        println!("{:?}", state.board());
        let action1 = make_move(&mut state);
        println!("{:?}", action1);
        println!("{:?}", state.board());
        state.advance(action1.clone().unwrap()).unwrap();
        let action2 = make_move(&mut state);
        println!("{:?}", action2);
        println!("{:?}", state.board());
        state.advance(action2.clone().unwrap()).unwrap();
        let action3 = make_move(&mut state);
        println!("{:?}", action3);
        println!("{:?}", state.board());
        state.advance(action3.clone().unwrap()).unwrap();
        let action4 = make_move(&mut state);
        println!("{:?}\n {:?}\n {:?}\n {:?}", action1, action2, action3, action4);
        println!("{:?}", state.board());

        assert!(matches!(action4, Some(Action::Capture(_, _, _, _))));
    }
}

