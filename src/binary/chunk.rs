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
}
