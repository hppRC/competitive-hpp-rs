use num::traits::{NumOps, PrimInt};
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
    fn inverse() -> Self;
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
    fn inverse() -> Self {
        todo!()
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

impl<T> Add<T> for ModInt<T>
where
    T: NumOps + PartialOrd + Copy,
{
    type Output = ModInt<T>;

    fn add(self, rhs: T) -> Self {
        ModInt {
            value: if self.value + rhs >= self.modulo {
                (self.value + rhs) % self.modulo
            } else {
                self.value + rhs
            },
            modulo: self.modulo,
        }
    }
}

macro_rules! impl_modint_add(($($ty:ty),*) => {
    $(
        impl<T> Add<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            type Output = ModInt<T>;

            fn add(self, rhs: ModInt<T>) -> ModInt<T> {
                ModInt {
                    value: if T::from(self).unwrap() + rhs.value >= rhs.modulo {
                        (T::from(self).unwrap() + rhs.value) % rhs.modulo
                    } else {
                        T::from(self).unwrap() + rhs.value
                    },
                    modulo: rhs.modulo,
                }
            }
        }
    )*
});

impl_modint_add!(i32, i64, u32, u64, isize, usize);

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

impl<T> Sub<T> for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    fn sub(self, rhs: T) -> Self {
        ModInt {
            value: if self.value < rhs {
                self.value + self.modulo - rhs
            } else {
                self.value - rhs
            },
            modulo: self.modulo,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modint_modint() {
        const MOD: usize = 7;
        let mi0 = ModInt::new(0, MOD);
        let mi1 = ModInt::new(1, MOD);
        let mi2 = ModInt::new(2, MOD);
        let mi4 = ModInt::new(4, MOD);
        let mi7 = ModInt::new(7, MOD);
        let mi11 = ModInt::new(11, MOD);

        assert_eq!(mi0 + mi7, ModInt::new(0, 7));
        assert_eq!(mi1 + mi2, ModInt::new(3, 7));
        assert_eq!(mi1 + mi11, ModInt::new(5, 7));
        assert_eq!(mi1 - mi4, ModInt::new(4, 7));
    }

    #[test]
    fn test_modint_other_type() {
        const MOD: usize = 7;
        let mi0 = ModInt::new(0, MOD);

        assert_eq!(mi0 + 6, ModInt::new(6, 7));
        assert_eq!(mi0 + 7, ModInt::new(0, 7));
        assert_eq!(7usize + mi0, ModInt::new(0, 7));
    }
}
