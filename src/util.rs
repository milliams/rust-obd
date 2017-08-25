use std::cmp::{max, min};

pub fn transform_u16_to_array_of_u8(x: u16) -> [u8; 2] {
    let b1: u8 = ((x >> 8) & 0xff) as u8;
    let b2: u8 = (x & 0xff) as u8;
    [b1, b2]
}

pub fn transform_array_of_u8_to_u16(x: [u8; 2]) -> u16 {
    ((x[0] as u16) << 8) + (x[1] as u16)
}

pub fn bound<T: Ord>(lower: T, upper: T, value: T) -> T {
    max(min(value, upper), lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_u16_to_array_of_u8() {
        assert_eq!(transform_u16_to_array_of_u8(0x3C81), [0x3C, 0x81]);
    }

    #[test]
    fn test_transform_array_of_u8_to_u16() {
        assert_eq!(transform_array_of_u8_to_u16([0x3C, 0x81]), 0x3C81);
    }

    #[test]
    fn test_bound() {
        assert_eq!(bound(-20, 500, 100), 100);
        assert_eq!(bound(-20, 500, 600), 500);
        assert_eq!(bound(-20, 500, -5), -5);
        assert_eq!(bound(-20, 500, -400), -20);
    }
}
