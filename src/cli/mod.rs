mod bin;
mod shared;

use clap::{App, ArgMatches};

pub struct Cli;

impl Cli {
    pub fn run() {
        Self::handle(
            &App::new("b2x")
                .subcommand(bin::Command::command())
                .get_matches(),
        );
    }

    fn handle(matches: &ArgMatches) {
        const BIN_CMD: &str = bin::Command::name();

        match matches.subcommand() {
            Some((BIN_CMD, args)) => bin::Command::handle(args),
            _ => {
                println!("No match");
            }
        }
    }
}
