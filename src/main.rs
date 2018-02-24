extern crate terminal_chess;
extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let app = App::new("Terminal Chess")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Niklas Begley")
                          .about("Play chess in your terminal.")
                          .subcommand(SubCommand::with_name("new")
                                      .about("Host a new game that can be joined by another player."))
                          .subcommand(SubCommand::with_name("join")
                                      .about("Joing an existing game over the network."))
                          .subcommand(SubCommand::with_name("ai")
                                      .about("Play against a simple AI."));

    let matches = app.get_matches();

    if let Some(_subcommand_matches) = matches.subcommand_matches("new") {
        let mut session = terminal_chess::new_session();
        session.run();
    } else {
        
    }
}
