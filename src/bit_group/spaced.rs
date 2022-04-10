use std::marker::PhantomData;

use super::common::{DecimalConverter, DecimalConverterError, FromBinary, LittleEndian};

pub struct SpacedBitGroup<T>(PhantomData<T>);

impl<'a, T> DecimalConverter<'a, SpacedBitGroup<T>, LittleEndian> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, E> DecimalConverter<'a, SpacedBitGroup<u64>, E> {
    pub fn signed(self) -> DecimalConverter<'a, SpacedBitGroup<i64>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a> FromBinary for DecimalConverter<'a, SpacedBitGroup<i64>, LittleEndian> {
    type Output = Result<Vec<i64>, DecimalConverterError>; // TODO: Should maybe be Iterator<i64>

    fn from_binary(&self) -> Self::Output {
        self.input
            .split(' ')
            .map(|i| {
                u64::from_str_radix(i, 2)
                    .map(|n| n as i64)
                    .map_err(|_| DecimalConverterError::ParseError(i.to_string()))
            })
            .collect()
    }
}

impl<'a> FromBinary for DecimalConverter<'a, SpacedBitGroup<u64>, LittleEndian> {
    type Output = Result<Vec<u64>, DecimalConverterError>; // TODO: Should maybe be Iterator<i64>

    fn from_binary(&self) -> Self::Output {
        self.input
            .split(' ')
            .map(|i| {
                u64::from_str_radix(i, 2)
                    .map_err(|_| DecimalConverterError::ParseError(i.to_string()))
            })
            .collect()
    }
}
