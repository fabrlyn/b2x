use std::marker::PhantomData;

use super::common::{DecimalConverter, DecimalConverterError, DefaultBitGroup, FromBinary};

// Structs

pub struct ExactBitGroup<T>(PhantomData<T>);

// Impls

impl<'a, E> DecimalConverter<'a, DefaultBitGroup, E> {
    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn i8(self) -> DecimalConverter<'a, ExactBitGroup<i8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn u16(self) -> DecimalConverter<'a, ExactBitGroup<u16>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
    pub fn i16(self) -> DecimalConverter<'a, ExactBitGroup<i16>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
    pub fn u32(self) -> DecimalConverter<'a, ExactBitGroup<u32>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn i32(self) -> DecimalConverter<'a, ExactBitGroup<i32>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn u64(self) -> DecimalConverter<'a, ExactBitGroup<u64>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn i64(self) -> DecimalConverter<'a, ExactBitGroup<i64>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn u128(self) -> DecimalConverter<'a, ExactBitGroup<u128>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }

    pub fn i128(self) -> DecimalConverter<'a, ExactBitGroup<i128>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<i8>, E> {
    type Output = Result<Vec<i8>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(8)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| {
                u8::from_str_radix(&s, 2)
                    .map_err(|_| DecimalConverterError::ParseError(s))
                    .map(|i| i as i8)
            })
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<u8>, E> {
    type Output = Result<Vec<u8>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(8)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| u8::from_str_radix(&s, 2).map_err(|_| DecimalConverterError::ParseError(s)))
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<u16>, E> {
    type Output = Result<Vec<u16>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(16)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| u16::from_str_radix(&s, 2).map_err(|_| DecimalConverterError::ParseError(s)))
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<i16>, E> {
    type Output = Result<Vec<i16>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(16)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| {
                u16::from_str_radix(&s, 2)
                    .map_err(|_| DecimalConverterError::ParseError(s))
                    .map(|i| i as i16)
            })
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<u32>, E> {
    type Output = Result<Vec<u32>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(32)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| u32::from_str_radix(&s, 2).map_err(|_| DecimalConverterError::ParseError(s)))
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<i32>, E> {
    type Output = Result<Vec<i32>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(32)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| {
                u32::from_str_radix(&s, 2)
                    .map_err(|_| DecimalConverterError::ParseError(s))
                    .map(|i| i as i32)
            })
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<u64>, E> {
    type Output = Result<Vec<u64>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(64)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| u64::from_str_radix(&s, 2).map_err(|_| DecimalConverterError::ParseError(s)))
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<i64>, E> {
    type Output = Result<Vec<i64>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(64)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| {
                u64::from_str_radix(&s, 2)
                    .map_err(|_| DecimalConverterError::ParseError(s))
                    .map(|i| i as i64)
            })
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<u128>, E> {
    type Output = Result<Vec<u128>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(128)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| u128::from_str_radix(&s, 2).map_err(|_| DecimalConverterError::ParseError(s)))
            .collect()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, ExactBitGroup<i128>, E> {
    type Output = Result<Vec<i128>, DecimalConverterError>;

    fn from_binary(&self) -> Self::Output {
        let input: Vec<_> = self.input.chars().map(|c| c as u8).collect();
        input
            .as_slice()
            .chunks(128)
            .flat_map(|s| {
                String::from_utf8(s.to_vec())
                    .map_err(|_| DecimalConverterError::ParseError(format!("{:?}", s)))
            })
            .map(|s| {
                u128::from_str_radix(&s, 2)
                    .map_err(|_| DecimalConverterError::ParseError(s))
                    .map(|i| i as i128)
            })
            .collect()
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
