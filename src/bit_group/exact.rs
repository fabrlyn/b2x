use std::marker::PhantomData;

use crate::{binary, bit_size::BitSize, from_binary::FromBinaryString};

use super::common::{
    BigEndian, DecimalConverter, DecimalConverterError, DefaultBitGroup, FromBinary, LittleEndian,
};

// Structs

pub struct ExactBitGroup<T>(PhantomData<T>);

// Impls

impl<'a, E> DecimalConverter<'a, DefaultBitGroup, E> {
    fn into_output<N>(self) -> DecimalConverter<'a, ExactBitGroup<N>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        self.into_output()
    }

    pub fn i8(self) -> DecimalConverter<'a, ExactBitGroup<i8>, E> {
        self.into_output()
    }

    pub fn u16(self) -> DecimalConverter<'a, ExactBitGroup<u16>, E> {
        self.into_output()
    }
    pub fn i16(self) -> DecimalConverter<'a, ExactBitGroup<i16>, E> {
        self.into_output()
    }
    pub fn u32(self) -> DecimalConverter<'a, ExactBitGroup<u32>, E> {
        self.into_output()
    }

    pub fn i32(self) -> DecimalConverter<'a, ExactBitGroup<i32>, E> {
        self.into_output()
    }

    pub fn u64(self) -> DecimalConverter<'a, ExactBitGroup<u64>, E> {
        self.into_output()
    }

    pub fn i64(self) -> DecimalConverter<'a, ExactBitGroup<i64>, E> {
        self.into_output()
    }

    pub fn u128(self) -> DecimalConverter<'a, ExactBitGroup<u128>, E> {
        self.into_output()
    }

    pub fn i128(self) -> DecimalConverter<'a, ExactBitGroup<i128>, E> {
        self.into_output()
    }
}

impl<'a, T> FromBinary for DecimalConverter<'a, ExactBitGroup<T>, LittleEndian>
where
    T: BitSize + FromBinaryString,
{
    type Output = Result<Vec<<T as FromBinaryString>::Output>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        binary::little_endian_to_decimal::<T>(self.input).map_err(DecimalConverterError::ParseError)
    }
}

impl<'a, T> FromBinary for DecimalConverter<'a, ExactBitGroup<T>, BigEndian>
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
    use crate::bit_group::common::DecimalConverterExt;

    use super::*;

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
