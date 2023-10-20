pub mod modulus;
pub use modulus::Modulus;

pub type ModInt998244353 = ModInt<u64, Mod998244353>;
pub type ModInt1000000007 = ModInt<u64, Mod1000000007>;

use modulus::*;
use std::ops::*;
use std::mem::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModInt<T, M> {
    value: T,
    modulus: M,
}

impl<T, M: Modulus<T>> ModInt<T, M> {
    pub fn new(value: T, modulus: M) -> Self {
        Self::raw(modulus.rem(value), modulus)
    }

    fn raw(value: T, modulus: M) -> Self {
        Self { value, modulus }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn modulus(&self) -> &M {
        &self.modulus
    }
}

impl<T, M: Modulus<T>> ModInt<T, M> {
    pub fn zero() -> Self where M: Default {
        let modulus = M::default();
        Self::raw(modulus.zero(), modulus)
    }

    pub fn one() -> Self where M: Default {
        let modulus = M::default();
        Self::raw(modulus.one(), modulus)
    }

    pub fn inv(self) -> Self {
        Self::raw(self.modulus.inv(self.value), self.modulus)
    }

    pub fn pow(self, n: usize) -> Self where T: Clone {
        Self::raw(self.modulus.pow(self.value, n), self.modulus)
    }
}

macro_rules! define_binary_operation {
    ($Op:ident, $op:ident, $OpAssign:ident, $op_assign:ident) => {
        impl<T, M: Modulus<T>> $Op for ModInt<T, M> {
            type Output = Self;
            fn $op(self, other: Self) -> Self {
                assert!(self.modulus == other.modulus, "mod mismatch");
                Self::raw(self.modulus.$op(self.value, other.value), self.modulus)
            }
        }

        impl<T, M: Modulus<T>> $OpAssign for ModInt<T, M> {
            fn $op_assign(&mut self, other: Self) {
                assert!(self.modulus == other.modulus, "mod mismatch");
                unsafe {
                    let value = replace(&mut self.value, MaybeUninit::uninit().assume_init());
                    self.value = self.modulus.$op(value, other.value);
                }
            }
        }

        impl<T, M: Modulus<T>> $Op<T> for ModInt<T, M> {
            type Output = Self;
            fn $op(self, other: T) -> Self {
                Self::raw(self.modulus.$op(self.value, self.modulus.rem(other)), self.modulus)
            }
        }

        impl<T, M: Modulus<T>> $OpAssign<T> for ModInt<T, M> {
            fn $op_assign(&mut self, other: T) {
                unsafe {
                    let value = replace(&mut self.value, MaybeUninit::uninit().assume_init());
                    self.value = self.modulus.$op(value, self.modulus.rem(other));
                }
            }
        }
    };
}

define_binary_operation!(Add, add, AddAssign, add_assign);
define_binary_operation!(Sub, sub, SubAssign, sub_assign);
define_binary_operation!(Mul, mul, MulAssign, mul_assign);
define_binary_operation!(Div, div, DivAssign, div_assign);

impl<T: std::fmt::Display, M> std::fmt::Display for ModInt<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T, M: Modulus<T> + Default> std::convert::From<T> for ModInt<T, M> {
    fn from(value: T) -> Self {
        Self::new(value, M::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_integer::*;

    #[test]
    fn inv() {
        for i in 1 .. 24 {
            type M = ModInt<u64, StaticModulus64<24>>;
            if gcd(i, 24) == 1 {
                let j = M::from(i).inv();
                assert!(j * i == M::from(1), "{} * {} != 1", j, i);
            }
        }
    }
}
