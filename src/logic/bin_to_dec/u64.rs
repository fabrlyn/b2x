use crate::logic::common::{
    BigEndian, BinToDec, Compact, ExactBits, LittleEndian, Spaced, StandardBits, ToDec,
    VariableBits,
};

impl ToDec<Vec<u64>> for BinToDec<&str, StandardBits<u64>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u64> {
        self.input
            .as_bytes()
            .chunks(64)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 64)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, ExactBits<u64>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u64> {
        self.input
            .as_bytes()
            .chunks(self.bit_alignment.1 as usize)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, self.bit_alignment.1)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, StandardBits<u64>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_group_size(a, 64))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, VariableBits<u64>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, 64))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, ExactBits<u64>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, StandardBits<u64>, BigEndian, Compact> {
    fn convert(self) -> Vec<u64> {
        self.input
            .as_bytes()
            .chunks(64)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 64)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .map(|s| u64::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, ExactBits<u64>, BigEndian, Compact> {
    fn convert(self) -> Vec<u64> {
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
            .map(|s| u64::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, StandardBits<u64>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_group_size(a, 64))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, ExactBits<u64>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, VariableBits<u64>, BigEndian, Spaced> {
    fn convert(self) -> Vec<u64> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|s| s.chars().rev().collect::<String>())
            .map(|a| validate_variable_group_size(a, 64))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u64::from_str_radix(s, 2).unwrap())
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
    fn bin_to_dec_std_bits_u64_little_endian_compact() {
        let input: &str = &vec![
            "0000000000000000000000000000000000000000000000000000000000001010",
            "0000000000000000000000000000000000000000000000000000000000001100",
        ]
        .into_iter()
        .collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u64_little_endian_compact() {
        let input: &str = &vec!["1010", "1100"].into_iter().collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().exact(4).unwrap().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u64_little_endian_spaced() {
        let input: &str = &vec![
            "0000000000000000000000000000000000000000000000000000000000001010",
            "0000000000000000000000000000000000000000000000000000000000001100",
        ]
        .join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u64_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u64()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u64_little_endian_spaced() {
        let input: &str = &vec!["10", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().spaced().variable().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u64_big_endian_compact() {
        let input: &str = &vec![
            "0100000000000000000000000000000000000000000000000000000000000000",
            "0011000000000000000000000000000000000000000000000000000000000000",
        ]
        .into_iter()
        .collect::<String>();
        println!("{}", input);

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().big_endian().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u64_big_endian_compact() {
        let input: &str = &vec!["0100", "0011"].join("");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u64()
            .big_endian()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u64_big_endian_spaced() {
        let input: &str = &vec![
            "0100000000000000000000000000000000000000000000000000000000000000",
            "0011000000000000000000000000000000000000000000000000000000000000",
        ]
        .join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u64().big_endian().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u64_big_endian_spaced() {
        let input: &str = &vec!["0100", "0011"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u64()
            .big_endian()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u64_big_endian_spaced() {
        let input: &str = &vec!["0100", "0011"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u64()
            .big_endian()
            .spaced()
            .variable()
            .convert();

        assert_eq!(expected, actual);
    }
}
