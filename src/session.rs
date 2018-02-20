use game::{Action, Game};
use ui::Cursor;
use Side;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(PartialEq, Clone, Debug)]
enum SessionState {
    NothingSelected,
    CoordinateSelected(Vec<Action>),
    WillQuit
}

#[derive(PartialEq, Debug)]
pub struct Session {
    current_game: Game,
    cursor: Cursor,
    player_as: Side,
    state: SessionState
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

    fn update(&mut self, input: Key) {
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
                                self.state = SessionState::CoordinateSelected(possible_actions)
                            }
                        }
                    }
                    _ => unimplemented!()
                }
            }
            _ => {}
        }
    }
}