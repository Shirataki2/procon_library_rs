#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
pub mod modint {
    use std::cell::RefCell;
    use std::ops::*;
    use std::mem::swap;

    type Num = i64;
    thread_local!(
        static MOD: RefCell<Num> = RefCell::new(0);
    );

    pub fn set_modint<T>(v: T)
    where
        Num: From<T>
    {
        MOD.with(|x| x.replace(Num::from(v)));
    }

    pub fn modulo() -> Num {
        MOD.with(|x| *x.borrow())
    }

    pub struct ModInt(Num);

    impl Clone for ModInt {
        fn clone(&self) -> ModInt {
            ModInt(self.0)
        }
    }

    impl Copy for ModInt {}

    impl ModInt {
        pub fn new<T>(v: T) -> ModInt
        where
            Num: From<T>
        {
            let mut v = Num::from(v);
            let m = modulo();
            if v >= m {
                v %= m;
            }
            if v < 0 {
                v = (v % m + m) % m;
            }
            ModInt(v)
        }

        fn internal_pow(&self, mut e: Num) -> ModInt {
            let mut result = 1;
            let mut cur = self.0;
            let m = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= m;
                }
                e >>= 1;
                cur = (cur * cur) % m;
            }
            ModInt(result)
        }

        pub fn pow<T>(&self, e: T) -> ModInt
        where
            Num: From<T>
        {
            self.internal_pow(Num::from(e))
        }

        pub fn value(&self) -> Num {
            self.0
        }

        pub fn inv(&self) -> Self
        {
            let (mut a, mut b, mut u, mut v) = (self.0, modulo(), 1, 0);
            while b > 0 {
                let t = a / b;
                a -= t * b;
                swap(&mut a, &mut b);
                u -= t * v;
                swap(&mut u, &mut v);
            }
            ModInt::new::<i64>(u)
        }
    }

    impl From<ModInt> for Num {
        fn from(m: ModInt) -> Num {
            m.value()
        }
    }

    impl<T> AddAssign<T> for ModInt
    where
        Num: From<T>
    {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T> Add<T> for ModInt
    where
        Num: From<T>
    {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }

    impl<T> SubAssign<T> for ModInt
    where
        Num: From<T>
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T> Sub<T> for ModInt
    where
        Num: From<T>
    {
        type Output = ModInt;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }

    impl<T> MulAssign<T> for ModInt
    where
        Num: From<T>
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }

    impl<T> Mul<T> for ModInt
    where
        Num: From<T>
    {
        type Output = ModInt;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T> DivAssign<T> for ModInt
    where
        Num: From<T>
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = ModInt(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T> Div<T> for ModInt
    where
        Num: From<T>
    {
        type Output = ModInt;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::modint::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    const PRIMES: [i64; 3] = [1_000_000_007, 1_000_000_009, 998_244_353];

    #[test]
    fn test_add_sub() {
        let mut rng = rand::thread_rng();
        for m in &PRIMES {
            set_modint(*m);
            for _ in 0..10000 {
                let x: i64 = rng.sample(Uniform::from(0..*m));
                let y: i64 = rng.sample(Uniform::from(0..*m));
                let mx = ModInt::new(x);
                let my = ModInt::new(y);
                assert_eq!((mx + my).value(), (x + y) % *m);
                assert_eq!((mx + y).value(), (x + y) % *m);
                assert_eq!((mx - my).value(), (x + *m - y) % *m);
                assert_eq!((mx - y).value(), (x + *m - y) % *m);
                let mut x = x;
                let mut mx = mx;
                x += y;
                mx += my;
                assert_eq!(mx.value(), x % *m);
                x += y;
                mx += y;
                assert_eq!(mx.value(), x % *m);
                x = (x + *m - y % *m) % *m;
                mx -= my;
                assert_eq!(mx.value(), x);
                x = (x + *m - y % *m) % *m;
                mx -= y;
                assert_eq!(mx.value(), x);
            }
        }
    }

    #[test]
    fn test_mul() {
        let mut rng = rand::thread_rng();
        for m in &PRIMES {
            set_modint(*m);
            for _ in 0..10000 {
                let x: i64 = rng.sample(Uniform::from(0..*m));
                let y: i64 = rng.sample(Uniform::from(0..*m));
                let mx = ModInt::new(x);
                let my = ModInt::new(y);
                assert_eq!((mx * my).value(), (x * y) % *m);
                assert_eq!((mx * y).value(), (x * y) % *m);
            }
        }
    }

    #[test]
    fn test_zero() {
        set_modint(1_000_000_007i64);
        let a = ModInt::new(1_000_000_000i64);
        let b = ModInt::new(7i64);
        let c = a + b;
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_pow() {
        set_modint(1_000_000_007i64);
        let a = ModInt::new(1_000_000i64);
        let a = a.pow(2i64);
        assert_eq!(a.value(), 999993007);
    }

    #[test]
    fn test_div() {
        set_modint(1_000_000_007i64);
        for i in 1..=100_000i64 {
            let mut a = ModInt::new(1i64);
            a /= i;
            a *= i;
            assert_eq!(a.value(), 1);
        }
    }

    #[test]
    fn test_invmod() {
        set_modint(7i64);
        assert_eq!(ModInt::new(3i64).inv().value(), 5);
        set_modint(429i64);
        assert_eq!(ModInt::new(2i64).inv().value(), 215);
        set_modint(1_000_000_007i64);
        assert_eq!(ModInt::new(123_456_789i64).inv().value(), 18_633_540);
    }
}