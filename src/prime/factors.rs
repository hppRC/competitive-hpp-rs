use num::traits::PrimInt;
use std::collections::HashMap;

pub trait Factor: PrimInt {
    //! ## Example:
    //! ```
    //! use competitive_hpp::prelude::*;
    //!
    //! assert_eq!(2, 10u64.first_factor());
    //! assert_eq!(vec![2, 5, 5], 50u64.factors());
    //! assert_eq!(vec![2, 5], 50u64.factors_uniq());
    //! assert!(7u64.is_prime());
    //!
    //! let mut map: HashMap<u64, u64> = HashMap::new();
    //! map.insert(2, 1);
    //! map.insert(5, 2);
    //! assert_eq!(map, 50u64.factors_map());
    //! ```
    fn factors(&self) -> Vec<Self>;
    fn factors_uniq(&self) -> Vec<Self>;
    fn factors_map(&self) -> HashMap<Self, Self>;
    fn is_prime(&self) -> bool;
    fn first_factor(&self) -> Self;
}

macro_rules! impl_prime_factors(($($ty:ty),*) => {
    $(
        impl Factor for $ty
        {
            /// Find all prime factors of a number
            /// Does not use a `PrimeSet`, but simply counts upwards
            fn factors(&self) -> Vec<Self> {
                if self <= &1 {
                    return Vec::new();
                };
                let mut lst: Vec<Self> = Vec::new();
                let mut curn = *self;
                loop {
                    let m = curn.first_factor();
                    lst.push(m);
                    if m == curn {
                        break;
                    }
                    curn /= m
                }
                lst
            }
            /// Find all unique prime factors of a number
            fn factors_uniq(&self) -> Vec<Self> {
                if self <= &1 {
                    return Vec::new();
                };
                let mut lst: Vec<Self> = Vec::new();
                let mut curn = *self;

                loop {
                    let m = curn.first_factor();
                    lst.push(m);
                    if curn == m {
                        break;
                    }
                    while curn % m == 0 {
                        curn /= m;
                    }
                    if curn == 1 {
                        break;
                    }
                }
                lst
            }

            fn factors_map(&self) -> HashMap<Self, Self> {
                let mut facs: HashMap<Self, Self> = HashMap::new();
                let mut n = *self;
                let mut a = 2;

                while n >= a * a {
                    if n % a == 0 {
                        n /= a;
                        *facs.entry(a).or_insert(0) += 1;
                    } else {
                        a += 1;
                    }
                }
                *facs.entry(n).or_insert(0) += 1;
                facs
            }
            /// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
            fn is_prime(&self) -> bool {
                if self <= &1 {
                    return false;
                }
                self.first_factor() == *self
            }

            fn first_factor(&self) -> Self {
                if *self % 2 == 0 {
                    return 2;
                };
                // for n in (3..).step_by(2).take_while(|m| m*m <= x) {
                for n in (1..)
                    .map(|m| 2 * m + 1)
                    .take_while(|m| m * m <= *self)
                {
                    if *self % n == 0 {
                        return n;
                    };
                }
                // No factor found. It must be prime.
                *self
            }
        }
    )*
});

impl_prime_factors!(u64, u32, u128, usize);


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! impl_factors_tests {
        ($ty:ty) => {
            assert_eq!(2, (10 as $ty).first_factor());
            assert_eq!(vec![2, 5, 5], (50 as $ty).factors());
            assert_eq!(vec![2, 5], (50 as $ty).factors_uniq());
            assert!((7 as $ty).is_prime());

            let mut map: HashMap<$ty, $ty> = HashMap::new();
            map.insert(2, 1);
            map.insert(5, 2);
            assert_eq!(map, (50 as $ty).factors_map());
        };
    }

    #[test]
    fn u64_factors_test() -> () {
        impl_factors_tests!(u64);
    }
    #[test]
    fn u32_factors_test() -> () {
        impl_factors_tests!(u32);
    }
    #[test]
    fn usize_factors_test() -> () {
        impl_factors_tests!(usize);
    }
    #[test]
    fn u8_factors_test() -> () {
        impl_factors_tests!(u128);
    }
}
