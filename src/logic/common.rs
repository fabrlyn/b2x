use std::marker::PhantomData;

// Types

pub trait ToDec<T> {
    fn convert(self) -> T;
}

pub type DefaultBitAlignment = StandardBits<u32>;

// Traits

pub trait BinToDecExt {
    type Input;
    type BitAlignment;
    type Endian;
    type Format;

    fn bin_to_dec(self) -> BinToDec<Self::Input, Self::BitAlignment, Self::Endian, Self::Format>;
}

// Structs

// BitAlignent

pub struct ExactBits<T>(pub PhantomData<T>, pub u8);

pub struct VariableBits<T>(pub PhantomData<T>);

pub struct StandardBits<T>(pub PhantomData<T>);

// Format

pub struct Spaced;

pub struct Compact;

// Endianess

pub struct BigEndian;

pub struct LittleEndian;

#[derive(Debug)]
pub struct BinToDec<I, B, E, F> {
    pub input: I,
    pub bit_alignment: B,
    pub endian_marker: PhantomData<E>,
    pub format_marker: PhantomData<F>,
}

// Enums

#[derive(Debug, PartialEq)]
pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
    ParseError(String),
}

// Impls

impl<'a> BinToDecExt for &'a str {
    type Input = &'a str;
    type BitAlignment = DefaultBitAlignment;
    type Endian = LittleEndian;
    type Format = Compact;

    fn bin_to_dec(self) -> BinToDec<Self::Input, Self::BitAlignment, Self::Endian, Self::Format> {
        BinToDec {
            input: self,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: PhantomData,
            format_marker: PhantomData,
        }
    }
}

impl BinToDecExt for String {
    type Input = String;
    type BitAlignment = DefaultBitAlignment;
    type Endian = LittleEndian;
    type Format = Compact;

    fn bin_to_dec(self) -> BinToDec<Self::Input, Self::BitAlignment, Self::Endian, Self::Format> {
        BinToDec {
            input: self,
            bit_alignment: StandardBits(PhantomData),
            endian_marker: PhantomData,
            format_marker: PhantomData,
        }
    }
}
