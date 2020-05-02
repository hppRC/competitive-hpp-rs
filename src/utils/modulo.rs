use num::integer::{ExtendedGcd, Integer};
use num::traits::{PrimInt, Signed};

pub fn mod_inv<A: Copy + Integer + Signed>(a: A, modulo: A) -> A {
    let ExtendedGcd { x, .. } = a.extended_gcd(&modulo);
    (modulo + x) % modulo
}

pub fn mod_pow<A: Copy + Integer + Signed + PrimInt>(x: A, n: A, m: A) -> A {
    let mut res = A::one();
    let mut x = x;
    let mut n = n;
    while n > A::zero() {
        if n & A::one() == A::one() {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n = n >> 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! impl_modulo_test(($ty:ty) => {
        assert_eq!(1, mod_inv(1 as $ty, MOD));
        assert_eq!(4, mod_inv(2 as $ty, MOD));
        assert_eq!(5, mod_inv(3 as $ty, MOD));
        assert_eq!(2, mod_pow(2 as $ty, 4 as $ty, MOD));
        assert_eq!(1, mod_pow(3 as $ty, 6 as $ty, MOD));
        assert_eq!(2, mod_pow(5 as $ty, 100 as $ty, MOD));
        assert_eq!(4, mod_pow(11 as $ty, 1000 as $ty, MOD));
    });

    #[test]
    fn test_isize_mod() -> () {
        const MOD: isize = 7;
        impl_modulo_test!(isize);
    }

    #[test]
    fn test_i64_mod() -> () {
        const MOD: i64 = 7;
        impl_modulo_test!(i64);
    }
    #[test]
    fn test_i32_mod() -> () {
        const MOD: i32 = 7;
        impl_modulo_test!(i32);
    }
}
