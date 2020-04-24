pub trait DigitUtils {
    fn digit(self) -> u32;
    fn leftmost_digit(self) -> Self;
    fn rightmost_digit(self) -> Self;
    fn nth_digit(self, n: u32) -> Self;
}

macro_rules! impl_digit_utils(($($ty:ty),*) => {
    $(
        impl DigitUtils for $ty {
            fn digit(self) -> u32 {
                (self as f64).log10().trunc() as u32 + 1
            }

            fn leftmost_digit(self) -> Self {
                (self / (10 as Self).pow(self.digit() - 1))
            }

            fn rightmost_digit(self) -> Self {
                (self % 10)
            }

            fn nth_digit(self, n:u32) -> Self {
                ((self / (10 as Self).pow(n - 1)) % 10)
            }
        }
    )*
});

impl_digit_utils!(u64, u32, i64, i32, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn u64_digit_test() -> () {
        assert_eq!(1u64.digit(), 1);
        assert_eq!(12u64.digit(), 2);

        assert_eq!(1u64.leftmost_digit(), 1);
        assert_eq!(200u64.leftmost_digit(), 2);

        assert_eq!(10u64.rightmost_digit(), 0);
        assert_eq!(202u64.rightmost_digit(), 2);

        assert_eq!(4321u64.nth_digit(1), 1);
        assert_eq!(4321u64.nth_digit(2), 2);
        assert_eq!(4321u64.nth_digit(3), 3);
    }
    #[test]
    fn u32_digit_test() -> () {
        assert_eq!(1u32.digit(), 1);
        assert_eq!(12u32.digit(), 2);

        assert_eq!(1u32.leftmost_digit(), 1);
        assert_eq!(200u32.leftmost_digit(), 2);

        assert_eq!(10u32.rightmost_digit(), 0);
        assert_eq!(202u32.rightmost_digit(), 2);

        assert_eq!(4321u32.nth_digit(1), 1);
        assert_eq!(4321u32.nth_digit(2), 2);
        assert_eq!(4321u32.nth_digit(3), 3);
    }
    #[test]
    fn i64_digit_test() -> () {
        assert_eq!(1i64.digit(), 1);
        assert_eq!(12i64.digit(), 2);

        assert_eq!(1i64.leftmost_digit(), 1);
        assert_eq!(200i64.leftmost_digit(), 2);

        assert_eq!(10i64.rightmost_digit(), 0);
        assert_eq!(202i64.rightmost_digit(), 2);

        assert_eq!(4321i64.nth_digit(1), 1);
        assert_eq!(4321i64.nth_digit(2), 2);
        assert_eq!(4321i64.nth_digit(3), 3);
    }
    #[test]
    fn i32_digit_test() -> () {
        assert_eq!(1i32.digit(), 1);
        assert_eq!(12i32.digit(), 2);

        assert_eq!(1i32.leftmost_digit(), 1);
        assert_eq!(200i32.leftmost_digit(), 2);

        assert_eq!(10i32.rightmost_digit(), 0);
        assert_eq!(202i32.rightmost_digit(), 2);

        assert_eq!(4321i32.nth_digit(1), 1);
        assert_eq!(4321i32.nth_digit(2), 2);
        assert_eq!(4321i32.nth_digit(3), 3);
    }
    #[test]
    fn usize_digit_test() -> () {
        assert_eq!(1usize.digit(), 1);
        assert_eq!(12usize.digit(), 2);

        assert_eq!(1usize.leftmost_digit(), 1);
        assert_eq!(200usize.leftmost_digit(), 2);

        assert_eq!(10usize.rightmost_digit(), 0);
        assert_eq!(202usize.rightmost_digit(), 2);

        assert_eq!(4321usize.nth_digit(1), 1);
        assert_eq!(4321usize.nth_digit(2), 2);
        assert_eq!(4321usize.nth_digit(3), 3);
    }
    #[test]
    fn isize_digit_test() -> () {
        assert_eq!(1isize.digit(), 1);
        assert_eq!(12isize.digit(), 2);

        assert_eq!(1isize.leftmost_digit(), 1);
        assert_eq!(200isize.leftmost_digit(), 2);

        assert_eq!(10isize.rightmost_digit(), 0);
        assert_eq!(202isize.rightmost_digit(), 2);

        assert_eq!(4321isize.nth_digit(1), 1);
        assert_eq!(4321isize.nth_digit(2), 2);
        assert_eq!(4321isize.nth_digit(3), 3);
    }
}
