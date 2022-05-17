use std::marker::PhantomData;

use crate::{binary, bit_size::BitSize, from_binary::FromBinaryString};

use super::common::{
    BigEndian, DecimalConverter, DecimalConverterError, DefaultBitAlignment, FromBinary,
    LittleEndian, Symmetric,
};

// Structs

pub struct ExactBitGroup<T>(PhantomData<T>);

// Impls

impl<'a, E, S, F> DecimalConverter<'a, DefaultBitAlignment, E, S, F> {
    fn into_exact<N>(self) -> DecimalConverter<'a, Symmetric<N>, E, S, F> {
        DecimalConverter {
            input: self.input,
            bit_alignment: Symmetric::<N>(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
            signifier: self.signifier,
        }
    }

    pub fn u8(self) -> DecimalConverter<'a, Symmetric<u8>, E, S, F> {
        self.into_exact()
    }

    pub fn i8(self) -> DecimalConverter<'a, Symmetric<i8>, E, S, F> {
        self.into_exact()
    }

    pub fn u16(self) -> DecimalConverter<'a, Symmetric<u16>, E, S, F> {
        self.into_exact()
    }
    pub fn i16(self) -> DecimalConverter<'a, Symmetric<i16>, E, S, F> {
        self.into_exact()
    }
    pub fn u32(self) -> DecimalConverter<'a, Symmetric<u32>, E, S, F> {
        self.into_exact()
    }

    pub fn i32(self) -> DecimalConverter<'a, Symmetric<i32>, E, S, F> {
        self.into_exact()
    }

    pub fn u64(self) -> DecimalConverter<'a, Symmetric<u64>, E, S, F> {
        self.into_exact()
    }

    pub fn i64(self) -> DecimalConverter<'a, Symmetric<i64>, E, S, F> {
        self.into_exact()
    }

    pub fn u128(self) -> DecimalConverter<'a, Symmetric<u128>, E, S, F> {
        self.into_exact()
    }

    pub fn i128(self) -> DecimalConverter<'a, Symmetric<i128>, E, S, F> {
        self.into_exact()
    }
}

impl<'a, T, S, F> FromBinary for DecimalConverter<'a, ExactBitGroup<T>, LittleEndian, S, F>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::little_endian_to_decimal::<T>(self.input).map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T, S, F> FromBinary for DecimalConverter<'a, ExactBitGroup<T>, BigEndian, S, F>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::little_endian_to_decimal::<T>(self.input).map_err(DecimalConverterError::ParseError)
    }
}

// Tests

#[cfg(test)]
mod tests {
    use crate::logic::common::DecimalConverterExt;

    #[test]
    fn from_u8_binary_to_decimal() {
        let actual = format!("{}{}", "00001011", "00001010")
            .decimal()
            .u8()
            .from_binary();
        let expected = Ok(vec![11, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_i8_binary_to_decimal() {
        let actual = format!("{}{}", "11111111", "10000000")
            .decimal()
            .i8()
            .from_binary();
        let expected = Ok(vec![-1, i8::MIN]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_u16_binary_to_decimal() {
        let actual = format!("{}{}", "1111111111111111", "0000000000001010")
            .decimal()
            .u16()
            .from_binary();
        let expected = Ok(vec![u16::MAX, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_i16_binary_to_decimal() {
        let actual = format!("{}{}", "1111111111111111", "1000000000000000")
            .decimal()
            .i16()
            .from_binary();
        let expected = Ok(vec![-1, i16::MIN]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_u32_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "11111111111111111111111111111111", "00000000000000000000000000001010"
        )
        .decimal()
        .u32()
        .from_binary();
        let expected = Ok(vec![u32::MAX, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_i32_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "11111111111111111111111111111111", "10000000000000000000000000000000"
        )
        .decimal()
        .i32()
        .from_binary();
        let expected = Ok(vec![-1, i32::MIN]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_u64_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "1111111111111111111111111111111111111111111111111111111111111111",
            "0000000000000000000000000000000000000000000000000000000000001010"
        )
        .decimal()
        .u64()
        .from_binary();
        let expected = Ok(vec![u64::MAX, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_i64_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "1111111111111111111111111111111111111111111111111111111111111111",
            "1000000000000000000000000000000000000000000000000000000000000000"
        )
        .decimal()
        .i64()
        .from_binary();
        let expected = Ok(vec![-1, i64::MIN]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_u128_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010"
        )
        .decimal()
        .u128()
        .from_binary();
        let expected = Ok(vec![u128::MAX, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_i128_binary_to_decimal() {
        let actual = format!(
            "{}{}",
            "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111",
            "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        )
        .decimal()
        .i128()
        .from_binary();
        let expected = Ok(vec![-1, i128::MIN]);
        assert_eq!(expected, actual);
    }
}
