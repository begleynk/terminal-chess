use game;
use action;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Foo(String)
//    Action(action::Action),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    GameState(game::GameState),
}
