use b2x::logic::common::{BinToDecExt, ToDec};
use clap::{App, ArgMatches};

use crate::cli::shared::arg;

const LITTLE_ENDIAN: bool = false;
const BIG_ENDIAN: bool = true;

const COMPACT: bool = false;
const SPACED: bool = true;

const UNSIGNED: bool = false;
const SIGNED: bool = true;

pub struct ToDecCommand;

impl ToDecCommand {
    const fn name() -> &'static str {
        "dec"
    }

    pub fn command<'a>() -> App<'a> {
        App::new(Self::name())
            .arg(arg::Input::arg())
            .arg(arg::BigEndian::arg())
            .arg(arg::GroupSize::arg())
            .arg(arg::Signed::arg())
            .arg(arg::Spaced::arg())
    }

    pub fn handle(matches: &ArgMatches) {
        let input = arg::Input::value(matches);
        let big_endian = arg::BigEndian::value(matches);

        let signed = arg::Signed::value(matches);
        let spaced = arg::Spaced::value(matches);

        let group_size = arg::GroupSize::value(matches);

        match (big_endian, signed, spaced, group_size) {
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 8) => {
                let converted: Vec<u8> = input.bin_to_dec().u8().convert();
                converted.iter().for_each(|output| {
                    println!("output: {}", output);
                });
            }
            (LITTLE_ENDIAN, UNSIGNED, SPACED, 8) => {}
            _ => {}
        }
    }
}

pub struct Command;

impl Command {
    pub const fn name() -> &'static str {
        "bin"
    }

    pub fn command<'a>() -> App<'a> {
        App::new(Self::name()).subcommand(ToDecCommand::command())
    }

    pub fn handle(matches: &ArgMatches) {
        const TO_DEC: &str = ToDecCommand::name();

        match matches.subcommand() {
            Some((TO_DEC, matches)) => {
                ToDecCommand::handle(matches);
            }
            _ => {
                println!("No match in bin");
            }
        }
    }
}
