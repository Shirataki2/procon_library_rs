use crate::math::ntt::ntt;
use crate::math::algebra::num_trait;

pub mod fps {
    use super::ntt::*;
    use super::num_trait::*;
    use std::ops::*;
    use std::cmp::*;

    impl<M: ModuloPrimitive> Zero for ModInt<M> {
        fn zero() -> Self { Self::new(0) }
        fn is_zero(&self) -> bool { self.value() == 0 }
    }

    #[derive(Debug, Clone)]
    pub struct FPS<M: ModuloPrimitive>(Box<Vec<ModInt<M>>>);

    impl <M: ModuloPrimitive> FPS<M> {
        pub fn new(v: Vec<ModInt<M>>) -> Self {
            Self(Box::new(v))
        }

        pub fn with_size(size: usize) -> Self {
            let v = vec![ModInt::zero(); size];
            Self(Box::new(v))
        }

        pub fn values(&self) -> Vec<i64> {
            self.0.iter().map(|&v| v.value()).collect()
        }

        #[inline]
        fn head(&self, n: usize) -> Self {
            Self::new(self.0.clone().drain(..min(n, self.0.len())).collect())
        }

        #[inline]
        fn rev(&self) -> Self {
            let mut res = self.0.clone();
            res.reverse();
            Self(res)
        }

        #[inline]
        fn cut(&mut self) {
            while !self.0.is_empty() && self.0.iter().next_back().unwrap().value() == 0 {
                self.0.pop();
            }
        }

        fn inner_gcd(x: &Self, y: &Self) -> Self {
            let (x, y) = (x.clone(), y.clone());
            if y.0.is_empty() { return x }
            let r = x % y.clone();
            return Self::inner_gcd(&y, &r)
        }

        pub fn gcd(&self, r: &Self) -> Self {
            Self::inner_gcd(&self, &r)
        }

        pub fn diff(&self) -> Self {
            let n = self.0.len();
            let mut f = Self::with_size(n - 1);
            for i in 1..n {
                f[i-1] = self[i] * i as i64;
            }
            f
        }

        pub fn integral(&self) -> Self {
            let n = self.0.len();
            let mut f = Self::with_size(n + 1);
            for i in 0..n {
                f[i+1] = self[i] / (i + 1) as i64;
            }
            f
        }

