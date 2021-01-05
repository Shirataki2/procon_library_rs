//! Verified [AtCoder Typical Contest 001 C - 高速フーリエ変換](https://atcoder.jp/contests/atc001/submissions/19095995)
//! Verified [Library Checker](https://judge.yosupo.jp/submission/34580)
//! TODO 定数倍がけっこう重いから軽量化しないとね

pub mod ntt {
    use std::marker::PhantomData;
    use std::ops::*;

    type Num = i64;

    pub trait ModuloPrimitive: Clone + Copy {
        fn modulo() -> Num;
        fn primitive_root() -> Num;
    }

    macro_rules! define_modulo_primitive {
        ($name:ident, $mod:expr, $proot:expr) => {
            #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug)]
    pub struct ModInt<M>(Num, PhantomData<M>);

    impl<M> Clone for ModInt<M> {
        fn clone(&self) -> ModInt<M> {
            ModInt(self.0, PhantomData)
        }
    }

    impl<M> Copy for ModInt<M> {}

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
                let tmp = a / b;
                a -= tmp * b;
                std::mem::swap(&mut a, &mut b);
                u -= tmp * v;
                std::mem::swap(&mut u, &mut v);
            }
            ModInt::new::<Num>(u)
        }
    }

    impl<M> Neg for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self::new(M::modulo() - self.0)
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

    impl<M> AddAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn add_assign(&mut self, rhs: ModInt<M>) {
            *self += rhs.value();
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

    impl<M> Add for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res += rhs.value();
            res
        }
    }

    impl<M> Add<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res += rhs.value();
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

    impl<M> SubAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn sub_assign(&mut self, rhs: ModInt<M>) {
            *self -= rhs.value();
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

    impl<M> Sub for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res -= rhs.value();
            res
        }
    }

    impl<M> Sub<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res -= rhs.value();
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

    impl<M> MulAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn mul_assign(&mut self, rhs: ModInt<M>) {
            *self *= rhs.value();
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

    impl<M> Mul for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res *= rhs.value();
            res
        }
    }

    impl<M> Mul<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res *= rhs.value();
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

    impl<M> DivAssign for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn div_assign(&mut self, rhs: ModInt<M>) {
            *self /= rhs.value();
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

    impl<M> Div for ModInt<M>
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = self;
            res /= rhs.value();
            res
        }
    }

    impl<M> Div<ModInt<M>> for Num
    where
        M: ModuloPrimitive
    {
        type Output = ModInt<M>;
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut res = ModInt::<M>::new(self);
            res /= rhs.value();
            res
        }
    }
    
    impl<M> PartialEq for ModInt<M>
    where
        M: ModuloPrimitive
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.value() == rhs.value()
        }
    }

    pub struct NumberTheoreticTransform<M>(PhantomData<M>);

    impl<M> NumberTheoreticTransform<M>
    where
        M: ModuloPrimitive
    {
        fn bit_reverse(f: &mut Vec<ModInt<M>>) {
            let mut i = 0;
            for j in 1..f.len()-1 {
                let mut k = f.len() >> 1;
                while { i ^= k; k > i } { k >>= 1; }
                if i > j { f.swap(i, j); }
            }
        }

        fn dft(f: &mut Vec<ModInt<M>>, inverse: bool) {
            let n = f.len();
            NumberTheoreticTransform::<M>::bit_reverse(f);
            let proot = ModInt::<M>::new(M::primitive_root());
            for i in (0..).map(|i| 1 << i).take_while(|&i| i < n) {
                let mut w = proot.pow((M::modulo() - 1) / (2 * i as Num));
                if inverse { w = 1 / w; }
                for k in 0..i {
                    let wn = w.pow(k as Num);
                    for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < n) {
                        let left = f[j + k];
                        let right = f[j + k + i] * wn;
                        f[j + k] = left + right;
                        f[j + k + i] = left - right;
                    }
                }
            }
            if inverse {
                f.iter_mut().for_each(|fi| { *fi /= ModInt::<M>::new(n as Num); })
            }
        }

        pub fn multiply(f: &[Num], g: &[Num]) -> Vec<Num> {
            let m = f.len() + g.len() - 1;
            let n = m.next_power_of_two();
            let zero = ModInt::<M>::new(0);
            let mut ff = vec![zero; n];
            let mut gg = vec![zero; n];
            for i in 0..f.len() { ff[i] += ModInt::<M>::new(f[i]); }
            for i in 0..g.len() { gg[i] += ModInt::<M>::new(g[i]); }
            NumberTheoreticTransform::<M>::dft(&mut ff, false);
            NumberTheoreticTransform::<M>::dft(&mut gg, false);
            for i in 0..n { ff[i] *= gg[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, true);
            ff.resize(m, zero);
            ff.iter().map(|&v| v.value()).collect()
        }

        pub fn multiply_modint(f: &[ModInt<M>], g: &[ModInt<M>]) -> Vec<ModInt<M>> {
            let m = f.len() + g.len();
            let n = m.next_power_of_two();
            let zero = ModInt::<M>::new(0);
            let mut ff = vec![zero; n];
            let mut gg = vec![zero; n];
            for i in 0..f.len() { ff[i] += f[i]; }
            for i in 0..g.len() { gg[i] += g[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, false);
            NumberTheoreticTransform::<M>::dft(&mut gg, false);
            for i in 0..n { ff[i] *= gg[i]; }
            NumberTheoreticTransform::<M>::dft(&mut ff, true);
            ff.resize(m-1, zero);
            ff
        }
    }

    pub fn multiply_for_any_mod(f: &mut Vec<Num>, g: &mut Vec<Num>, modulo: Num) -> Vec<Num> {
        f.iter_mut().for_each(|v| *v %= modulo);
        g.iter_mut().for_each(|v| *v %= modulo);
        let f = f.to_vec();
        let g = g.to_vec();
        type M2 = ModInt<Mod469762049>;
        type M3 = ModInt<Mod1224736769>;
        type NTT1 = NumberTheoreticTransform::<Mod167772161>;
        type NTT2 = NumberTheoreticTransform::<Mod469762049>;
        type NTT3 = NumberTheoreticTransform::<Mod1224736769>;
        let ntt1 = NTT1::multiply(&f, &g);
        let ntt2 = NTT2::multiply(&f, &g);
        let ntt3 = NTT3::multiply(&f, &g);
        let (m1, m2) = (Mod167772161::modulo(), Mod469762049::modulo());
        let m1_inv_m2 = 1 / M2::new(m1);
        let m12_inv_m3 = 1 / (M3::new(m1) * M3::new(m2));
        let mut ret = vec![0; ntt1.len()];
        for i in 0..ntt1.len() {
            let v1 = ((M2::new(ntt2[i]) - M2::new(ntt1[i])) * m1_inv_m2).value();
            let v2 = ((M3::new(ntt3[i]) - (M3::new(ntt1[i]) + M3::new(m1) * M3::new(v1))) * m12_inv_m3).value();
            ret[i] = add(ntt1[i], add(mul(m1, v1, modulo), mul(mul(m1, m2, modulo), v2, modulo), modulo), modulo);
        }
        ret
    }

    fn add(mut x: Num, y: Num, modulo: Num) -> Num {
        x += y;
        if x >= modulo {
            x -= modulo;
        }
        x
    }

    fn mul(x: Num, y: Num, modulo: Num) -> Num {
        x * y % modulo
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
        let x = NTT::multiply(&f, &g);
        assert_eq!(x, vec![0, 0, 1, 4, 11, 26, 36, 40, 32]);
    }

    #[test]
    fn test_yosupo() {
        let f = vec![1, 2, 3, 4];
        let g = vec![5, 6, 7, 8, 9];
        let x = NTT::multiply(&f, &g);
        assert_eq!(x, vec![5, 16, 34, 60, 70, 70, 59, 36]);

        let f = vec![10000000];
        let g = vec![10000000];
        let x = NTT::multiply(&f, &g);
        assert_eq!(x, vec![871938225]);
    }

    #[test]
    fn test_yosupo_mod1e9p7() {
        let mut f = vec![1, 2, 3, 4];
        let mut g = vec![5, 6, 7, 8, 9];
        let x = multiply_for_any_mod(&mut f, &mut g, 1_000_000_007);
        assert_eq!(x, vec![5, 16, 34, 60, 70, 70, 59, 36]);

        let mut f = vec![10000000];
        let mut g = vec![10000000];
        let x = multiply_for_any_mod(&mut f, &mut g, 1_000_000_007);
        assert_eq!(x, vec![999300007]);
    }
}