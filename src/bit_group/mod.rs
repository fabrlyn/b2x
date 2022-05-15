use std::marker::PhantomData;

use self::common::{BigEndian, DecimalConverter, LittleEndian};

mod common;
mod exact;
mod spaced;
mod variable;

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

#[cfg(test)]
mod tests {
    /*

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
    */
}
