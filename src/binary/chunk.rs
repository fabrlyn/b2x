// Big Endian Unsigned Decimal
pub fn as_big_endian_u64(chunk: &[u8]) -> u64 {
    chunk
        .iter()
        .map(|v| *v as u64)
        .enumerate()
        .map(|(index, value)| value << (63 - index))
        .sum()
}

pub fn as_big_endian_u32(chunk: &[u8]) -> u32 {
    chunk
        .iter()
        .map(|v| *v as u32)
        .enumerate()
        .map(|(index, value)| value << (31 - index))
        .sum()
}

pub fn as_big_endian_u16(chunk: &[u8]) -> u16 {
    chunk
        .iter()
        .map(|v| *v as u16)
        .enumerate()
        .map(|(index, value)| value << (15 - index))
        .sum()
}

pub fn as_big_endian_u8(chunk: &[u8]) -> u8 {
    chunk
        .iter()
        .enumerate()
        .map(|(index, value)| value << (7 - index))
        .sum()
}

// Big Endian Signed Deciaml
pub fn as_big_endian_i8(chunk: &[u8]) -> i8 {
    let signed = *chunk.first().unwrap();
    let tail = &chunk[1..];
    if signed == 1 {
        let mut inverted = tail
            .iter()
            .map(|v| if *v == 0 { 1 } else { 0 })
            .collect::<Vec<_>>();
        let mut new_inverted = vec![0];
        new_inverted.append(&mut inverted);

        let number = as_big_endian_u8(&new_inverted) as i8;
        return (number * -1) - 1;
    }

    as_big_endian_u8(chunk) as i8
}

// Little Endian Unsigned Decimal
pub fn as_little_endian_u8(chunk: &[u8]) -> u8 {
    chunk
        .iter()
        .enumerate()
        .map(|(index, value)| value << index)
        .sum()
}

pub fn as_little_endian_u16(chunk: &[u8]) -> u16 {
    chunk
        .iter()
        .map(|v| *v as u16)
        .enumerate()
        .map(|(index, value)| value << index)
        .sum()
}

pub fn as_little_endian_u32(chunk: &[u8]) -> u32 {
    chunk
        .iter()
        .map(|v| *v as u32)
        .enumerate()
        .map(|(index, value)| value << index)
        .sum()
}

pub fn as_little_endian_u64(chunk: &[u8]) -> u64 {
    chunk
        .iter()
        .map(|v| *v as u64)
        .enumerate()
        .map(|(index, value)| value << index)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn as_big_endian_u8() {
        let input = &[0, 1, 1, 0, 0, 1, 0, 0];
        let actual = super::as_big_endian_u8(input);
        assert_eq!(100, actual);
    }

    #[test]
    fn as_big_endian_u16() {
        let input = [[1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0]]
            .iter()
            .flatten()
            .map(|v| *v)
            .collect::<Vec<u8>>();

        let actual = super::as_big_endian_u16(&input);
        assert_eq!(32772, actual);
    }

    #[test]
    fn as_big_endian_u32() {
        let input = [
            [1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]
        .iter()
        .flatten()
        .map(|v| *v)
        .collect::<Vec<u8>>();

        let actual = super::as_big_endian_u32(&input);
        assert_eq!(2_147_483_648, actual);
    }

    #[test]
    fn as_big_endian_u64() {
        let input = [
            [1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]
        .iter()
        .flatten()
        .map(|v| *v)
        .collect::<Vec<u8>>();

        let actual = super::as_big_endian_u64(&input);
        assert_eq!(0b1 << 63, actual);
    }

    #[test]
    fn as_little_endian_u8() {
        let input = &[0, 1, 1, 0, 0, 1, 0, 0];
        let actual = super::as_little_endian_u8(input);
        assert_eq!(38, actual);
    }

    #[test]
    fn as_little_endian_u16() {
        let input = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 1]]
            .iter()
            .flatten()
            .map(|v| *v)
            .collect::<Vec<u8>>();

        let actual = super::as_little_endian_u16(&input);
        assert_eq!(0b1 << 15, actual);
    }

    #[test]
    fn as_little_endian_u32() {
        let input = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1],
        ]
        .iter()
        .flatten()
        .map(|v| *v)
        .collect::<Vec<u8>>();

        let actual = super::as_little_endian_u32(&input);
        assert_eq!(0b1 << 31, actual);
    }

    #[test]
    fn as_little_endian_u64() {
        let input = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1],
        ]
        .iter()
        .flatten()
        .map(|v| *v)
        .collect::<Vec<u8>>();

        let actual = super::as_little_endian_u64(&input);
        assert_eq!(0b1 << 63, actual);
    }

    #[test]
    fn as_big_endian_i8() {
        let input = &[1, 0, 0, 0, 0, 0, 0, 0];
        let actual = super::as_big_endian_i8(input);
        assert_eq!(-128, actual);

        let input = &[1, 0, 0, 0, 0, 0, 0, 1];
        let actual = super::as_big_endian_i8(input);
        assert_eq!(-127, actual);

        let input = &[0, 0, 0, 0, 0, 0, 0, 1];
        let actual = super::as_big_endian_i8(input);
        assert_eq!(1, actual);

        let input = &[0, 1, 1, 1, 1, 1, 1, 1];
        let actual = super::as_big_endian_i8(input);
        assert_eq!(127, actual);
    }
}
