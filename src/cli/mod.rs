use std::{error::Error, str::FromStr};

use clap::{App, Arg};

use crate::binary::Endian;

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

fn validate_group(val: &str) -> Result<(), String> {
    let group = val.parse::<u8>().map_err(|e| e.to_string())?;
    if group > 64 {
        return Err("group can't be larger than 64".into());
    }
    Ok(())
}

pub fn run() {
    let _matches = App::new("b2x")
        .arg(Arg::new("input").required(true))
        .arg(
            Arg::new("signed")
                .short('s')
                .long("signed")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("strict-group")
                .required(false)
                .takes_value(false)
                .long("strict-group"),
        )
        .arg(
            Arg::new("group")
                .default_value("8")
                .validator(validate_group)
                .required(false)
                .short('g')
                .long("group")
                .takes_value(true),
        )
        .arg(
            Arg::new("endian")
                .possible_values(&["big", "little"])
                .default_value("little")
                .short('e')
                .long("endian")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    /*
    let signed = matches.is_present("signed");
    let strict_group = matches.is_present("strict-group");
    let group: usize = matches.value_of_t("group").unwrap_or_else(|e| e.exit());
    let endian: Endian = matches.value_of_t("endian").unwrap_or_else(|e| e.exit());
    let input = matches.value_of("input").unwrap();
    */
}
