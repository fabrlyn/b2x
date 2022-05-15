mod bin;
use std::{error::Error, str::FromStr};

use clap::{App, Arg};

enum Output {
    Decimal,
    Float,
    Hex,
    Utf8,
    BinaryLittleEndian,
    BinaryBigEndian,
}

impl FromStr for Output {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Output::*;
        match s.to_lowercase() {
            s if s == "decimal" => Ok(Decimal),
            s if s == "float" => Ok(Float),
            s if s == "hex" => Ok(Hex),
            s if s == "utf-8" => Ok(Utf8),
            s if s == "binary-little-endian" => Ok(BinaryLittleEndian),
            s if s == "binary-big-endian" => Ok(BinaryBigEndian),
            _ => Err(format!("{} is not a valid output value", s).into()),
        }
    }
}

fn arg_bit_group<'a>() -> Arg<'a> {
    Arg::new("bit-group")
        .required(false)
        .default_value("8")
        .possible_values(&["8", "16", "32", "64"])
        .long("bit-group")
        .takes_value(true)
}

fn arg_input<'a>() -> Arg<'a> {
    Arg::new("input").required(true)
}

fn arg_endian<'a>() -> Arg<'a> {
    Arg::new("endian")
        .possible_values(&["big", "little"])
        .default_value("big")
        .short('e')
        .long("endian")
        .takes_value(true)
        .required(false)
}

fn arg_signed<'a>() -> Arg<'a> {
    Arg::new("signed")
        .short('s')
        .long("signed")
        .takes_value(false)
        .required(false)
}

fn arg_output<'a>() -> Arg<'a> {
    Arg::new("output")
        .short('o')
        .long("output")
        .default_value("decimal")
        .takes_value(true)
        .required(false)
}

struct Args {
    bit_group: u8,
    signed: bool,
    output: Output,
}

pub fn run() {
    let app = App::new("b2x").subcommand(bin::app());
    let matches = app.get_matches();
    /*
    .arg(arg_input())
    .arg(arg_bit_group())
    .arg(arg_endian())
    .arg(arg_signed())
    .arg(arg_output())
    */

    /*
    let args = Args {
        bit_group: matches.value_of_t("bit-group").unwrap(),
        signed: matches.is_present("signed"),
        output: matches.value_of_t("output").unwrap(),
    };

    let input = matches.value_of("input").unwrap();
    */

    match matches.subcommand() {
        Some(("bin", args)) => {
            bin::handle(args);
        }
        _ => {
            println!("No match");
        }
    }
}
