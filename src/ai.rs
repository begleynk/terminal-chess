use Side;
use board::{Board, Coordinate};
use game::{GameState};
use action::Action;
use piece::{Piece, Rank};
use engine;


pub fn make_move(state: &GameState) -> Option<Action> {
    root_alpha_beta(state)
}

pub fn root_alpha_beta(state: &GameState) -> Option<Action> {
    let side = state.next_to_move();
    let action = possible_actions(state, side).into_iter().max_by_key(|action| {
        let mut new_state = state.clone();
        engine::apply_action(&action, &mut new_state).unwrap();
        let score = -alpha_beta(3, &new_state, side, <i32>::min_value()+1, <i32>::max_value());
        score
    });
    action
}

fn possible_actions(state: &GameState, side: Side) -> Vec<Action> {
    state.board().pieces_with_coordinates()
        .into_iter()
        .filter(|&(_coordinate, piece)| piece.side() == side)
        .map(|(coordinate, _piece)| coordinate)
        .flat_map(|coordinate| state.actions_at(coordinate))
        .collect()
}


fn alpha_beta(depth: u8, state: &GameState, my_side: Side, mut alpha: i32, beta: i32) -> i32 {
    let mut actions = possible_actions(state, state.next_to_move());
    actions.sort_by_key(|&a| { match a { Action::Capture(_, _, _, _) => 1, _ => 2 } });
    if depth == 0 || actions.is_empty() {
        return evaluate_board(state, state.next_to_move()) - depth as i32; // penalty for games that end early
    }

    let mut score = <i32>::min_value()+1;

    for action in actions {
        let mut new_state = state.clone();
        engine::apply_action(&action, &mut new_state).unwrap();
        let value = -alpha_beta(depth - 1, &new_state, my_side, -beta, -alpha);
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

fn evaluate_board(state: &GameState, my_side: Side) -> i32 {
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

    macro_rules! coord {
        ($x:expr) => { Coordinate::from_human($x.to_string()).unwrap() }
    }

    macro_rules! matches {
        ($expression: expr, $($pattern:tt)+) => {
            _tt_as_expr_hack! {
                match $expression {
                    $($pattern)+ => true,
                    _ => false
                }
            }
        }
    }

    macro_rules! _tt_as_expr_hack {
            ($value:expr) => ($value)
    }

    #[test]
    fn makes_the_obvious_move() {
        let mut state = GameState::new();
        let mut board = Board::empty();
        board.update(&coord!("b5"), Some(Piece::pack(Side::White, Rank::Bishop))).unwrap();
        board.update(&coord!("e8"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        state.set_board(board);
        let action = make_move(&state);

        assert!(matches!(action, Some(Action::Capture(_, _, _, _))));
    }

    #[test]
    fn tries_not_to_die() {
        let mut state = GameState::new();
        let mut board = Board::empty();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::Rook))).unwrap();
        board.update(&coord!("a5"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("a3"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        board.update(&coord!("f8"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        state.set_board(board);
        let action = make_move(&state);
        let from = &coord!("e4");
        let to = &coord!("d4");

        assert!(matches!(action, Some(Action::MovePiece(_, from, to))));
    }

    #[test]
    fn never_ends_the_game_with_two_kings() {
        let mut state = GameState::new();
        let mut board = Board::empty();
        board.update(&coord!("e4"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        board.update(&coord!("f8"), Some(Piece::pack(Side::Black, Rank::King))).unwrap();
        state.set_board(board);
        for _ in 0..500 {
            let action = make_move(&state);
            engine::apply_action(&action.unwrap(), &mut state);

            assert!(matches!(action, Some(Action::MovePiece(_, _, _))));
        }
    }

    #[test]
    fn queen_wins_with_two_moves() {
        let mut state = GameState::new();
        let mut board = Board::empty();
        board.update(&coord!("a5"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        board.update(&coord!("g8"), Some(Piece::pack(Side::Black, Rank::Rook))).unwrap();
        board.update(&coord!("h1"), Some(Piece::pack(Side::White, Rank::King))).unwrap();
        state.set_board(board);
        let action1 = make_move(&state);
        println!("{:?}", action1);
        engine::apply_action(&action1.unwrap(), &mut state);
        let action2 = make_move(&state);
        println!("{:?}", action2);
        engine::apply_action(&action2.unwrap(), &mut state);
        let action3 = make_move(&state);
        println!("{:?}", action3);
        engine::apply_action(&action3.unwrap(), &mut state);
        let action4 = make_move(&state);
        println!("{:?}\n {:?}\n {:?}\n {:?}", action1, action2, action3, action4);

        assert!(matches!(action4, Some(Action::Capture(_, _, _, _))));
    }
}

