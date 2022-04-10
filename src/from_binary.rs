pub trait FromBinaryString {
    type Output;

    fn from_binary_string(input: &str) -> Result<Self::Output, String>;
}

impl FromBinaryString for u8 {
    type Output = u8;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u8::from_str_radix(input, 2).map_err(|_| input.to_string())
    }
}

impl FromBinaryString for i8 {
    type Output = i8;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u8::from_binary_string(input).map(|i| i as i8)
    }
}

impl FromBinaryString for u16 {
    type Output = u16;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u16::from_str_radix(input, 2).map_err(|_| input.to_string())
    }
}

impl FromBinaryString for i16 {
    type Output = i16;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u16::from_binary_string(input).map(|i| i as i16)
    }
}

impl FromBinaryString for u32 {
    type Output = u32;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u32::from_str_radix(input, 2).map_err(|_| input.to_string())
    }
}

impl FromBinaryString for i32 {
    type Output = i32;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u32::from_binary_string(input).map(|i| i as i32)
    }
}

impl FromBinaryString for u64 {
    type Output = u64;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u64::from_str_radix(input, 2).map_err(|_| input.to_string())
    }
}

impl FromBinaryString for i64 {
    type Output = i64;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u64::from_binary_string(input).map(|i| i as i64)
    }
}

impl FromBinaryString for u128 {
    type Output = u128;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u128::from_str_radix(input, 2).map_err(|_| input.to_string())
    }
}

impl FromBinaryString for i128 {
    type Output = i128;

    fn from_binary_string(input: &str) -> Result<Self::Output, String> {
        u128::from_binary_string(input).map(|i| i as i128)
    }
}
