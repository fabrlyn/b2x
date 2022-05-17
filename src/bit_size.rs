pub trait BitSize {
    fn bit_size() -> usize;
}

impl BitSize for u8 {
    fn bit_size() -> usize {
        8
    }
}

impl BitSize for i8 {
    fn bit_size() -> usize {
        8
    }
}

impl BitSize for u16 {
    fn bit_size() -> usize {
        16
    }
}

impl BitSize for i16 {
    fn bit_size() -> usize {
        16
    }
}

impl BitSize for u32 {
    fn bit_size() -> usize {
        32
    }
}

impl BitSize for i32 {
    fn bit_size() -> usize {
        32
    }
}

impl BitSize for u64 {
    fn bit_size() -> usize {
        64
    }
}

impl BitSize for i64 {
    fn bit_size() -> usize {
        64
    }
}

impl BitSize for u128 {
    fn bit_size() -> usize {
        128
    }
}

impl BitSize for i128 {
    fn bit_size() -> usize {
        128
    }
}

pub trait AsymmetricSized {
    fn size() -> usize;    
}
