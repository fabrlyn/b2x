use std::str::Chars;

use crate::bit_size::BitSize;

pub const ONE: char = '1';
pub const ZERO: char = '0';

pub fn is_one(c: &char) -> bool {
    *c == ONE
}

pub fn is_zero(c: &char) -> bool {
    *c == ZERO
}

pub trait AddBit
where
    Self: Sized,
{
    fn add_bit(self, index: usize) -> Self;
}

#[derive(Debug, PartialEq)]
pub enum FromBitsError {
    InvalidSize,
}

pub fn from_bits<'a, T>(input: Chars<'a>) -> Result<T, FromBitsError>
where
    T: Default + AddBit + BitSize,
{
    if input.as_str().len() != T::bit_size() {
        return Err(FromBitsError::InvalidSize);
    }

    Ok(input
        .enumerate()
        .filter(|(_, value)| is_one(value))
        .fold(T::default(), |acc, (index, _)| acc.add_bit(index)))
}
