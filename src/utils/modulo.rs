use num::integer::{ExtendedGcd, Integer};
use num::traits::Signed;

pub fn mod_inv<A: Copy + Integer + Signed>(a: A, modulo: A) -> A {
    let ExtendedGcd { x, .. } = a.extended_gcd(&modulo);
    (modulo + x) % modulo
}

pub fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
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

    #[test]
    fn test_isize_mod_inv() -> () {
        const MOD: isize = 7;
        assert_eq!(1, mod_inv(1, MOD));
        assert_eq!(4, mod_inv(2, MOD));
        assert_eq!(5, mod_inv(3, MOD));
    }
}
