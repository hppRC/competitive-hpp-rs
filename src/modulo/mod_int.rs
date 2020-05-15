use num::integer::*;
use num::traits::{NumOps, PrimInt, ToPrimitive};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// ## Example
/// ```
/// use competitive_hpp::prelude::*;
/// const MOD: usize = 7;
///
/// let mi0 = ModInt::new_with(0, MOD);
/// let mi1 = ModInt::new_with(1, MOD);
/// let mi2 = ModInt::new_with(2, MOD);
/// let mi11 = ModInt::new_with(11, MOD);
/// assert_eq!(4, ModInt::new(4));
/// assert_eq!(mi0 + mi11, ModInt::new_with(4, 7));
/// assert_eq!(mi1 + mi2, ModInt::new_with(3, 7));
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInt<T> {
    pub value: T,
    pub modulo: T,
}

pub trait ModIntTrait<T> {
    fn new(n: T) -> Self;
    fn new_with(n: T, modulo: T) -> Self;
    fn inverse(&self) -> Self;
    fn static_inverse_with(n: T, modulo: T) -> T;
}

impl<T> ModIntTrait<T> for ModInt<T>
where
    T: PrimInt,
{
    fn new(n: T) -> Self {
        Self::new_with(n, T::from(1000000007).unwrap())
    }

    fn new_with(n: T, modulo: T) -> Self {
        ModInt {
            value: n % modulo,
            modulo,
        }
    }

    #[inline]
    fn inverse(&self) -> Self {
        let value = Self::static_inverse_with(self.value, self.modulo);
        ModInt {
            value,
            modulo: self.modulo,
        }
    }

    #[inline]
    fn static_inverse_with(n: T, modulo: T) -> T {
        let ExtendedGcd { x, .. } = n.to_i64().unwrap().extended_gcd(&modulo.to_i64().unwrap());

        T::from(if x < 0 {
            x + modulo.to_i64().unwrap()
        } else {
            x
        })
        .unwrap()
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

    #[inline]
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

    #[inline]
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

            #[inline]
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

impl_modint_add!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T> Sub for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    #[inline]
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

    #[inline]
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

macro_rules! impl_modint_sub(($($ty:ty),*) => {
    $(
        impl<T> Sub<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            type Output = ModInt<T>;

            #[inline]
            fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
                ModInt {
                    value: if T::from(self).unwrap() < rhs.value {
                        T::from(self).unwrap() + rhs.modulo - rhs.value
                    } else {
                        (T::from(self).unwrap() - rhs.value) % rhs.modulo
                    },
                    modulo: rhs.modulo,
                }
            }
        }
    )*
});

impl_modint_sub!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T> Mul for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    #[inline]
    fn mul(self, other: ModInt<T>) -> Self {
        ModInt {
            value: (self.value * other.value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl<T> Mul<T> for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        ModInt {
            value: (self.value * rhs) % self.modulo,
            modulo: self.modulo,
        }
    }
}

macro_rules! impl_modint_mul(($($ty:ty),*) => {
    $(
        impl<T> Mul<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            type Output = ModInt<T>;

            #[inline]
            fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
                ModInt {
                    value: (T::from(self).unwrap() * rhs.value) % rhs.modulo,
                    modulo: rhs.modulo,
                }
            }
        }
    )*
});

impl_modint_mul!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T> Div for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    #[inline]
    fn div(self, other: ModInt<T>) -> Self {
        ModInt {
            value: (self.value * other.inverse().value) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl<T> Div<T> for ModInt<T>
where
    T: PrimInt,
{
    type Output = ModInt<T>;

    #[inline]
    fn div(self, rhs: T) -> Self {
        let inv = Self::static_inverse_with(rhs, self.modulo);
        ModInt {
            value: (self.value * inv) % self.modulo,
            modulo: self.modulo,
        }
    }
}

macro_rules! impl_modint_div(($($ty:ty),*) => {
    $(
        impl<T> Div<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            type Output = ModInt<T>;

            #[inline]
            fn div(self, rhs: ModInt<T>) -> ModInt<T> {
                let inv = ModInt::static_inverse_with(rhs.value, rhs.modulo);
                ModInt {
                    value: (T::from(self).unwrap() * inv) % rhs.modulo,
                    modulo: rhs.modulo,
                }
            }
        }
    )*
});

impl_modint_div!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T> PartialEq<T> for ModInt<T>
where
    T: PrimInt,
{
    fn eq(&self, other: &T) -> bool {
        self.value == other.clone()
    }
}

macro_rules! impl_modint_partial_eq(($($ty:ty),*) => {
    $(
        impl<T> PartialEq<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            #[inline]
            fn eq(&self, other: &ModInt<T>) -> bool {
                T::from(self.clone()).unwrap() == other.value.clone()
            }
        }
    )*
});

impl_modint_partial_eq!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T> PartialOrd<T> for ModInt<T>
where
    T: PrimInt,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.value.cmp(other))
    }
}

