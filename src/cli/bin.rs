use b2x::bit_group::common::{DecimalConverterExt, FromBinary};
use clap::{App, Arg, ArgMatches};

fn app_dec<'a>() -> App<'a> {
    App::new("dec").arg(Arg::new("input").takes_value(true).required(true))
}

fn handle_dec(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap();
    let output = input.decimal().spaced_u8().from_binary();
    println!("input: {}", input);
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
