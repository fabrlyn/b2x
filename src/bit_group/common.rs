use std::marker::PhantomData;

use super::spaced::SpacedBitGroup;

// Types

pub type DefaultBitGroup = SpacedBitGroup<u64>;

// Traits

pub trait DecimalConverterExt {
    type Output;
    type Endian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian>;
}

pub trait FromBinary {
    type Output;

    fn from_binary(&self) -> Self::Output;
}

// Structs

pub struct BigEndian;

pub struct LittleEndian;

#[derive(Debug)]
pub struct DecimalConverter<'a, O, E> {
    pub input: &'a str,
    pub output_marker: PhantomData<O>,
    pub endian_marker: PhantomData<E>,
}

// Enums

#[derive(Debug, PartialEq)]
pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
    ParseError(String),
}

// Impls

impl DecimalConverterExt for &str {
    type Output = DefaultBitGroup;
    type Endian = LittleEndian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian> {
        DecimalConverter::new(self)
    }
}

impl DecimalConverterExt for String {
    type Output = DefaultBitGroup;
    type Endian = LittleEndian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian> {
        DecimalConverter::new(self)
    }
}
