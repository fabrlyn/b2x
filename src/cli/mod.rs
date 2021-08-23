use std::{error::Error, str::FromStr};

use clap::{App, Arg};

use crate::binary;
use crate::binary::Endian;

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

impl FromStr for Endian {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase() {
            s if s == "little" => Ok(Endian::Little),
            s if s == "big" => Ok(Endian::Big),
            s => Err(format!("{} is not a valid endian value", s).into()),
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
    endian: Endian,
    output: Output,
}

pub fn run() {
    let matches = App::new("b2x")
        .arg(arg_input())
        .arg(arg_bit_group())
        .arg(arg_endian())
        .arg(arg_signed())
        .arg(arg_output())
        .get_matches();

    let args = Args {
        bit_group: matches.value_of_t("bit-group").unwrap(),
        endian: matches.value_of_t("endian").unwrap(),
        signed: matches.is_present("signed"),
        output: matches.value_of_t("output").unwrap(),
    };

    let input = matches.value_of("input").unwrap();

    let result = match args {
        Args {
            bit_group: 8,
            signed: false,
            endian: Endian::Big,
            ..
        } => binary::to_big_endian_u8(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 16,
            signed: false,
            endian: Endian::Big,
            ..
        } => binary::to_big_endian_u16(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 32,
            signed: false,
            endian: Endian::Big,
            ..
        } => binary::to_big_endian_u32(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 64,
            signed: false,
            endian: Endian::Big,
            ..
        } => binary::to_big_endian_u64(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 8,
            signed: false,
            endian: Endian::Little,
            ..
        } => binary::to_little_endian_u8(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 16,
            signed: false,
            endian: Endian::Little,
            ..
        } => binary::to_little_endian_u16(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 32,
            signed: false,
            endian: Endian::Little,
            ..
        } => binary::to_little_endian_u32(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        Args {
            bit_group: 64,
            signed: false,
            endian: Endian::Little,
            ..
        } => binary::to_little_endian_u64(input)
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
        _ => return,
    };

    result.iter().for_each(|v| {
        print!("{}", v);
    });

    println!();
}
