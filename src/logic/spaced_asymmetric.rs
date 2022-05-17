use std::marker::PhantomData;

use super::common::DecimalConverterError;

pub struct SpacedAsymmetricBitGroup<T> {
    bit_group_marker: PhantomData<T>,
    bit_group_size: u8,
}

impl SpacedAsymmetricBitGroup<u8> {
    pub fn new(bit_group_size: u8) -> Result<Self, DecimalConverterError> {
        if !(1..=7).contains(&bit_group_size) {
            return Err(DecimalConverterError::ParseError(
                "bit_group_size can't be larger than 7 or smaller than 1".to_string(),
            ));
        }

        Ok(Self {
            bit_group_marker: PhantomData,
            bit_group_size,
        })
    }
}
