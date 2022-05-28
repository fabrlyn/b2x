use crate::logic::common::{BigEndian, BinToDec, Compact, StandardBits, ToDec, LittleEndian};

impl ToDec<Vec<i32>> for BinToDec<&str, StandardBits<i32>, LittleEndian, Compact> {
    fn convert(self) -> Vec<i32> {
        self.input
            .as_bytes()
            .chunks(32)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 32)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i32::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<i32>> for BinToDec<&str, StandardBits<i32>, BigEndian, Compact> {
    fn convert(self) -> Vec<i32> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 32)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i32::from_str_radix(&s, 2).unwrap())
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
