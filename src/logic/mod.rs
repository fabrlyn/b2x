use std::marker::PhantomData;

use self::common::{
    BigEndian, BinToDec, Compact, DefaultBitAlignment, ExactBits, LittleEndian, Spaced,
    StandardBits, ToDec, VariableBits,
};

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

// LITTLE_ENDIAN, UNSIGNED, COMPACT, N

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

impl ToDec<Vec<u16>> for BinToDec<&str, StandardBits<u16>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u16> {
        self.input
            .as_bytes()
            .chunks(16)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 16)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u32>> for BinToDec<&str, StandardBits<u32>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u32> {
        self.input
            .as_bytes()
            .chunks(32)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 32)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

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

impl ToDec<Vec<u128>> for BinToDec<&str, StandardBits<u128>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u128> {
        self.input
            .as_bytes()
            .chunks(128)
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 128)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u128::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

// LITTLE_ENDIAN, UNSIGNED, SPACED, N

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

impl ToDec<Vec<u16>> for BinToDec<&str, StandardBits<u16>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u16> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_group_size(a, 16))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u16>> for BinToDec<&str, VariableBits<u16>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u16> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, 16))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u16>> for BinToDec<&str, ExactBits<u16>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u16> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u32>> for BinToDec<&str, StandardBits<u32>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u32> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_group_size(a, 32))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u32>> for BinToDec<&str, VariableBits<u32>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u32> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, 32))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u32>> for BinToDec<&str, ExactBits<u32>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u32> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
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

impl ToDec<Vec<u128>> for BinToDec<&str, StandardBits<u128>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u128> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_group_size(a, 128))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u128::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u128>> for BinToDec<&str, VariableBits<u128>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u128> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, 128))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u128::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u128>> for BinToDec<&str, ExactBits<u128>, LittleEndian, Spaced> {
    fn convert(self) -> Vec<u128> {
        self.input
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .map(|a| validate_variable_group_size(a, self.bit_alignment.1))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u128::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
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

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, BigEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
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

impl ToDec<Vec<u16>> for BinToDec<&str, StandardBits<u16>, BigEndian, Compact> {
    fn convert(self) -> Vec<u16> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 16)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u16::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u32>> for BinToDec<&str, StandardBits<u32>, BigEndian, Compact> {
    fn convert(self) -> Vec<u32> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 32)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<Vec<u64>> for BinToDec<&str, StandardBits<u64>, BigEndian, Compact> {
    fn convert(self) -> Vec<u64> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
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

impl ToDec<Vec<u128>> for BinToDec<&str, StandardBits<u128>, BigEndian, Compact> {
    fn convert(self) -> Vec<u128> {
        self.input
            .as_bytes()
            .chunks(8)
            .rev()
            .map(|a| String::from_utf8(a.to_vec()))
            .map(|a| a.map_err(|e| e.to_string()))
            .map(|a| a.and_then(|b| validate_group_size(b, 128)))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u128::from_str_radix(s, 2).unwrap())
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

fn validate_group_size(source: String, target_size: usize) -> Result<String, String> {
    if source.len() != target_size {
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
    fn bin_to_dec_std_bits_u8_little_endian_spaced() {
        let input: &str = &vec!["00001010", "00001100"].join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().spaced().convert();

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
    fn bin_to_dec_exact_bits_u8_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u8().spaced().exact(4).unwrap().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u16_little_endian_compact() {
        let input: &str = &vec!["0000000000001010", "0000000000001100"]
            .into_iter()
            .collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u16().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u16_little_endian_spaced() {
        let input: &str = &vec!["0000000000001010", "0000000000001100"].join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u16().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u16_little_endian_spaced() {
        let input: &str = &vec!["10", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u16().spaced().variable().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u16_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u16()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u32_little_endian_compact() {
        let input: &str = &vec![
            "00000000000000000000000000001010",
            "00000000000000000000000000001100",
        ]
        .into_iter()
        .collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u32().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u32_little_endian_spaced() {
        let input: &str = &vec![
            "00000000000000000000000000001010",
            "00000000000000000000000000001100",
        ]
        .join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u32().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_variable_bits_u32_little_endian_spaced() {
        let input: &str = &vec!["10", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input.bin_to_dec().u32().spaced().variable().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u32_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u32()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }

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
    fn bin_to_dec_std_bits_u128_little_endian_compact() {
        let input: &str = &vec![
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001100",
        ]
        .into_iter()
        .collect::<String>();

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u128().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_std_bits_u128_little_endian_spaced() {
        let input: &str = &vec![
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010",
            "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001100",
        ]
        .join(" ");

        let expected = vec![10, 12];
        let actual: Vec<_> = input.bin_to_dec().u128().spaced().convert();

        assert_eq!(expected, actual);
    }

    #[test]
    fn bin_to_dec_exact_bits_u128_little_endian_spaced() {
        let input: &str = &vec!["0010", "1100"].join(" ");

        let expected = vec![2, 12];
        let actual: Vec<_> = input
            .bin_to_dec()
            .u128()
            .spaced()
            .exact(4)
            .unwrap()
            .convert();

        assert_eq!(expected, actual);
    }
}
