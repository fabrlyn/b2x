use std::marker::PhantomData;

use crate::bit_size::BitSize;

use super::common::{DecimalConverter, DecimalConverterError};

pub struct VariableBitGroup(u8);

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

