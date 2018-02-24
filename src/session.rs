use game::{Game};
use ui::Cursor;
use Side;
use action::{Action, to_coordinate_for};

use piece::{Piece, Rank};
use board::Coordinate;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(PartialEq, Clone, Debug)]
pub enum SessionState {
    NothingSelected,
    CoordinateSelected(Coordinate, Vec<Action>),
    WillQuit
}

#[derive(PartialEq, Debug)]
pub struct Session {
    current_game: Game,
    cursor: Cursor,
    player_as: Side,
    state: SessionState,
}

impl Session {
    pub fn new() -> Session {
        Session {
            current_game: Game::new(),
            cursor: Cursor::new(Side::White),
            player_as: Side::White,
            state: SessionState::NothingSelected,
        }
    }

    pub fn run(mut self) {
        let mut stdout = ::std::io::stdout().into_raw_mode().unwrap();

        ::ui::clear(&mut stdout);
        ::ui::draw(&self, &mut stdout);

        for c in ::std::io::stdin().keys() {
            self.update(c.unwrap());
            ::ui::draw(&self, &mut stdout);

            if self.state == SessionState::WillQuit {
                break;
            }
        }
    }

    pub fn game(&self) -> &Game {
        &self.current_game
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn player_as(&self) -> Side {
        self.player_as
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }

    fn update(&mut self, input: Key) {
        let mut next_state: Option<SessionState> = None;

        match input {
            Key::Char('q') => self.state = SessionState::WillQuit,
            Key::Up => self.cursor.up(),
            Key::Down => self.cursor.down(),
            Key::Right => self.cursor.right(),
            Key::Left => self.cursor.left(),
            Key::Char(' ') => {
                match self.state {
                    SessionState::NothingSelected => {
                        if let &Some(piece) = self.game().state().piece_at(self.cursor.to_coord()) {
                            if piece.side() == self.game().state().next_to_move() {
                                let possible_actions = self.game().state().actions_at(self.cursor.to_coord());
                                next_state = Some(SessionState::CoordinateSelected(self.cursor.to_coord(), possible_actions));
                            }
                        }
                    },
                    SessionState::CoordinateSelected(ref _coord, ref actions) => {
                        if let Some(action) = actions.into_iter().find(|a| to_coordinate_for(a) == &self.cursor.to_coord() ) {
                            self.current_game.advance(action.clone());
                            next_state = Some(SessionState::NothingSelected);
                        } else if let &Some(piece) = self.game().state().piece_at(self.cursor.to_coord()) {
                            if piece.side() == self.game().state().next_to_move() {
                                let possible_actions = self.game().state().actions_at(self.cursor.to_coord());
                                next_state = Some(SessionState::CoordinateSelected(self.cursor.to_coord(), possible_actions));
                            }
                        } else {
                            next_state = Some(SessionState::NothingSelected);
                        }
                    }
                    _ => unimplemented!()
                }
            }
            _ => {}
        }

        if let Some(state) = next_state {
            self.state = state;
        }
    }
}