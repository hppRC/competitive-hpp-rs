use num::traits::{int::PrimInt, Unsigned};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ModFact<T> {
    fact: Vec<T>,
    inv: Vec<T>,
    factinv: Vec<T>,
    modulo: T,
}

impl<T> ModFact<T>
where
    T: PrimInt + Unsigned,
{
    pub fn new(n: T, modulo: T) -> Self {
        let zero = T::zero();
        let one = T::one();
        let vec_size = n.to_usize().unwrap() + 1usize;

        let mut fact: Vec<T> = vec![zero; vec_size];
        fact[0] = one;
        for i in 1..vec_size {
            fact[i] = T::from(i).unwrap() * fact[i - 1] % modulo;
        }

        let mut inv: Vec<T> = vec![zero; vec_size];
        inv[0] = zero;
        inv[1] = one;
        for i in 2..vec_size {
            inv[i] = inv[modulo.to_usize().unwrap() % i] * (modulo - modulo / T::from(i).unwrap())
                % modulo;
        }

        let mut factinv: Vec<T> = vec![zero; vec_size];
        factinv[0] = one;
        for i in 1..vec_size {
            factinv[i] = factinv[i - 1] * inv[i] % modulo;
        }

        ModFact {
            fact,
            inv,
            factinv,
            modulo,
        }
    }

    pub fn perm(&self, n: T, r: T) -> T {
        let n = n.to_usize().unwrap();
        let r = r.to_usize().unwrap();
        if n < r {
            T::from(0).unwrap()
        } else {
            self.fact[n] * self.factinv[n - r] % self.modulo
        }
    }

    pub fn comb(&self, n: T, r: T) -> T {
        let n = n.to_usize().unwrap();
        let r = r.to_usize().unwrap();
        if n < r {
            T::from(0).unwrap()
        } else {
            (self.fact[n] * self.factinv[r] % self.modulo) * self.factinv[n - r] % self.modulo
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_u16_factinv()  {
        let _table = ModFact::new(10u16, 91u16);
    }
    #[test]
    fn test_u32_factinv()  {
        let _table = ModFact::new(10u32, 91u32);
    }
    #[test]
    fn test_u64_factinv()  {
        let _table = ModFact::new(10u64, 91u64);
    }
    #[test]
    fn test_usize_factinv()  {
        let _table = ModFact::new(10usize, 91usize);
    }
}
