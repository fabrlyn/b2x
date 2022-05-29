use crate::{bit_size::BitSize, from_binary::FromBinaryString};

pub type ToResult<T> = Result<Vec<T>, String>;

fn to_decimal<T: BitSize + FromBinaryString>(
    data: &str,
) -> ToResult<<T as FromBinaryString>::Output>
where
    T: BitSize + FromBinaryString,
{
    data.as_bytes()
        .chunks(T::bit_size())
        .flat_map(|s| String::from_utf8(s.to_vec()).map_err(|_| format!("{:?}", s)))
        .map(|s| T::from_binary_string(&s))
        .collect()
}

pub fn spaced_little_endian_to_decimal<T: BitSize + FromBinaryString>(
    data: &str,
) -> ToResult<<T as FromBinaryString>::Output> {
    data.split(' ')
        .map(|d| to_decimal::<T>(d))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().flatten().collect())
}

pub fn little_endian_to_decimal<T: BitSize + FromBinaryString>(
    data: &str,
) -> ToResult<<T as FromBinaryString>::Output> {
    to_decimal::<T>(data)
}

pub fn big_endian_to_decimal<T: BitSize + FromBinaryString>(
    data: &str,
) -> ToResult<<T as FromBinaryString>::Output> {
    let data: String = data.chars().rev().collect();
    to_decimal::<T>(&data)
}
