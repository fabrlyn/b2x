use b2x::logic::common::{
    BigEndian, DecimalConverter, DecimalConverterExt, DefaultBitAlignment, FromBinary, LittleEndian,
};
use clap::{App, ArgMatches};

use crate::cli::shared::arg;

const LITTLE_ENDIAN: bool = false;
const BIG_ENDIAN: bool = true;

const UNSPACED: bool = false;
const SPACED: bool = true;

const UNSIGNED: bool = false;
const SIGNED: bool = true;

const ASYMMETRIC: bool = false;
const SYMMETRIC: bool = true;

pub struct ToDecCommand;

impl ToDecCommand {
    pub fn command<'a>() -> App<'a> {
        App::new(Self::name())
            .arg(arg::Input::arg())
            .arg(arg::BigEndian::arg())
            .arg(arg::GroupSize::arg())
            .arg(arg::Signed::arg())
            .arg(arg::Spaced::arg())
    }

    /*
    fn to_little_endian<T, F>(d: DecimalConverter<T, LittleEndian, F>, matches: &ArgMatches) {
        let group_size = arg::GroupSize::value(matches);
        let signed = arg::Signed::value(matches);
        let spaced = arg::Spaced::value(matches);

        let symmetric = [8, 16, 32, 64, 128].contains(&group_size);

        match (signed, spaced, symmetric) {
            (UNSIGNED, UNSPACED, SYMMETRIC) => {}
            (UNSIGNED, UNSPACED, ASYMMETRIC) => {}
            (UNSIGNED, SPACED, SYMMETRIC) => {}
            (UNSIGNED, SPACED, ASYMMETRIC) => {}
            (SIGNED, UNSPACED, SYMMETRIC) => {}
            (SIGNED, UNSPACED, ASYMMETRIC) => {}
            (SIGNED, SPACED, SYMMETRIC) => {}
            (SIGNED, SPACED, ASYMMETRIC) => {}
        }
    }
    */

    /*
    fn to_big_endian<T, F>(
        d: DecimalConverter<DefaultBitAlignment, BigEndian, F>,
        matches: &ArgMatches,
    ) {
        let group_size = arg::GroupSize::value(matches);
        let signed = arg::Signed::value(matches);
        let spaced = arg::Spaced::value(matches);

        let symmetric = [8, 16, 32, 64, 128].contains(&group_size);

        match (signed, spaced, symmetric) {
            (UNSIGNED, UNSPACED, SYMMETRIC) => {}
            (UNSIGNED, UNSPACED, ASYMMETRIC) => {}
            (UNSIGNED, SPACED, SYMMETRIC) => {}
            (UNSIGNED, SPACED, ASYMMETRIC) => {}
            (SIGNED, UNSPACED, SYMMETRIC) => {}
            (SIGNED, UNSPACED, ASYMMETRIC) => {}
            (SIGNED, SPACED, SYMMETRIC) => {}
            (SIGNED, SPACED, ASYMMETRIC) => {}
        }
    }
    */

    pub fn handle(matches: &ArgMatches) {
        let input = arg::Input::value(matches);
        let big_endian = arg::BigEndian::value(matches);

        let signed = arg::Signed::value(matches);
        let spaced = arg::Spaced::value(matches);

        let group_size = arg::GroupSize::value(matches);
        let symmetric = [8, 16, 32, 64, 128].contains(&group_size);
        /*

        match big_endian {
            LITTLE_ENDIAN => {
                Self::to_little_endian(input.decimal(), matches);
            }
            BIG_ENDIAN => {
                Self::to_little_endian(input.decimal(), matches);
            }
        }
        */
    }

    const fn name() -> &'static str {
        "dec"
    }
}

pub struct Command;

impl Command {
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

    pub const fn name() -> &'static str {
        "bin"
    }
}
