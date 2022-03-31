use std::marker::PhantomData;

/*
let _ = "01010101".decimal().signed().big_endian().group_size(4).from_binary();
let _ = "10010101001".float32().big_endian().convert();
let _ = "10010101001".float64().big_endian().convert();
let _ = "1001 1010".hex().signed().big_endian().group_size(8).convert();
let actual: u32 = 10f32.binary().signed().big_endian().group_size().convert();
*/

#[derive(Debug)]
pub struct DecimalConverter<'a, O, E> {
    input: &'a str,
    group_size: Option<u8>,
    output_marker: PhantomData<O>,
    endian_marker: PhantomData<E>,
}

pub struct BigEndian;
pub struct LittleEndian;

pub enum DecimalConverterError {
    GroupSizeOutOfBounds,
}

pub trait DecimalConverterExt<O, E> {
    fn decimal(&self) -> DecimalConverter<O, E>;
}

impl DecimalConverterExt<u64, LittleEndian> for &str {
    fn decimal(&self) -> DecimalConverter<u64, LittleEndian> {
        DecimalConverter::new(self)
    }
}

impl<'a> DecimalConverter<'a, u64, LittleEndian> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            group_size: None,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, E> DecimalConverter<'a, i64, E> {
    pub fn unsigned(self) -> DecimalConverter<'a, u64, E> {
        DecimalConverter {
            input: self.input,
            group_size: self.group_size,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, E> DecimalConverter<'a, u64, E> {
    pub fn signed(self) -> DecimalConverter<'a, i64, E> {
        DecimalConverter {
            input: self.input,
            group_size: self.group_size,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O> DecimalConverter<'a, O, LittleEndian> {
    pub fn big_endian(self) -> DecimalConverter<'a, O, BigEndian> {
        DecimalConverter {
            input: self.input,
            group_size: self.group_size,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O> DecimalConverter<'a, O, BigEndian> {
    pub fn little_endian(self) -> DecimalConverter<'a, O, LittleEndian> {
        DecimalConverter {
            input: self.input,
            group_size: self.group_size,
            output_marker: PhantomData,
            endian_marker: PhantomData,
        }
    }
}

impl<'a, O, E> DecimalConverter<'a, O, E> {
    pub fn group_size(self, group_size: u8) -> Result<Self, DecimalConverterError> {
        if !(2..=64).contains(&group_size) {
            return Err(DecimalConverterError::GroupSizeOutOfBounds);
        }

        Ok(Self {
            group_size: Some(group_size),
            ..self
        })
    }

    pub fn auto_group_size(self) -> Self {
        Self {
            group_size: None,
            ..self
        }
    }
}

pub trait FromBinary {
    type Output;

    fn from_binary(&self) -> Self::Output;
}

impl<'a> FromBinary for DecimalConverter<'a, u64, LittleEndian> {
    type Output = Vec<u64>;

    fn from_binary(&self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{DecimalConverterExt, FromBinary};

    #[test]
    fn binary_from_decimal() {
        let d = "1010".decimal().from_binary();
    }
}
