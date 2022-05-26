use std::marker::PhantomData;

use self::common::{
    BigEndian, BinToDec, Compact, DefaultBitAlignment, LittleEndian, Spaced, StandardBits, ToDec,
};

pub mod common;

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

impl<I, E, F> BinToDec<I, DefaultBitAlignment, E, F> {
    pub fn u8(self) -> BinToDec<I, StandardBits<u8>, E, F> {
        BinToDec {
            input: self.input,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
        }
    }
}

impl ToDec<Vec<u8>> for BinToDec<&str, StandardBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> Vec<u8> {
        self.input
            .as_bytes()
            .chunks(8)
            .map(|a| String::from_utf8(a.to_vec()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .iter()
            .map(|s| u8::from_str_radix(s, 2).unwrap())
            .collect::<Vec<_>>()
    }
}

impl ToDec<u8> for BinToDec<&str, StandardBits<u8>, LittleEndian, Compact> {
    fn convert(self) -> u8 {
        u8::from_str_radix(self.input, 2).unwrap()
    }
}
