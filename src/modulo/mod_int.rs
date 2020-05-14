use num::traits::PrimInt;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInt<T> {
    value: T,
    modulo: T,
}

trait ModIntTrait<T>
where
    T: PrimInt,
{
    fn new(n: T, modulo: T) -> Self;
}

impl<T> ModIntTrait<T> for ModInt<T>
where
    T: PrimInt,
{
    fn new(n: T, modulo: T) -> Self {
        ModInt {
            value: n % modulo,
            modulo,
        }
    }
}

impl<T> fmt::Display for ModInt<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Add for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    fn add(self, other: ModInt<T>) -> Self {
        ModInt {
            value: if self.value + other.value >= self.modulo {
                (self.value + other.value) % self.modulo
            } else {
                self.value + other.value
            },
            modulo: self.modulo,
        }
    }
}

impl<T> Sub for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    fn sub(self, other: ModInt<T>) -> Self {
        ModInt {
            value: if self.value < other.value {
                self.value + self.modulo - other.value
            } else {
                self.value - other.value
            },
            modulo: self.modulo,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        const MOD: usize = 7;
        let mi0 = ModInt::new(0, MOD);
        let mi1 = ModInt::new(1, MOD);
        let mi2 = ModInt::new(2, MOD);
        let mi4 = ModInt::new(4, MOD);
        let mi5 = ModInt::new(5, MOD);
        let mi6 = ModInt::new(6, MOD);
        let mi7 = ModInt::new(7, MOD);
        let mi11 = ModInt::new(11, MOD);

        assert_eq!(mi0 + mi7, ModInt::new(0, 7));
        assert_eq!(mi1 + mi2, ModInt::new(3, 7));
        assert_eq!(mi1 + mi11, ModInt::new(5, 7));
        assert_eq!(mi1 - mi4, ModInt::new(4, 7));
    }
}
