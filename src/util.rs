use std::cmp::{max, min};

pub fn transform_u16_to_array_of_u8(x: u16) -> [u8; 2] {
    let b1: u8 = ((x >> 8) & 0xff) as u8;
    let b2: u8 = ((x >> 0) & 0xff) as u8;
    return [b1, b2]
}

pub fn transform_array_of_u8_to_u16(x: [u8; 2]) -> u16 {
    ((x[0] as u16) << 8) + ((x[1] as u16) << 0)
}

pub fn bound<T: Ord>(lower: T, upper: T, value: T) -> T {
    max(min(value, upper), lower)
}
