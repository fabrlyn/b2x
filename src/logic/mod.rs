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
