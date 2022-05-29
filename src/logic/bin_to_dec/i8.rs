use crate::logic::common::{BigEndian, BinToDec, Compact, LittleEndian, StandardBits, ToDec};

impl ToDec<Vec<i8>> for BinToDec<&str, StandardBits<i8>, LittleEndian, Compact> {
    fn convert(self) -> Vec<i8> {
        self.input
            .as_bytes()
            .chunks(8)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 8)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i8::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<i8>> for BinToDec<&str, StandardBits<i8>, BigEndian, Compact> {
    fn convert(self) -> Vec<i8> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 8)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i8::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

fn to_signed_string(s: String) -> String {
    if s.starts_with('1') {
        format!("-{}", s.chars().skip(1).collect::<String>())
    } else {
        format!("+{}", s.chars().skip(1).collect::<String>())
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
