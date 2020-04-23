pub trait U64Utils {
    fn digit(self) -> u32;
    fn rightmost_digit(self) -> u8;
    fn leftmost_digit(self) -> u8;
}

impl U64Utils for u64 {
    fn digit(self: u64) -> u32 {
        (self as f64).log10().trunc() as u32 + 1
    }

    fn leftmost_digit(self: u64) -> u8 {
        let digit = self.digit() - 1;
        (self / 10u64.pow(digit)) as u8
    }

    fn rightmost_digit(self: u64) -> u8 {
        (self % 10) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn u64_utils_test() -> () {
        assert_eq!(1u64.digit(), 1);
        assert_eq!(12u64.digit(), 2);
        assert_eq!(123u64.digit(), 3);
        assert_eq!(1234u64.digit(), 4);

        assert_eq!(1u64.leftmost_digit(), 1);
        assert_eq!(1u64.leftmost_digit(), 1);

        assert_eq!(1u64.rightmost_digit(), 1);
        assert_eq!(1u64.rightmost_digit(), 1);
    }
}
