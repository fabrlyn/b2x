pub trait DecimalExt {
    type Output;

    fn decimal(&self) -> Self::Output;
}

impl DecimalExt for f32 {
    type Output = u32;

    fn decimal(&self) -> Self::Output {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::decimal::DecimalExt;

    #[test]
    fn decimal_ext_f32_to_u32() {
        let actual: u32 = 10f32.decimal();
        let expected = 10u32;

        assert_eq!(expected, actual);
    }
}
