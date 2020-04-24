pub trait MathUtils {
    /// ## Example:
    /// ```
    /// use competitive_hpp::prelude::*;
    ///
    /// assert_eq!(16.log2_trunc(), 4);
    /// assert_eq!(10.log2_trunc(), 3);
    /// ```
    fn log2_trunc(self) -> Self;
}

macro_rules! impl_digit_utils(($($ty:ty),*) => {
    $(
        impl MathUtils for $ty {
            fn log2_trunc(self) -> Self {
                (self as f64).log2().trunc() as Self
            }
        }
    )*
});

impl_digit_utils!(u64, u32, i64, i32, usize, isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_math_test() -> () {
        assert_eq!(16u64.log2_trunc(), 4);
        assert_eq!(10u64.log2_trunc(), 3);
    }
    #[test]
    fn u32_math_test() -> () {
        assert_eq!(16u32.log2_trunc(), 4);
        assert_eq!(10u32.log2_trunc(), 3);
    }
    #[test]
    fn i64_math_test() -> () {
        assert_eq!(16i64.log2_trunc(), 4);
        assert_eq!(10i64.log2_trunc(), 3);
    }
    #[test]
    fn i32_math_test() -> () {
        assert_eq!(16i32.log2_trunc(), 4);
        assert_eq!(10i32.log2_trunc(), 3);
    }
    #[test]
    fn usize_math_test() -> () {
        assert_eq!(16usize.log2_trunc(), 4);
        assert_eq!(10usize.log2_trunc(), 3);
    }
    #[test]
    fn isize_math_test() -> () {
        assert_eq!(16isize.log2_trunc(), 4);
        assert_eq!(10isize.log2_trunc(), 3);
    }
}
