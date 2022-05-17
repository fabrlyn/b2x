use std::marker::PhantomData;

// Types

pub type DefaultBitAlignment = Symmetric<u64>;

// Traits

pub trait DecimalConverterExt {
    type BitAlignment;
    type Endian;
    type Format;
    type Signifier;

    fn decimal(
        &self,
    ) -> DecimalConverter<Self::BitAlignment, Self::Endian, Self::Signifier, Self::Format>;
}

pub trait FromBinary {
    type Output;

    fn from_binary(&self) -> Self::Output;
}

// Structs

// BitAlignent

pub struct Asymmetric<T>(pub PhantomData<T>, pub u8);

pub struct Symmetric<T>(pub PhantomData<T>);

// Signifier

pub struct Signed;

pub struct Unsigned;

// Format

pub struct Spaced;

pub struct Compact;

// Endianess

pub struct BigEndian;

pub struct LittleEndian;

#[derive(Debug)]
pub struct DecimalConverter<'a, B, E, S, F> {
    pub input: &'a str,
    pub bit_alignment: B,
    pub endian_marker: PhantomData<E>,
    pub signifier: PhantomData<S>,
    pub format_marker: PhantomData<F>,
}

// Enums

#[derive(Debug, PartialEq)]
pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
    ParseError(String),
}

// Impls

impl DecimalConverterExt for &str {
    type BitAlignment = DefaultBitAlignment;
    type Endian = LittleEndian;
    type Format = Compact;
    type Signifier = Unsigned;

    fn decimal(
        &self,
    ) -> DecimalConverter<Self::BitAlignment, Self::Endian, Self::Signifier, Self::Format> {
        DecimalConverter::new(self)
    }
}

impl DecimalConverterExt for String {
    type BitAlignment = DefaultBitAlignment;
    type Endian = LittleEndian;
    type Format = Compact;
    type Signifier = Unsigned;

    fn decimal(
        &self,
    ) -> DecimalConverter<Self::BitAlignment, Self::Endian, Self::Signifier, Self::Format> {
        DecimalConverter::new(self)
    }
}
