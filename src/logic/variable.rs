use std::marker::PhantomData;

use super::common::{DecimalConverter, DecimalConverterError, Symmetric};

pub struct VariableBitGroup(u8);

impl<'a, O, E, S, F> DecimalConverter<'a, O, E, S, F> {
    pub fn bit_group_size(
        self,
        group_size: u8,
    ) -> Result<DecimalConverter<'a, Symmetric<u8>, E, S, F>, DecimalConverterError> {
        if !(2..=64).contains(&group_size) {
            return Err(DecimalConverterError::GroupSizeOutOfBounds);
        }

        Ok(DecimalConverter {
            input: self.input,
            bit_alignment: Symmetric::<u8>(PhantomData),
            endian_marker: self.endian_marker,
            format_marker: self.format_marker,
            signifier: self.signifier,
        })
    }
}