        pub fn inv_degree(&self, mut deg: i64) -> Self {
            assert!(self[0] != ModInt::<M>::zero());
            if deg < 0 { deg = self.0.len() as i64; }
            let v = vec![ModInt::<M>::new(1) / self[0]];
            let mut res = Self::new(v);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                let rres = res.clone() + res.clone();
                let mres = res.clone() * res.clone();
                res = ( rres - mres * self.head(i << 1)).head(i << 1);
            });
            res.0.resize(deg as usize, ModInt::zero());
            res
        }

        pub fn inv(&self) -> Self {
            self.inv_degree(self.0.len() as i64)
        }

        pub fn log_degree(&self, deg: i64) -> Self {
            assert!(self[0] == ModInt::<M>::new(1));
            let mut v = (self.diff() * self.inv_degree(deg)).integral();
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn log(&self) -> Self {
            self.log_degree(self.0.len() as i64)
        }

        pub fn exp_degree(&self, deg: i64) -> Self {
            assert!(self[0] == ModInt::<M>::zero());
            let one = ModInt::<M>::new(1);
            let mut v = Self::new(vec![one]);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                v = v.clone() * (self.head(i << 1) - v.log_degree((i << 1) as i64) + one).head(i << 1);
            });
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn exp(&self) -> Self {
            self.exp_degree(self.0.len() as i64)
        }

        pub fn pow_degree(&self, n: usize, deg: i64) -> Self {
            let mut i = 0;
            while i < self.0.len() && self[i].value() == 0 { i += 1; }
            if i == self.0.len() { return Self::with_size(deg as usize); }
            if i * n >= deg as usize { return Self::with_size(deg as usize); }
            let k = self[i];
            let nm = ModInt::<M>::new(n as i64);
            let mut v = (((self.clone() >> i) / k).log_degree(deg) * nm).exp_degree(deg) * k.pow(n as i64) << (n * i);
            v.0.resize(deg as usize, ModInt::zero());
            v
        }

        pub fn pow(&self, n: usize) -> Self {
            self.pow_degree(n, self.0.len() as i64)
        }

        /// WIP
        pub fn sqrt_degree(&self, deg: i64) -> Self {
            let one = ModInt::<M>::new(1);
            let inv2 = one / 2;
            let mut v = Self::new(vec![one]);
            (0..).map(|i| 1 << i).take_while(|&i| i < deg as usize).for_each(|i| {
                v = (v.clone() + self.head(i << 1) * v.inv_degree((i << 1) as i64)).head(i << 1);
                v.0.iter_mut().for_each(|vi| *vi *= inv2);
            });
            v.0.resize(deg as usize, ModInt::zero());
            v

        }

        /// WIP
        pub fn sqrt(&self) -> Self {
            self.sqrt_degree(self.0.len() as i64)
        }
    }

    impl<M: ModuloPrimitive> Index<usize> for FPS<M> {
        type Output = ModInt<M>;
        #[inline]
        fn index(&self, idx: usize) -> &Self::Output {
            &self.0[idx]
        }
    }

    impl<M: ModuloPrimitive> IndexMut<usize> for FPS<M> {
        #[inline]
        fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
            &mut self.0[idx]
        }
    }

    impl<M: ModuloPrimitive> Neg for FPS<M> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self::Output {
            let mut v = *self.0;
            for i in 0..v.len() {
                v[i] = -v[i];
            }
            Self::new(v)
        }
    }

    impl<M: ModuloPrimitive> AddAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn add_assign(&mut self, rhs: ModInt<M>) {
            if self.0.is_empty() {
                self.0.resize(1, ModInt::zero());
            }
            self[0] += rhs;
        }
    }

    impl<M: ModuloPrimitive> AddAssign for FPS<M> {
        #[inline]
        fn add_assign(&mut self, rhs: FPS<M>) {
            if rhs.0.len() > self.0.len() { self.0.resize(rhs.0.len(), ModInt::zero()); }
            for i in 0..rhs.0.len() {
                self[i] += rhs[i];
            }
            self.cut();
        }
    }

    impl<M: ModuloPrimitive> Add<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn add(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x += rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Add for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x += rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> SubAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: ModInt<M>) {
            if self.0.is_empty() {
                self.0.resize(1, ModInt::zero());
            }
            self[0] -= rhs;
        }
    }

    impl<M: ModuloPrimitive> SubAssign for FPS<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: FPS<M>) {
            if rhs.0.len() > self.0.len() { self.0.resize(rhs.0.len(), ModInt::zero()); }
            for i in 0..rhs.0.len() {
                self[i] -= rhs[i];
            }
            self.cut();
        }
    }

    impl<M: ModuloPrimitive> Sub<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn sub(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x -= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Sub for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x -= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> MulAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: ModInt<M>) {
            for i in 0..self.0.len() {
                self[i] *= rhs;
            }
        }
    }

    impl<M: ModuloPrimitive> MulAssign for FPS<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: FPS<M>) {
            let v = NumberTheoreticTransform::<M>::multiply_modint(&self.0, &rhs.0);
            self.0 = Box::new(v);
        }
    }

    impl<M: ModuloPrimitive> Mul<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn mul(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x *= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Mul for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x *= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> DivAssign<ModInt<M>> for FPS<M> {
        #[inline]
        fn div_assign(&mut self, rhs: ModInt<M>) {
            let rinv = 1 / rhs;
            for i in 0..self.0.len() {
                self[i] *= rinv;
            }
        }
    }

    impl<M: ModuloPrimitive> DivAssign for FPS<M> {
        #[inline]
        fn div_assign(&mut self, rhs: FPS<M>) {
            assert!(!rhs.0.is_empty());
            assert!(rhs.0.iter().next_back().unwrap() != &ModInt::<M>::zero());
            self.cut();
            if self.0.len() < rhs.0.len() {
                self.0.clear();
                return
            }
            let need = self.0.len() - rhs.0.len() + 1;
            let v = (self.rev().head(need) * rhs.rev().inv_degree(need as i64)).head(need).rev();
            *self = v;
        }
    }

    impl<M: ModuloPrimitive> Div<ModInt<M>> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn div(self, rhs: ModInt<M>) -> Self::Output {
            let mut x = self;
            x /= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> Div for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x /= rhs;
            x
        }
    }

    impl<M: ModuloPrimitive> RemAssign for FPS<M> {
        fn rem_assign(&mut self, rhs: Self) {
            self.cut();
            let r = self.clone();
            let q = r / rhs.clone();
            *self -= q * rhs;
        }
    }

    impl<M: ModuloPrimitive> Rem for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn rem(self, rhs: Self) -> Self::Output {
            let mut x = self;
            x %= rhs;
            x
        }
    }

    // <<=
    impl<M: ModuloPrimitive> ShlAssign<usize> for FPS<M> {
        #[inline]
        fn shl_assign(&mut self, x: usize) {
            let mut v = vec![ModInt::<M>::zero(); x];
            v.append(&mut self.0);
            self.0 = Box::new(v);
        }
    }

    impl<M: ModuloPrimitive> Shl<usize> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn shl(self, rhs: usize) -> Self::Output {
            let mut x = self;
            x <<= rhs;
            x
        }
    }

    // >>=
    impl<M: ModuloPrimitive> ShrAssign<usize> for FPS<M> {
        #[inline]
        fn shr_assign(&mut self, x: usize) {
            self.0 = Box::new(self.0.drain(x..).collect());
        }
    }

    impl<M: ModuloPrimitive> Shr<usize> for FPS<M> {
        type Output = FPS<M>;
        #[inline]
        fn shr(self, rhs: usize) -> Self::Output {
            let mut x = self;
            x >>= rhs;
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fps::*;
    use super::ntt::*;

    #[test]
    fn test_inv_fps() {
        let v = vec![5, 4, 3, 2, 1];
        let v = v.iter().map(|&v| ModInt::<Mod998244353>::new(v)).collect();
        let f = FPS::<Mod998244353>::new(v);
        let g = f.inv();
        assert_eq!(g.values(), vec![598946612, 718735934, 862483121, 635682004, 163871793]);
    }

    #[test]
    fn test_exp_fps() {
        let v = vec![0, 1, 2, 3, 4];
        let v = v.iter().map(|&v| ModInt::<Mod998244353>::new(v)).collect();
        let f = FPS::<Mod998244353>::new(v);
        let g = f.exp();
        assert_eq!(g.values(), vec![1, 1, 499122179, 166374064, 291154613]);
    }

    #[test]
    fn test_log_fps() {
        let v = vec![1, 1, 499122179, 166374064, 291154613];
        let v = v.iter().map(|&v| ModInt::<Mod998244353>::new(v)).collect();
        let f = FPS::<Mod998244353>::new(v);
        let g = f.log();
        assert_eq!(g.values(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_pow_fps() {
        let v = vec![0, 0, 9, 12];
        let v = v.iter().map(|&v| ModInt::<Mod998244353>::new(v)).collect();
        let f = FPS::<Mod998244353>::new(v);
        let g = f.pow(3);
        assert_eq!(g.values(), vec![0; 4]);
    }

    #[test]
    #[ignore]
    fn test_sqrt_fps() {
        let v = vec![0, 0, 9, 12];
        let v = v.iter().map(|&v| ModInt::<Mod998244353>::new(v)).collect();
        let f = FPS::<Mod998244353>::new(v);
        let g = f.sqrt();
        assert_eq!(g.values(), vec![0, 3, 2, 332748117]);
    }
}