extern crate terminal_chess;
extern crate clap;

use clap::{App, SubCommand, Arg};

fn main() {
    let app = App::new("Terminal Chess")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Niklas Begley")
                          .about("Play chess in your terminal.")
                          .subcommand(SubCommand::with_name("new")
                                      .about("Play against a simple AI."))
                          .subcommand(SubCommand::with_name("host")
                                      .about("Host a new game that can be joined by another player.")
                                      .arg(Arg::with_name("port")
                                        .short("p")
                                        .help("Port to connect to")))
                          .subcommand(SubCommand::with_name("join")
                                      .about("Joing an existing game over the network."));

    let matches = app.get_matches();

    if let Some(host_args) = matches.subcommand_matches("host") {
        let mut server = terminal_chess::server::Server::new(host_args.value_of("port").unwrap_or("8080").parse().unwrap());

        server.run();
    } else {
        
    }
}

// - Start session
// - Wait for other player to connect
// - Play...
// ...?
