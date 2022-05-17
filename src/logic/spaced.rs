use std::marker::PhantomData;

use crate::{binary, bit_size::BitSize, from_binary::FromBinaryString};

use super::common::{
    BigEndian, DecimalConverter, DecimalConverterError, DefaultBitGroup, FromBinary, LittleEndian,
};

// Structs

pub struct SpacedBitGroup<T>(PhantomData<T>);

// Impls

impl<'a, E> DecimalConverter<'a, DefaultBitGroup, E> {
    fn into_spaced<N>(self) -> DecimalConverter<'a, SpacedBitGroup<N>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }

    pub fn spaced_u8(self) -> DecimalConverter<'a, SpacedBitGroup<u8>, E> {
        self.into_spaced()
    }

    pub fn spaced_i8(self) -> DecimalConverter<'a, SpacedBitGroup<i8>, E> {
        self.into_spaced()
    }

    pub fn spaced_u16(self) -> DecimalConverter<'a, SpacedBitGroup<u16>, E> {
        self.into_spaced()
    }

    pub fn spaced_i16(self) -> DecimalConverter<'a, SpacedBitGroup<i16>, E> {
        self.into_spaced()
    }

    pub fn spaced_u32(self) -> DecimalConverter<'a, SpacedBitGroup<u32>, E> {
        self.into_spaced()
    }

    pub fn spaced_i32(self) -> DecimalConverter<'a, SpacedBitGroup<i32>, E> {
        self.into_spaced()
    }

    pub fn spaced_u64(self) -> DecimalConverter<'a, SpacedBitGroup<u64>, E> {
        self.into_spaced()
    }

    pub fn spaced_i64(self) -> DecimalConverter<'a, SpacedBitGroup<i64>, E> {
        self.into_spaced()
    }

    pub fn spaced_u128(self) -> DecimalConverter<'a, SpacedBitGroup<u128>, E> {
        self.into_spaced()
    }

    pub fn spaced_i128(self) -> DecimalConverter<'a, SpacedBitGroup<i128>, E> {
        self.into_spaced()
    }
}

impl<'a, T> FromBinary for DecimalConverter<'a, SpacedBitGroup<T>, LittleEndian>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::spaced_little_endian_to_decimal::<T>(self.input)
            .map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T> FromBinary for DecimalConverter<'a, SpacedBitGroup<T>, BigEndian>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::little_endian_to_decimal::<T>(self.input).map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T> DecimalConverter<'a, SpacedBitGroup<T>, LittleEndian> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use crate::bit_group::common::{DecimalConverterExt, FromBinary};

    #[test]
    fn from_u8_binary_to_decimal() {
        let actual = "1010 10 00001011".decimal().spaced_u8().from_binary();
        let expected = Ok(vec![10, 2, 11]);
        assert_eq!(expected, actual);
    }
}
