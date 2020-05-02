pub trait DigitUtils {
    fn digit(self) -> u32;
    fn leftmost_digit(self) -> Self;
    fn rightmost_digit(self) -> Self;
    fn nth_digit(self, n: u32) -> Self;
    fn digit_sum(self) -> Self;
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

            fn digit_sum(self) -> Self {
                (0..self.digit()).fold((0, self),|(sum, x), _| (sum + x%10, x/10)).0
            }
        }
    )*
});

impl_digit_utils!(u64, u32, i64, i32, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_digit_utils_test(($ty:ty) => {
        assert_eq!((1 as $ty).digit(), 1);
        assert_eq!((12 as $ty).digit(), 2);
        assert_eq!((1 as $ty).leftmost_digit(), 1);
        assert_eq!((200 as $ty).leftmost_digit(), 2);

        assert_eq!((10 as $ty).rightmost_digit(), 0);
        assert_eq!((202 as $ty).rightmost_digit(), 2);

        assert_eq!((4321 as $ty).nth_digit(1), 1);
        assert_eq!((4321 as $ty).nth_digit(2), 2);
        assert_eq!((4321 as $ty).nth_digit(3), 3);

        assert_eq!((1234 as $ty).digit_sum(), 10);
    });

    #[test]
    fn u64_digit_test() -> () {
        impl_digit_utils_test!(u64);
    }

    #[test]
    fn u32_digit_test() -> () {
        impl_digit_utils_test!(u32);
    }
    #[test]
    fn i64_digit_test() -> () {
        impl_digit_utils_test!(i64);
    }
    #[test]
    fn i32_digit_test() -> () {
        impl_digit_utils_test!(i32);
    }
    #[test]
    fn usize_digit_test() -> () {
        impl_digit_utils_test!(usize);
    }
    #[test]
    fn isize_digit_test() -> () {
        impl_digit_utils_test!(isize);
    }
}
