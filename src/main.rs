use std::error::Error;

fn main() {
    let matches = clap::App::new("b2x")
        .arg(clap::Arg::new("input").required(true))
        .get_matches();

    let sum: i64 = matches
        .value_of("input")
        .unwrap()
        .chars()
        .rev()
        .map(char_to_i64)
        .enumerate()
        .map(|(i, d)| adjust_to_nth_place(i, d.unwrap()))
        .sum();

    println!("{}", sum);
}

fn adjust_to_nth_place(nth_place: usize, number: i64) -> i64 {
    number << nth_place
}

fn char_to_i64(c: char) -> Result<i64, Box<dyn Error>> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        _ => Err("Not valid binary digit".into()),
    }
}
