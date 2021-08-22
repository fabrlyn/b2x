mod chunk;
use std::error::Error;

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}

fn char_to_u8(c: char) -> Result<u8, Box<dyn Error>> {
    match c {
        '0' => Ok(0),
        '1' => Ok(1),
        _ => Err("Not valid binary digit".into()),
    }
}

pub fn chunk_binary_list(binary_list: &[u8], chunk_size: usize) -> Vec<&[u8]> {
    binary_list.chunks(chunk_size).collect()
}

pub fn as_binary_list(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    input.chars().map(char_to_u8).collect()
}

pub fn as_big_endian_u8_list(chunked_binary_list: &[&[u8]]) -> Vec<u8> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_big_endian_u8(c))
        .collect()
}

pub fn to_big_endian_u8(input: &str) -> Result<(), Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 8);
    let value_list = as_big_endian_u8_list(&chunked_binary_list);
    Ok(())
}
