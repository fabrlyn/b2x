use std::fmt::Display;

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

fn output<T>(data: Vec<T>)
where
    T: Display,
{
    data.iter().for_each(|output| {
        print!("{} ", output);
    });
    println!();
}

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
            // u8
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 8) => {
                output(input.bin_to_dec().u8().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 1..=7) => {
                output(input.bin_to_dec().u8().exact(group_size).unwrap().convert());
            }
            (BIG_ENDIAN, UNSIGNED, COMPACT, 8) => {
                output(input.bin_to_dec().big_endian().u8().convert());
            }
            (BIG_ENDIAN, SIGNED, COMPACT, 8) => {
                output(input.bin_to_dec().big_endian().i8().convert());
            }
            (BIG_ENDIAN, UNSIGNED, SPACED, 8) => {
                output(input.bin_to_dec().u8().convert());
            }
            (BIG_ENDIAN, SIGNED, SPACED, 8) => {
                output(input.bin_to_dec().i8().convert());
            }

            (LITTLE_ENDIAN, SIGNED, COMPACT, 8) => {
                output(input.bin_to_dec().i8().convert());
            }

            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 16) => {
                output(input.bin_to_dec().u16().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 32) => {
                output(input.bin_to_dec().u32().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 64) => {
                output(input.bin_to_dec().u64().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 128) => {
                output(input.bin_to_dec().u128().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 9..=15) => {
                output(
                    input
                        .bin_to_dec()
                        .u16()
                        .exact(group_size)
                        .unwrap()
                        .convert(),
                );
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 17..=31) => {
                output(
                    input
                        .bin_to_dec()
                        .u32()
                        .exact(group_size)
                        .unwrap()
                        .convert(),
                );
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 33..=63) => {
                output(
                    input
                        .bin_to_dec()
                        .u64()
                        .exact(group_size)
                        .unwrap()
                        .convert(),
                );
            }
            (LITTLE_ENDIAN, UNSIGNED, COMPACT, 65..=127) => {
                output(
                    input
                        .bin_to_dec()
                        .u128()
                        .exact(group_size)
                        .unwrap()
                        .convert(),
                );
            }

            // LITLE_ENDIAN, UNSIGNED, SPACED, N
            (LITTLE_ENDIAN, UNSIGNED, SPACED, 16) => {
                output(input.bin_to_dec().u16().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, SPACED, 32) => {
                output(input.bin_to_dec().u32().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, SPACED, 64) => {
                output(input.bin_to_dec().u64().convert());
            }
            (LITTLE_ENDIAN, UNSIGNED, SPACED, 128) => {
                output(input.bin_to_dec().u128().convert());
            }

            // LITTLE_ENDIAN, SIGNED, COMPACT, N
            (LITTLE_ENDIAN, SIGNED, COMPACT, 16) => {
                output(input.bin_to_dec().i16().convert());
            }
            (LITTLE_ENDIAN, SIGNED, COMPACT, 32) => {
                output(input.bin_to_dec().i32().convert());
            }
            (LITTLE_ENDIAN, SIGNED, COMPACT, 64) => {
                output(input.bin_to_dec().i64().convert());
            }
            (LITTLE_ENDIAN, SIGNED, COMPACT, 128) => {
                output(input.bin_to_dec().i128().convert());
            }

            // BIG_ENDIAN, UNSIGNED, COMPACT, N
            (BIG_ENDIAN, UNSIGNED, COMPACT, 16) => {
                output(input.bin_to_dec().big_endian().u16().convert());
            }
            (BIG_ENDIAN, UNSIGNED, COMPACT, 32) => {
                output(input.bin_to_dec().big_endian().u32().convert());
            }
            (BIG_ENDIAN, UNSIGNED, COMPACT, 64) => {
                output(input.bin_to_dec().big_endian().u64().convert());
            }
            (BIG_ENDIAN, UNSIGNED, COMPACT, 128) => {
                output(input.bin_to_dec().big_endian().u128().convert());
            }

            // BIG_ENDIAN, SIGNED, COMPACT, N
            (BIG_ENDIAN, SIGNED, COMPACT, 16) => {
                output(input.bin_to_dec().big_endian().i16().convert());
            }
            (BIG_ENDIAN, SIGNED, COMPACT, 32) => {
                output(input.bin_to_dec().big_endian().i32().convert());
            }
            (BIG_ENDIAN, SIGNED, COMPACT, 64) => {
                output(input.bin_to_dec().big_endian().i64().convert());
            }
            (BIG_ENDIAN, SIGNED, COMPACT, 128) => {
                output(input.bin_to_dec().big_endian().i128().convert());
            }

            // LITLE_ENDIAN, SIGNED, SPACED, N
            (LITTLE_ENDIAN, SIGNED, SPACED, 16) => {
                output(input.bin_to_dec().i16().convert());
            }
            (LITTLE_ENDIAN, SIGNED, SPACED, 32) => {
                output(input.bin_to_dec().i32().convert());
            }
            (LITTLE_ENDIAN, SIGNED, SPACED, 64) => {
                output(input.bin_to_dec().i64().convert());
            }
            (LITTLE_ENDIAN, SIGNED, SPACED, 128) => {
                output(input.bin_to_dec().i128().convert());
            }

            // BIG_ENDIAN, UNSIGNED, SPACED, N
            (BIG_ENDIAN, UNSIGNED, SPACED, 16) => {
                output(input.bin_to_dec().u8().convert());
            }
            (BIG_ENDIAN, UNSIGNED, SPACED, 32) => {
                output(input.bin_to_dec().u8().convert());
            }
            (BIG_ENDIAN, UNSIGNED, SPACED, 64) => {
                output(input.bin_to_dec().u8().convert());
            }
            (BIG_ENDIAN, UNSIGNED, SPACED, 128) => {
                output(input.bin_to_dec().u8().convert());
            }

            // BIG_ENDIAN, UNSIGNED, SPACED, N
            (BIG_ENDIAN, SIGNED, SPACED, 16) => {
                output(input.bin_to_dec().i16().convert());
            }
            (BIG_ENDIAN, SIGNED, SPACED, 32) => {
                output(input.bin_to_dec().i32().convert());
            }
            (BIG_ENDIAN, SIGNED, SPACED, 64) => {
                output(input.bin_to_dec().i64().convert());
            }
            (BIG_ENDIAN, SIGNED, SPACED, 128) => {
                output(input.bin_to_dec().i128().convert());
            }

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
