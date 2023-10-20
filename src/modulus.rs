/// 法を表す型。
/// なお、 `rem` 以外のメソッドの引数で `x: T` な型のものは、すべて `self.rem(x) == x` となる必要がある。
pub trait Modulus<T>: PartialEq {
    fn modulus(&self) -> T;

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

    fn is_prime(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct StaticModulus64<const M: u64>;
impl<const M: u64> StaticModulus64<M> {
    pub const fn primitive_root() -> u64 {
        match M {
            2 => 1,
            65537 => 3,
            167772161 => 3,
            469762049 => 3,
            754974721 => 11,
            998244353 => 3,
            _ => Self::compute_primitive_root()
        }
    }

    pub const fn compute_primitive_root() -> u64 {
        let mut primes = [2; 16];
        let mut primes_len = 1;
        let mut x = (M - 1) >> (M - 1).trailing_zeros();
        {
            let mut i = 3;
            while i * i <= x {
                if x % i == 0 {
                    primes[primes_len] = i;
                    primes_len += 1;
                    x /= i;
                    while x % i == 0 {
                        x /= i;
                    }
                }
                i += 2;
            }
            if x > 1 {
                primes[primes_len] = x;
                primes_len += 1;
            }
        }
        let mut g = 2;
        'find_root: loop {
            let mut i = 0;
            while i < primes_len {
                if mod_pow(g, (M - 1) / primes[i], M) == 1 {
                    g += 1;
                    continue 'find_root;
                }
                i += 1;
            }
            return g;
        }
    }
}
impl<const M: u64> Modulus<u64> for StaticModulus64<M> {
    fn modulus(&self) -> u64 {
        M
    }

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

    fn is_prime(&self) -> bool {
        is_prime(M)
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
    fn modulus(&self) -> T {
        self.0.clone()
    }

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

    fn is_prime(&self) -> bool {
        if self.modulus() == self.one() {
            return false;
        }
        let mut i = self.one() + self.one();
        while i.clone() * i.clone() <= self.modulus() {
            if self.modulus() % i.clone() == self.zero() {
                return false;
            }
            i = i + self.one();
        }
        true
    }
}

const fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 7 || n == 61 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let d = (n - 1) >> (n - 1).trailing_zeros();
    let witnesses = [2, 7, 61];
    let mut i = 0;
    while i < witnesses.len() {
        let mut t = d;
        let mut y = mod_pow(witnesses[i], t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y as u128 * y as u128 % n as u128) as u64;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 {
            return false;
        }
        i += 1;
    }
    true
}

const fn mod_pow(mut x: u64, mut y: u64, m: u64) -> u64 {
    let mut z = 1;
    while y != 0 {
        if y & 1 != 0 {
            z = (z as u128 * x as u128 % m as u128) as u64;
        }
        x = (x as u128 * x as u128 % m as u128) as u64;
        y >>= 1;
    }
    z
}