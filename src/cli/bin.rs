use b2x::bit_group::common::{DecimalConverterExt, FromBinary};
use clap::{App, Arg, ArgMatches};

fn app_dec<'a>() -> App<'a> {
    App::new("dec")
        .arg(Arg::new("input").takes_value(true).required(true).about("The binary string input"))
        .arg(Arg::new("big-endian").long("big-endian").takes_value(false).required(false).about("Parse the input digits as big endian bits"))
        .arg(Arg::new("group-size").long("group-size").takes_value(true).required(false).default_value("64").about("Set the number of digits for each binary number"))
        .arg(Arg::new("signed").long("signed").takes_value(false).about("If the number should be parsed as a signed number"))
        .arg(Arg::new("spaced").long("spaced").takes_value(false).about("Each binary number can be variable length and is parsed based the space character as a delimiter"))
}

fn handle_dec(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap();

    let big_endian = matches.is_present("big-endian");
    let group_size: u8 = matches.value_of_t("group-size").unwrap();
    let signed = matches.is_present("signed");
    let spaced = matches.is_present("spaced");

    println!("input: {}", input);

    println!("group_size: {}", group_size);
    println!("signed: {}", signed);
    println!("big_endian: {}", big_endian);

    let output = match (big_endian, signed, spaced) {
        (false, false, false) => match group_size {
            8 => input
                .decimal()
                .u8()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            16 => input
                .decimal()
                .u16()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            32 => input
                .decimal()
                .u32()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            64 => input
                .decimal()
                .u64()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            128 => input
                .decimal()
                .u128()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            _ => Err("Not yet supported".to_string()),
        },
        (false, false, true) => match group_size {
            2..=8 => input
                .decimal()
                .spaced_u8()
                .from_binary()
                .map_err(|e| format!("{:?}", e))
                .map(|n| format!("{:?}", n)),
            _ => Err("Not yet supported".to_string()),
        },
        _ => Err("Not yet supported".to_string()),
    };

    println!("output: {:?}", output);
}

pub fn app<'a>() -> App<'a> {
    App::new("bin").subcommand(app_dec())
}

pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("dec", matches)) => {
            handle_dec(matches);
        }
        _ => {
            println!("No match in bin");
        }
    }
}
