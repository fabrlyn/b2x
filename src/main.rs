use std::error::Error;

use prettytable::{Cell, Row, Table};

fn main() {
    let matches = clap::App::new("b2x")
        .arg(clap::Arg::new("input").required(true))
        .get_matches();

    let values: Vec<_> = matches
        .value_of("input")
        .unwrap()
        .chars()
        .rev()
        .map(char_to_u64)
        .enumerate()
        .map(|(i, d)| (i, adjust_to_nth_place(i, d.unwrap())))
        .collect::<Vec<(usize, u64)>>()
        .into_iter()
        .rev()
        .collect();

    print_values(&values);
    println!("Equals: {}\n", values.iter().map(|(_, v)| v).sum::<u64>());
}

fn print_values(values: &[(usize, u64)]) {
    let mut table = Table::new();

    let header = Row::new(
        values
            .iter()
            .map(|(h, _)| h.to_string())
            .map(|s| Cell::new(&s))
            .collect(),
    );
    table.add_row(header);

    let values = Row::new(
        values
            .iter()
            .map(|(_, v)| v.to_string())
            .map(|s| Cell::new(&s))
            .collect(),
    );
    table.add_row(values);

    table.printstd();
}

fn adjust_to_nth_place(nth_place: usize, number: u64) -> u64 {
    number << nth_place
}

fn char_to_u64(c: char) -> Result<u64, Box<dyn Error>> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        _ => Err("Not valid binary digit".into()),
    }
}
