use std::marker::PhantomData;

/*
let _ = "01010101".decimal().signed().big_endian().group_size(4).from_binary();
let _ = "10010101001".float32().big_endian().convert();
let _ = "10010101001".float64().big_endian().convert();
let _ = "1001 1010".hex().signed().big_endian().group_size(8).convert();
let actual: u32 = 10f32.binary().signed().big_endian().group_size().convert();
*/

pub trait BitGroup {}

pub struct ExactBitGroup<T>(PhantomData<T>);

pub struct VariableBitGroup(u8);

pub struct SpacedBitGroup<T>(PhantomData<T>);

impl<T> BitGroup for ExactBitGroup<T> {}
impl BitGroup for VariableBitGroup {}
impl<T> BitGroup for SpacedBitGroup<T> {}

#[derive(Debug)]
pub struct DecimalConverter<'a, O, E> {
    input: &'a str,
    output_marker: PhantomData<O>,
    endian_marker: PhantomData<E>,
}

pub struct BigEndian;
pub struct LittleEndian;

#[derive(Debug)]
pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
}

pub trait DecimalConverterExt {
    type Output;
    type Endian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian>;
}

impl DecimalConverterExt for &str {
    type Output = SpacedBitGroup<u64>;
    type Endian = LittleEndian;

    fn decimal(&self) -> DecimalConverter<Self::Output, Self::Endian> {
        DecimalConverter::new(self)
    }
}

impl<'a, T> DecimalConverter<'a, SpacedBitGroup<T>, LittleEndian> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, E> DecimalConverter<'a, ExactBitGroup<i8>, E> {
    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, E> DecimalConverter<'a, VariableBitGroup, E> {
    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, E> DecimalConverter<'a, ExactBitGroup<u64>, E> {
    pub fn u8(self) -> DecimalConverter<'a, ExactBitGroup<u8>, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        }
    }
}

impl<'a, O, E> DecimalConverter<'a, O, E> {
    pub fn bit_group_size(
        self,
        group_size: u8,
    ) -> Result<DecimalConverter<'a, VariableBitGroup, E>, DecimalConverterError> {
        if !(2..=64).contains(&group_size) {
            return Err(DecimalConverterError::GroupSizeOutOfBounds);
        }

        Ok(DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: self.endian_marker,
        })
    }
}

impl<'a, E> DecimalConverter<'a, i64, E> {
    pub fn unsigned(self) -> DecimalConverter<'a, u64, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, E> DecimalConverter<'a, u64, E> {
    pub fn signed(self) -> DecimalConverter<'a, i64, E> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O> DecimalConverter<'a, O, LittleEndian> {
    pub fn big_endian(self) -> DecimalConverter<'a, O, BigEndian> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O> DecimalConverter<'a, O, BigEndian> {
    pub fn little_endian(self) -> DecimalConverter<'a, O, LittleEndian> {
        DecimalConverter {
            input: self.input,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

pub trait FromBinary {
    type Output;

    fn from_binary(&self) -> Self::Output;
}

impl<'a> FromBinary for DecimalConverter<'a, SpacedBitGroup<i64>, LittleEndian> {
    type Output = Vec<i64>; // TODO: Should maybe be Iterator<i64>

    fn from_binary(&self) -> Self::Output {
        todo!()
    }
}

impl<'a> FromBinary for DecimalConverter<'a, SpacedBitGroup<u64>, LittleEndian> {
    type Output = Vec<u64>; // TODO: Should maybe be Iterator<i64>

    fn from_binary(&self) -> Self::Output {
        todo!()
    }
}

/*
impl<'a> FromBinary for DecimalConverter<'a, ExactBitGroup<u8>, LittleEndian> {
    type Output = Vec<u8>;

    fn from_binary(&self) -> Self::Output {
        todo!()
    }
}

impl<'a, E> FromBinary for DecimalConverter<'a, VariableBitGroup, E> {
    type Output = Vec<u64>;

    fn from_binary(&self) -> Self::Output {
        todo!()
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::{DecimalConverterExt, FromBinary};

    #[test]
    fn binary_from_decimal() {
        let d = "1010".decimal().from_binary();
    }
}
