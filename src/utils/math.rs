pub trait MathUtils {
    /// ## Example:
    /// ```
    /// use competitive_hpp::prelude::*;
    ///
    /// assert_eq!(16.log2_trunc(), 4);
    /// assert_eq!(10.log2_trunc(), 3);
    /// ```
    fn log2_trunc(self) -> Self;
    fn sqrt_floor(self) -> Self;
    fn sqrt_ceil(self) -> Self;
}

macro_rules! impl_digit_utils(($($ty:ty),*) => {
    $(
        impl MathUtils for $ty {
            fn log2_trunc(self) -> Self {
                (self as f64).log2().trunc() as Self
            }

            fn sqrt_floor(self) -> Self {
                (self as f64).sqrt() as Self
            }

            fn sqrt_ceil(self) -> Self {
                let tmp = (self as f64).sqrt() as Self;
                if tmp * tmp == self {
                    tmp
                } else {
                    tmp + 1
                }
            }
        }
    )*
});

impl_digit_utils!(u64, u32, i64, i32, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! impl_math_tests (($ty:ty) => {
        assert_eq!((16 as $ty).log2_trunc(), 4);
        assert_eq!((10 as $ty).log2_trunc(), 3);

        assert_eq!((9 as $ty).sqrt_floor(), 3);
        assert_eq!((10 as $ty).sqrt_floor(), 3);
        assert_eq!((15 as $ty).sqrt_floor(), 3);

        assert_eq!((9 as $ty).sqrt_ceil(), 3);
        assert_eq!((10 as $ty).sqrt_ceil(), 4);
        assert_eq!((10 as $ty).sqrt_ceil(), 4);

    });

    #[test]
    fn u64_math_test() {
        impl_math_tests!(u64);
    }
    #[test]
    fn u32_math_test() {
        impl_math_tests!(u32);
    }
    #[test]
    fn i64_math_test() {
        impl_math_tests!(i64);
    }
    #[test]
    fn i32_math_test() {
        impl_math_tests!(i32);
    }
    #[test]
    fn usize_math_test() {
        impl_math_tests!(usize);
    }
    #[test]
    fn isize_math_test() {
        impl_math_tests!(isize);
    }
}
