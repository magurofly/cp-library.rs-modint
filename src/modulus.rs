/// 法を表す型。
/// なお、 `rem` 以外のメソッドの引数で `x: T` な型のものは、すべて `self.rem(x) == x` となる必要がある。
pub trait Modulus<T>: PartialEq {
    /// あまりを返す。
    fn rem(&self, x: T) -> T;
    
    fn zero(&self) -> T;
    
    fn one(&self) -> T;
    
    /// 符号を反転したときの値を返す。
    fn neg(&self, x: T) -> T;
    
    fn inv(&self, x: T) -> T;
    
    fn add(&self, x: T, y: T) -> T;
    
    fn mul(&self, x: T, y: T) -> T;
    
    fn sub(&self, x: T, y: T) -> T {
        self.add(x, self.neg(y))
    }
    
    fn div(&self, x: T, y: T) -> T {
        self.mul(x, self.inv(y))
    }
    
    fn pow(&self, mut x: T, mut y: usize) -> T where T: Clone {
        let mut z = self.one();
        while y != 0 {
            if y & 1 != 0 {
                z = self.mul(z, x.clone());
            }
            x = self.mul(x.clone(), x);
            y >>= 1;
        }
        z
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct StaticModulus64<const M: u64>;
impl<const M: u64> Modulus<u64> for StaticModulus64<M> {
    fn rem(&self, x: u64) -> u64 {
        x % M
    }
    
    fn zero(&self) -> u64 {
        0
    }
    
    fn one(&self) -> u64 {
        1
    }
    
    fn neg(&self, x: u64) -> u64 {
        assert!(x < M);
        if x == 0 {
            0
        } else {
            M - x
        }
    }
    
    fn inv(&self, x: u64) -> u64 {
        assert!(x < M);
        assert!(x != 0, "division by zero occured");
        let mut s = (M as i64, 0);
        let mut t = (x as i64, 1);
        while t.0 != 0 {
            let u = s.0 / t.0;
            s.0 -= t.0 * u;
            s.1 -= t.1 * u;
            std::mem::swap(&mut s, &mut t);
        }
        assert!(s.0 == 1, "gcd({}, {}) = {}, which is not 1", x, M, s.1);
        if s.1 < 0 {
            s.1 += (M / s.0 as u64) as i64;
        }
        s.1 as u64
    }
    
    fn add(&self, x: u64, y: u64) -> u64 {
        assert!(x < M && y < M);
        let mut z = x + y;
        if z >= M {
            z -= M;
        }
        z
    }
    
    fn mul(&self, x: u64, y: u64) -> u64 {
        assert!(x < M && y < M);
        (x as u128 * y as u128 % M as u128) as u64
    }
}

pub type Mod998244353 = StaticModulus64<998244353>;
pub type Mod1000000007 = StaticModulus64<1000000007>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicModulus<T>(T);
impl<T: Clone> DynamicModulus<T> {
    pub fn new(modulus: T) -> Self {
        Self(modulus)
    }
    
    pub fn modulus(&self) -> T {
        self.0.clone()
    }
}

impl<T> Modulus<T> for DynamicModulus<T>
where
T: Clone +
std::ops::Rem<Output = T> +
Ord +
std::convert::From<bool> +
std::ops::Neg<Output = T> +
std::ops::Add<Output = T> +
std::ops::Mul<Output = T> +
std::ops::Div<Output = T>
{
    fn rem(&self, x: T) -> T {
        x % self.modulus()
    }
    
    fn zero(&self) -> T {
        T::from(false)
    }
    
    fn one(&self) -> T {
        T::from(true) % self.modulus()
    }
    
    fn neg(&self, x: T) -> T {
        -x % self.modulus()
    }
    
    fn inv(&self, x: T) -> T {
        assert!(x != self.zero(), "division by zero occured");
        let mut s = (self.modulus(), self.zero());
        let mut t = (x, self.one());
        while t.0 != self.zero() {
            let u = s.0.clone() / t.0.clone();
            s.0 = s.0 + -t.0.clone() * u.clone();
            s.1 = s.1 + -t.1.clone() * u.clone();
            std::mem::swap(&mut s, &mut t);
        }
        if s.1 < self.zero() {
            s.1 = self.add(s.1, self.modulus() / s.0.clone());
        }
        s.1
    }
    
    fn add(&self, x: T, y: T) -> T {
        (x + y) % self.0.clone()
    }
    
    fn mul(&self, x: T, y: T) -> T {
        (x * y) % self.0.clone()
    }
}