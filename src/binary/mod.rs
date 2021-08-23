mod chunk;
use std::error::Error;

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}

fn char_to_u8(c: char) -> Result<Option<u8>, Box<dyn Error>> {
    match c {
        '0' => Ok(Some(0)),
        '1' => Ok(Some(1)),
        ' ' => Ok(None),
        _ => Err("Not valid binary digit".into()),
    }
}

pub fn chunk_binary_list(
    binary_list: &[u8],
    chunk_size: usize,
) -> Result<Vec<&[u8]>, Box<dyn Error>> {
    let chunked = binary_list.chunks(chunk_size).collect::<Vec<_>>();
    if chunked.last().unwrap().len() != chunk_size {
        return Err("Not evenly chunkable".into());
    }
    Ok(chunked)
}

pub fn as_binary_list(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let input: Vec<Option<u8>> = match input.chars().map(char_to_u8).collect::<_>() {
        Ok(input) => input,
        Err(e) => return Err(e),
    };

    Ok(input.into_iter().filter_map(|v| v).collect::<_>())
}

pub fn as_big_endian_u8_list(chunked_binary_list: &[&[u8]]) -> Vec<u8> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_big_endian_u8(c))
        .collect()
}

pub fn as_big_endian_u16_list(chunked_binary_list: &[&[u8]]) -> Vec<u16> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_big_endian_u16(c))
        .collect()
}

pub fn as_big_endian_u32_list(chunked_binary_list: &[&[u8]]) -> Vec<u32> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_big_endian_u32(c))
        .collect()
}

pub fn as_big_endian_u64_list(chunked_binary_list: &[&[u8]]) -> Vec<u64> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_big_endian_u64(c))
        .collect()
}

pub fn as_little_endian_u8_list(chunked_binary_list: &[&[u8]]) -> Vec<u8> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_little_endian_u8(c))
        .collect()
}

pub fn as_little_endian_u16_list(chunked_binary_list: &[&[u8]]) -> Vec<u16> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_little_endian_u16(c))
        .collect()
}

pub fn as_little_endian_u32_list(chunked_binary_list: &[&[u8]]) -> Vec<u32> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_little_endian_u32(c))
        .collect()
}

pub fn as_little_endian_u64_list(chunked_binary_list: &[&[u8]]) -> Vec<u64> {
    chunked_binary_list
        .iter()
        .map(|c| chunk::as_little_endian_u64(c))
        .collect()
}

pub fn to_big_endian_u8(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 8)?;
    let value_list = as_big_endian_u8_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_big_endian_u16(input: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 16)?;
    let value_list = as_big_endian_u16_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_big_endian_u32(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 32)?;
    let value_list = as_big_endian_u32_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_big_endian_u64(input: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 64)?;
    let value_list = as_big_endian_u64_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_little_endian_u8(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 8)?;
    let value_list = as_little_endian_u8_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_little_endian_u16(input: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 16)?;
    let value_list = as_little_endian_u16_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_little_endian_u32(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 32)?;
    let value_list = as_little_endian_u32_list(&chunked_binary_list);
    Ok(value_list)
}

pub fn to_little_endian_u64(input: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let binary_list = as_binary_list(input)?;
    let chunked_binary_list = chunk_binary_list(&binary_list, 64)?;
    let value_list = as_little_endian_u64_list(&chunked_binary_list);
    Ok(value_list)
}
