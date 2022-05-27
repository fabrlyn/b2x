use std::marker::PhantomData;

use self::common::{
    BigEndian, BinToDec, Compact, DefaultBitAlignment, ExactBits, LittleEndian, Spaced,
    StandardBits, ToDec, VariableBits,
};

pub mod bin_to_dec;
pub mod common;
pub mod test;

impl<I, O, F> BinToDec<I, O, LittleEndian, F> {
    pub fn big_endian(self) -> BinToDec<I, O, BigEndian, F> {
        BinToDec {
            input: self.input,
            bit_alignment: self.bit_alignment,
            endian_marker: PhantomData,
            format_marker: PhantomData,
        }
    }
}

impl<I, T, E> BinToDec<I, T, E, Compact> {
    pub fn spaced(self) -> BinToDec<I, T, E, Spaced> {
        BinToDec {
            input: self.input,
            bit_alignment: self.bit_alignment,
            endian_marker: self.endian_marker,
            format_marker: PhantomData,
        }
    }
}

impl<I, T, E, F> BinToDec<I, StandardBits<T>, E, F> {
    pub fn variable(self) -> BinToDec<I, VariableBits<T>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: VariableBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }
}

impl<I, E, F> BinToDec<I, StandardBits<u8>, E, F> {
    pub fn exact(self, n: u8) -> Result<BinToDec<I, ExactBits<u8>, E, F>, String> {
        if n > 7 {
            Err("Must be one bit or smaller then the expected output type".to_string())
        } else {
            Ok(BinToDec {
                input: self.input,
                bit_alignment: ExactBits(PhantomData, n),
                endian_marker: self.endian_marker,
                format_marker: self.format_marker,
            })
        }
    }
}

impl<I, E, F> BinToDec<I, StandardBits<u16>, E, F> {
    pub fn exact(self, n: u8) -> Result<BinToDec<I, ExactBits<u16>, E, F>, String> {
        if n > 15 {
            Err("Must be one bit or smaller then the expected output type".to_string())
        } else {
            Ok(BinToDec {
                input: self.input,
                bit_alignment: ExactBits(PhantomData, n),
                endian_marker: self.endian_marker,
                format_marker: self.format_marker,
            })
        }
    }
}

impl<I, E, F> BinToDec<I, StandardBits<u32>, E, F> {
    pub fn exact(self, n: u8) -> Result<BinToDec<I, ExactBits<u32>, E, F>, String> {
        if n > 31 {
            Err("Must be one bit or smaller then the expected output type".to_string())
        } else {
            Ok(BinToDec {
                input: self.input,
                bit_alignment: ExactBits(PhantomData, n),
                endian_marker: self.endian_marker,
                format_marker: self.format_marker,
            })
        }
    }
}

impl<I, E, F> BinToDec<I, StandardBits<u64>, E, F> {
    pub fn exact(self, n: u8) -> Result<BinToDec<I, ExactBits<u64>, E, F>, String> {
        if n > 63 {
            Err("Must be one bit or smaller then the expected output type".to_string())
        } else {
            Ok(BinToDec {
                input: self.input,
                bit_alignment: ExactBits(PhantomData, n),
                endian_marker: self.endian_marker,
                format_marker: self.format_marker,
            })
        }
    }
}

impl<I, E, F> BinToDec<I, StandardBits<u128>, E, F> {
    pub fn exact(self, n: u8) -> Result<BinToDec<I, ExactBits<u128>, E, F>, String> {
        if n > 127 {
            Err("Must be one bit or smaller then the expected output type".to_string())
        } else {
            Ok(BinToDec {
                input: self.input,
                bit_alignment: ExactBits(PhantomData, n),
                endian_marker: self.endian_marker,
                format_marker: self.format_marker,
            })
        }
    }
}

impl<I, E, F> BinToDec<I, DefaultBitAlignment, E, F> {
    pub fn u8(self) -> BinToDec<I, StandardBits<u8>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn u16(self) -> BinToDec<I, StandardBits<u16>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn u32(self) -> BinToDec<I, StandardBits<u32>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn u64(self) -> BinToDec<I, StandardBits<u64>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn u128(self) -> BinToDec<I, StandardBits<u128>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn i8(self) -> BinToDec<I, StandardBits<i8>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn i16(self) -> BinToDec<I, StandardBits<i16>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn i32(self) -> BinToDec<I, StandardBits<i32>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn i64(self) -> BinToDec<I, StandardBits<i64>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }

    pub fn i128(self) -> BinToDec<I, StandardBits<i128>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }
}

// LITTLE_ENDIAN, SIGNED, COMPACT, N

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

impl ToDec<Vec<i16>> for BinToDec<&str, StandardBits<i16>, LittleEndian, Compact> {
    fn convert(self) -> Vec<i16> {
        self.input
            .as_bytes()
            .chunks(16)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 16)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i16::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

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

impl ToDec<Vec<i64>> for BinToDec<&str, StandardBits<i64>, LittleEndian, Compact> {
    fn convert(self) -> Vec<i64> {
        self.input
            .as_bytes()
            .chunks(64)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 64)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i64::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<i128>> for BinToDec<&str, StandardBits<i128>, LittleEndian, Compact> {
    fn convert(self) -> Vec<i128> {
        self.input
            .as_bytes()
            .chunks(128)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 128)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i128::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

// BIG_ENDIAN, SIGNED, COMPACT, N

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

impl ToDec<Vec<i16>> for BinToDec<&str, StandardBits<i16>, BigEndian, Compact> {
    fn convert(self) -> Vec<i16> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 16)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i16::from_str_radix(&s, 2).unwrap())
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

impl ToDec<Vec<i64>> for BinToDec<&str, StandardBits<i64>, BigEndian, Compact> {
    fn convert(self) -> Vec<i64> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 64)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i64::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<i128>> for BinToDec<&str, StandardBits<i128>, BigEndian, Compact> {
    fn convert(self) -> Vec<i128> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 128)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(to_signed_string)
            .map(|s| i128::from_str_radix(&s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<u8> for BinToDec<&str, StandardBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> u8 {
        u8::from_str_radix(self.input, 2).unwrap()
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
