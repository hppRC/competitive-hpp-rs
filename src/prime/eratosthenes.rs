use num::traits::PrimInt;
use std::collections::HashMap;
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Eratosthenes {
    primes: Vec<usize>,
    flags: Vec<usize>,
}

/// # Eratosthenes
///
/// Example:
/// ```
/// use competitive_hpp::prelude::*;
/// let eratosthenes = Eratosthenes::new(100);
///
/// assert!(!eratosthenes.is_prime(-1));
/// assert!(!eratosthenes.is_prime(0));
/// assert!(eratosthenes.is_prime(2));
/// assert!(!eratosthenes.is_prime(10));
/// assert!(eratosthenes.is_prime(11));
///
/// let mut map = HashMap::new();
/// map.insert(2, 1);
/// map.insert(5, 2);
/// assert_eq!(map, eratosthenes.factorization(50));
/// ```
#[allow(clippy::many_single_char_names)]
impl Eratosthenes {
    pub fn new<T>(n: T) -> Self
    where
        T: PrimInt,
    {
        let n = n.to_usize().unwrap();
        let bits = 32;
        let flags_num = n / bits + 1;

        let defaults: Vec<usize> = vec![0x5D75D75D, 0xD75D75D7, 0x75D75D75];

        let (mut i, mut f, mut j);
        let max = ((n as f64).sqrt() as usize) + 1;

        let mut flags: Vec<usize> = (0..flags_num).map(|i| defaults[i % 3]).collect();
        flags[flags_num - 1] &= (1 << (n % bits + 1)) - 1;

        i = 5;
        f = 4;
        while i <= max {
            let t = (flags[i / bits] >> (i % bits)) & 1 == 1;
            if !t {
                j = i * (i | 1);
                while j <= n {
                    flags[j / bits] |= 1 << (j % bits);
                    j += i * 2;
                }
            }
            f = 6 - f;
            i += f;
        }

        flags[0] &= !0b1100;
        flags[0] |= 0b11;

        let mut primes = vec![];
        for p in 2..n + 1 {
            if (flags[p / bits] >> (p % bits)) & 1 == 0 {
                primes.push(p);
            }
        }

        Eratosthenes { primes, flags }
    }

    pub fn is_prime<T>(&self, m: T) -> bool
    where
        T: PrimInt,
    {
        match m.to_usize() {
            Some(um) => self.flags[um / 32] >> (um % 32) & 1 == 0,
            None => false,
        }
    }

    pub fn factorization<T>(&self, n: T) -> HashMap<usize, usize>
    where
        T: PrimInt,
    {
        let mut n = n.to_usize().unwrap();
        let mut factors: HashMap<usize, usize> = HashMap::new();
        for &p in &self.primes {
            while n % p == 0 {
                n /= p;
                factors
                    .entry(p)
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
            }
            if p > n {
                break;
            }
        }
        factors
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eratosthenes() {
        let eratosthenes = Eratosthenes::new(100);

        assert!(!eratosthenes.is_prime(-1));
        assert!(!eratosthenes.is_prime(0));
        assert!(!eratosthenes.is_prime(1));
        assert!(eratosthenes.is_prime(2));
        assert!(eratosthenes.is_prime(3));
        assert!(!eratosthenes.is_prime(4));
        assert!(eratosthenes.is_prime(5));
        assert!(!eratosthenes.is_prime(6));
        assert!(eratosthenes.is_prime(7));
        assert!(!eratosthenes.is_prime(8));
        assert!(!eratosthenes.is_prime(9));
        assert!(!eratosthenes.is_prime(10));
        assert!(eratosthenes.is_prime(11));

        let mut map = HashMap::new();
        map.insert(2, 1);
        map.insert(5, 2);

        assert_eq!(map, eratosthenes.factorization(50));
        assert_ne!(map, eratosthenes.factorization(100));
    }
}
