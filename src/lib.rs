use std::marker::PhantomData;

pub trait BitGroup {}

pub struct ExactBitGroup<T>(PhantomData<T>);

pub struct VariableBitGroup(u8);

pub struct SpacedBitGroup<T>(PhantomData<T>);

impl<T> BitGroup for ExactBitGroup<T> {}
impl BitGroup for VariableBitGroup {}
impl<T> BitGroup for SpacedBitGroup<T> {}

#[derive(Debug)]
pub struct DecimalConverter<'a, O, E> {
    input: &'a str,
    output_marker: PhantomData<O>,
    endian_marker: PhantomData<E>,
}

pub struct BigEndian;
pub struct LittleEndian;

#[derive(Debug, PartialEq)]
pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
    ParseError(String),
}

pub trait DecimalConverterExt {
    type Output;
    type Endian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian>;
}

impl DecimalConverterExt for &str {
    type Output = SpacedBitGroup<u64>;
    type Endian = LittleEndian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian> {
        DecimalConverter::new(self)
    }
}

impl DecimalConverterExt for String {
    type Output = SpacedBitGroup<u64>;
    type Endian = LittleEndian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian> {
        DecimalConverter::new(self)
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

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn i8(self) -> DecimalConverter<'a, ExactBitGroup<i8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn u16(self) -> DecimalConverter<'a, ExactBitGroup<u16>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn i16(self) -> DecimalConverter<'a, ExactBitGroup<i16>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn u32(self) -> DecimalConverter<'a, ExactBitGroup<u32>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn i32(self) -> DecimalConverter<'a, ExactBitGroup<i32>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn u64(self) -> DecimalConverter<'a, ExactBitGroup<u64>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, T, E> DecimalConverter<'a, T, E> {
    pub fn i64(self) -> DecimalConverter<'a, ExactBitGroup<i64>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, O, E> DecimalConverter<'a, O, E> {
    pub fn bit_group_size(
        self,
        group_size: u8,
    ) -> Result<DecimalConverter<'a, VariableBitGroup, E>, DecimalConverterError> {
        if !(2..=64).contains(&group_size) {
            return Err(DecimalConverterError::GroupSizeOutOfBounds);
        }

        Ok(DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        })
    }
}

impl<'a, E> DecimalConverter<'a, i64, E> {
    pub fn unsigned(self) -> DecimalConverter<'a, u64, E> {
        DecimalConverter {
            input: self.input,
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

impl<'a, O> DecimalConverter<'a, O, LittleEndian> {
    pub fn big_endian(self) -> DecimalConverter<'a, O, BigEndian> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O> DecimalConverter<'a, O, BigEndian> {
    pub fn little_endian(self) -> DecimalConverter<'a, O, LittleEndian> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

pub trait FromBinary {
    type Output;

    fn from_binary(&self) -> Self::Output;
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

#[cfg(test)]
mod tests {
    use crate::{DecimalConverterError, DecimalConverterExt, FromBinary};

    #[test]
    fn multiple_decimal_exact_u8_from_binary() {
        let input = format!("{}{}", "11111111", "00001110");
        let actual = input.decimal().u8().from_binary();
        let expected = Ok(vec![u8::MAX, 14]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_decimal_exact_i8_from_binary() {
        let input = format!("{}{}", "10000000", "00001110");
        let actual = input.decimal().i8().from_binary();
        let expected = Ok(vec![i8::MIN, 14]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_incomplete_signed_decimal_from_binary() {
        let actual = "1110".decimal().signed().from_binary();
        let expected = Ok(vec![14]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_signed_decimal_from_binary() {
        let actual = "1000000000000000000000000000000000000000000000000000000000000000"
            .decimal()
            .signed()
            .from_binary();
        let expected = Ok(vec![i64::MIN]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn single_decimal_from_binary() {
        let actual = "1010".decimal().from_binary();
        let expected = Ok(vec![10u64]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_decimal_from_binary() {
        let actual = "11010 11 1011 1010".decimal().from_binary();
        let expected = Ok(vec![26, 3, 11, 10]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn failed_decimal_from_binary_with_parse_error() {
        let actual = "1101 101a".decimal().from_binary();
        let expected = Err(DecimalConverterError::ParseError("101a".to_string()));
        assert_eq!(expected, actual);
    }
}
