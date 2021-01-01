pub mod ntt {
    use std::marker::PhantomData;
    use std::ops::*;

    type Num = i64;

    pub trait ModuloPrimitive {
        fn modulo() -> Num;
        fn primitive_root() -> Num;

        fn add(mut x: Num, y: Num) -> Num {
            x += y;
            if x >= Self::modulo() {
                x -= Self::modulo();
            }
            x
        }

        fn mul(x: Num, y: Num) -> Num {
            ((x as i128) * (y as i128) % Self::modulo() as i128) as i64
        }

        fn pow(mut x: Num, mut n: Num) -> Num {
            let mut res = 1;
            while n > 0 {
                if n & 1 > 0 { res = Self::mul(res, x); }
                x = Self::mul(x, x);
                n >>= 1;
            }
            res
        }

        fn inv(x: Num) -> Num {
            Self::pow(x, Self::modulo() - 2)
        }
    }

    macro_rules! define_modulo_primitive {
        ($name:ident, $mod:expr, $proot:expr) => {
            pub struct $name;
            impl ModuloPrimitive for $name {
                fn modulo() -> i64 { $mod }
                fn primitive_root() -> i64 { $proot }
            }
        };
    }

    define_modulo_primitive!(Mod924844033, 924844033, 5);
    define_modulo_primitive!(Mod998244353, 998244353, 3);
    define_modulo_primitive!(Mod1012924417, 1012924417, 5);
    define_modulo_primitive!(Mod167772161, 167772161, 3);
    define_modulo_primitive!(Mod469762049, 469762049, 3);
    define_modulo_primitive!(Mod1224736769, 1224736769, 3);

    pub struct ModInt<M>(Num, PhantomData<M>);

    impl<M: ModuloPrimitive> ModInt<M> {
        pub fn new<T>(v: T) -> ModInt<M>
        where
            Num: From<T>
        {
            let mut v = Num::from(v);
            let m = M::modulo();
            if v >= m {
                v %= m;
            }
            if v < 0 {
                v = (v % m + m) % m;
            }
            ModInt(v, PhantomData)
        }

        fn internal_pow(&self, mut e: Num) -> ModInt<M> {
            let mut result = 1;
            let mut cur = self.0;
            let m = M::modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= m;
                }
                e >>= 1;
                cur = (cur * cur) % m;
            }
            ModInt(result, PhantomData)
        }

        pub fn pow<T>(&self, e: T) -> ModInt<M>
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
            let (mut a, mut b, mut u, mut v) = (self.0, M::modulo(), 1, 0);
            while b > 0 {
                let t = a / b;
                a -= t * b;
                std::mem::swap(&mut a, &mut b);
                u -= t * v;
                std::mem::swap(&mut u, &mut v);
            }
            ModInt::new::<Num>(u)
        }
    }

    impl<T, M> AddAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T, M> Add<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }

    impl<T, M> SubAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
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

    impl<T, M> Sub<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }

    impl<T, M> MulAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }

    impl<T, M> Mul<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T, M> DivAssign<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = Num::from(rhs);
            let m = M::modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = ModInt::<M>(rhs, PhantomData).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T, M> Div<T> for ModInt<M>
    where
        Num: From<T>,
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }

    pub struct NumberTheoreticTransform<M>(PhantomData<M>);

    impl<M> NumberTheoreticTransform<M>
    where
        M: ModuloPrimitive
    {
        fn bit_reverse(f: &mut Vec<Num>) {
            let mut i = 0;
            for j in 1..f.len()-1 {
                let mut k = f.len() >> 1;
                while { i ^= k; k > i } { k >>= 1; }
                if i > j { f.swap(i, j); }
            }
        }

        fn dft(f: &mut Vec<Num>, inverse: bool) {
            let n = f.len();
            NumberTheoreticTransform::<M>::bit_reverse(f);
            for i in (0..).map(|i| 1 << i).take_while(|&i| i < n) {
                let mut w = M::pow(M::primitive_root(), (M::modulo() - 1) / (2 * i as i64));
                if inverse { w = M::inv(w); }
                for k in 0..i {
                    let wn = M::pow(w, k as i64);
                    for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < n) {
                        let s = f[j + k];
                        let t = M::mul(f[j + k + i], wn);
                        f[j + k] = M::add(s, t);
                        f[j + k + i] = M::add(s, M::modulo() - t);
                    }
                }
            }
            if inverse {
                let ninv = M::inv(n as Num);
                for i in 0..n { f[i] = M::mul(f[i], ninv) }
            }
        }

        pub fn multiply(f: Vec<Num>, g: Vec<Num>) -> Vec<Num> {
            let m = f.len() + g.len() + 1;
            let n = m.next_power_of_two();
            let mut ff = vec![0; n];
            let mut gg = vec![0; n];
            for i in 0..f.len() { ff[i] += f[i]; }
            for i in 0..g.len() { gg[i] += g[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, false);
            NumberTheoreticTransform::<M>::dft(&mut gg, false);
            for i in 0..n { ff[i] = M::mul(ff[i], gg[i]); }
            NumberTheoreticTransform::<M>::dft(&mut ff, true);
            ff.resize(m, 0);
            ff
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ntt::*;

    type NTT = NumberTheoreticTransform::<Mod998244353>;

    #[test]
    fn test_atc001c() {
        let f = vec![0, 1, 2, 3, 4];
        let g = vec![0, 1, 2, 4, 8];
        let x = NTT::multiply(f, g);
        assert_eq!(x, vec![0, 0, 1, 4, 11, 26, 36, 40, 32, 0, 0]);
    }
}