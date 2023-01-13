pub fn u64_be(array: Vec<u8>) -> u64 {
    assert!(array.len() == 8);
    ((array[0] as u64) << 56)
        + ((array[1] as u64) << 48)
        + ((array[2] as u64) << 40)
        + ((array[3] as u64) << 32)
        + ((array[4] as u64) << 24)
        + ((array[5] as u64) << 16)
        + ((array[6] as u64) << 8)
        + ((array[7] as u64) << 0)
}

pub fn p64_be(value: u64) -> [u8; 8] {
    return [
        (value >> 56) as u8,
        (value >> 48) as u8,
        (value >> 40) as u8,
        (value >> 32) as u8,
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        (value >> 0) as u8,
    ];
}