macro_rules! impl_modint_partial_ord(($($ty:ty),*) => {
    $(
        impl<T> PartialOrd<ModInt<T>> for $ty
        where
            T: PrimInt,
        {
            #[inline]
            fn partial_cmp(&self, other: &ModInt<T>) -> Option<Ordering> {
                Some(T::from(self.clone()).unwrap().cmp(&other.value))
            }
        }
    )*
});

impl_modint_partial_ord!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

macro_rules! impl_modint_to_primitive(($(($ty:ty, $method:ident)),*) => {
    $(
        #[inline]
        fn $method(&self) -> Option<$ty> {
            self.value.$method()
        }
    )*
});

impl<T> ToPrimitive for ModInt<T>
where
    T: PrimInt,
{
    impl_modint_to_primitive!(
        (i8, to_i8),
        (i16, to_i16),
        (i32, to_i32),
        (i64, to_i64),
        (u8, to_u8),
        (u16, to_u16),
        (u32, to_u32),
        (u64, to_u64),
        (isize, to_isize),
        (usize, to_usize)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modint_modint() {
        const MOD: usize = 7;
        let mi0 = ModInt::new_with(0, MOD);
        let mi1 = ModInt::new_with(1, MOD);
        let mi2 = ModInt::new_with(2, MOD);
        let mi4 = ModInt::new_with(4, MOD);
        let mi7 = ModInt::new_with(7, MOD);
        let mi11 = ModInt::new_with(11, MOD);

        assert_eq!(mi0 + mi7, ModInt::new_with(0, 7));
        assert_eq!(mi1 + mi2, ModInt::new_with(3, 7));
        assert_eq!(mi1 + mi11, ModInt::new_with(5, 7));
        assert_eq!(mi1 - mi4, ModInt::new_with(4, 7));
    }

    #[test]
    fn test_modint_other_type() {
        const MOD: usize = 7;
        let mi0 = ModInt::new_with(0, MOD);

        assert_eq!(mi0 + 6, ModInt::new_with(6, MOD));
        assert_eq!(mi0 + 7, ModInt::new_with(0, MOD));
        assert_eq!(7usize + mi0, ModInt::new_with(0, MOD));
        assert_eq!(15usize + mi0, ModInt::new_with(1, MOD));
        assert_eq!(mi0 - 4, ModInt::new_with(3, MOD));
        assert_eq!(mi0 - 0, ModInt::new_with(0, MOD));
        assert_eq!(7usize - mi0, ModInt::new_with(0, MOD));
    }

    #[test]
    fn test_new() {
        let mi0 = ModInt::new(0u64);
        let mi1 = ModInt::new(7u64);
        let mi2 = ModInt::new(1000000007u64);

        assert!(mi0 == 0);
        assert_eq!(mi0, ModInt::new(0));
        assert_eq!(mi1 + mi2, ModInt::new(7));
        assert_eq!(mi0 - mi1, ModInt::new(1000000007 - 7));
        assert_eq!(100 * mi1, ModInt::new(700u64));
        assert_eq!(100u64 * mi1 * 2 / 10 / ModInt::new(5), ModInt::new(28));
    }
    #[test]
    fn test_inverse() {
        const MOD: u64 = 13;

        assert_eq!(1, ModInt::new_with(1, MOD).inverse());
        assert_eq!(7, ModInt::new_with(2, MOD).inverse());
        assert_eq!(9, ModInt::new_with(3, MOD).inverse());
        assert_eq!(10, ModInt::new_with(4, MOD).inverse());
        assert_eq!(8, ModInt::new_with(5, MOD).inverse());
        assert_eq!(11, ModInt::new_with(6, MOD).inverse());
        assert_eq!(2, ModInt::new_with(7, MOD).inverse());
        assert_eq!(5, ModInt::new_with(8, MOD).inverse());
        assert_eq!(3, ModInt::new_with(9, MOD).inverse());
        assert_eq!(4, ModInt::new_with(10, MOD).inverse());
        assert_eq!(6, ModInt::new_with(11, MOD).inverse());
        assert_eq!(12, ModInt::new_with(12, MOD).inverse());
    }

    #[test]
    fn test_div() {
        const MOD: u64 = 13;

        assert_eq!(4, (ModInt::new_with(2, MOD) / ModInt::new_with(7, MOD)));
        assert_eq!(4, (2u64 / ModInt::new_with(7, MOD)));
        assert_eq!(4, (ModInt::new_with(2, MOD) / 7));
    }

    #[test]
    fn test_mul() {
        const MOD: u64 = 13;

        assert_eq!(2, ModInt::new_with(3, MOD) * ModInt::new_with(5, MOD));
        assert_eq!(2, ModInt::new_with(3, MOD) * 5);
        assert_eq!(2, 3 * ModInt::new_with(5, MOD));
    }

    #[test]
    fn test_partialord() {
        const MOD: u64 = 13;

        assert!(ModInt::new_with(3, MOD) < ModInt::new_with(5, MOD));
        assert!(3 < ModInt::new_with(5, MOD));
        assert!(ModInt::new_with(3, MOD) < 5);
        assert!(!(ModInt::new(10) < 7));
    }

    #[test]
    fn test_to_primitive() {
        assert_ne!(2, ModInt::new(13).to_u64().unwrap());
    }
}
