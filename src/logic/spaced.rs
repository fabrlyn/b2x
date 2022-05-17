use std::marker::PhantomData;

use crate::{binary, bit_size::BitSize, from_binary::FromBinaryString};

use super::common::{
    BigEndian, DecimalConverter, DecimalConverterError, DefaultBitAlignment, FromBinary,
    LittleEndian, Symmetric,
};

// Structs

pub struct SpacedBitGroup<T>(PhantomData<T>);

// Impls

impl<'a, E, S, F> DecimalConverter<'a, DefaultBitAlignment, E, S, F> {
    fn into_spaced<N>(self) -> DecimalConverter<'a, Symmetric<N>, E, S, F> {
        DecimalConverter {
            input: self.input,
            bit_alignment: Symmetric::<N>(PhantomData),
            endian_marker: PhantomData,
            format_marker: PhantomData,
            signifier: PhantomData,
        }
    }

    pub fn spaced_u8(self) -> DecimalConverter<'a, Symmetric<u8>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_i8(self) -> DecimalConverter<'a, Symmetric<i8>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_u16(self) -> DecimalConverter<'a, Symmetric<u16>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_i16(self) -> DecimalConverter<'a, Symmetric<i16>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_u32(self) -> DecimalConverter<'a, Symmetric<u32>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_i32(self) -> DecimalConverter<'a, Symmetric<i32>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_u64(self) -> DecimalConverter<'a, Symmetric<u64>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_i64(self) -> DecimalConverter<'a, Symmetric<i64>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_u128(self) -> DecimalConverter<'a, Symmetric<u128>, E, S, F> {
        self.into_spaced()
    }

    pub fn spaced_i128(self) -> DecimalConverter<'a, Symmetric<i128>, E, S, F> {
        self.into_spaced()
    }
}

impl<'a, T, S, F> FromBinary for DecimalConverter<'a, SpacedBitGroup<T>, LittleEndian, S, F>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::spaced_little_endian_to_decimal::<T>(self.input)
            .map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T, S, F> FromBinary for DecimalConverter<'a, SpacedBitGroup<T>, BigEndian, S, F>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::little_endian_to_decimal::<T>(self.input).map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T, S, F> DecimalConverter<'a, Symmetric<T>, LittleEndian, S, F> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bit_alignment: Symmetric::<T>(PhantomData),
            endian_marker: PhantomData,
            format_marker: PhantomData,
            signifier: PhantomData,
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use crate::logic::common::{DecimalConverterExt, FromBinary};

    #[test]
    fn from_u8_binary_to_decimal() {
        let actual = "1010 10 00001011".decimal().spaced_u8().from_binary();
        let expected = Ok(vec![10, 2, 11]);
        assert_eq!(expected, actual);
    }
}
