use crate::logic::common::{
    BigEndian, BinToDec, Compact, ExactBits, LittleEndian, Spaced, StandardBits, ToDec,
    VariableBits,
};

impl ToDec<u8> for BinToDec<&str, StandardBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> u8 {
        u8::from_str_radix(self.input, 2).unwrap()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(8)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 8)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, ExactBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(self.bit_alignment.1 as usize)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, self.bit_alignment.1)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_group_size(a, 8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, VariableBits<u8>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, 8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, ExactBits<u8>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, BigEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(8)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 8)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .map(|s| u8::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, ExactBits<u8>, BigEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(self.bit_alignment.1 as usize)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, self.bit_alignment.1)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .map(|s| u8::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_group_size(a, 8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, ExactBits<u8>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, VariableBits<u8>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u8> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_variable_group_size(a, 8))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

fn validate_variable_group_size(source: String, max_size: u8) -> Result<String, String> {
    if source.len() > max_size as usize {
        Err(format!(
            "Source {} has size {}, max size is {}",
            source,
            source.len(),
            max_size
        ))
    } else {
        Ok(source)
    }
}

fn validate_group_size(source: String, target_size: u8) -> Result<String, String> {
    if source.len() != target_size as usize {
        Err(format!(
            "Source {} has size {}, needs to be {}",
            source,
            source.len(),
            target_size
        ))
    } else {
        Ok(source)
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::common::{BinToDecExt, ToDec};

    #[test]
    fn bin_to_dec_std_bits_u8_little_endian_compact() {
        let input: &str = &vec!["00001010", "00001100"].into_iter().collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u8_little_endian_compact() {
        let input: &str = &vec!["1010", "1100"].into_iter().collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().exact(4).unwrap().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u8_little_endian_spaced() {
        let input: &str = &vec!["00001010", "00001100"].join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u8_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().spaced().exact(4).unwrap().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u8_little_endian_spaced() {
        let input: &str = &vec!["10", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().spaced().variable().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u8_big_endian_compact() {
        let input: &str = &vec!["01000000", "00110000"].into_iter().collect::<String>();

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().big_endian().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u8_big_endian_compact() {
        let input: &str = &vec!["0100", "0011"].join("");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u8()
            .big_endian()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u8_big_endian_spaced() {
        let input: &str = &vec!["01000000", "00110000"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().big_endian().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u8_big_endian_spaced() {
        let input: &str = &vec!["0100", "0011"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u8()
            .big_endian()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u8_big_endian_spaced() {
        let input: &str = &vec!["0100", "0011"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u8()
            .big_endian()
            .spaced()
            .variable()
            .convert();

        assert_eq!(expected, actual);
    }
}
