pub mod complex {
    use std::ops::*;

    #[derive(Debug, Copy, Clone)]
    pub struct Complex(pub f64, pub f64);

    impl Complex {
        pub fn i() -> Complex { Complex(0.0, 1.0) }

        pub fn from_polar(r: f64, theta: f64) -> Complex {
            Complex(r * theta.cos(), r * theta.sin())
        }

        pub fn real(&self) -> f64 { self.0 }

        pub fn imag(&self) -> f64 { self.1 }

        pub fn norm(&self) -> f64 {
            self.0.hypot(self.1)
        }
        pub fn sqnorm(&self) -> f64 {
            self.0 * self.0 + self.1 * self.1
        }

        pub fn arg(&self) -> f64 {
            self.1.atan2(self.0)
        }

        pub fn conj(&self) -> Complex {
            Complex(self.0, -self.1)
        }

        pub fn sinh(&self) -> Complex {
            Complex(self.0.sinh() * self.1.cos(), self.0.cosh() * self.1.sin())
        }

        pub fn cosh(&self) -> Complex {
            Complex(self.0.cosh() * self.1.cos(), self.0.sinh() * self.1.sin())
        }

        pub fn tanh(&self) -> Complex {
            let (r_2, i_2) = (self.0 + self.0, self.1 + self.1);
            Complex(r_2.sinh(), i_2.sin()) / (r_2.cosh() + i_2.cos())
        }

        pub fn sin(&self) -> Complex {
            Complex(self.0.sin() * self.1.cosh(), self.0.cos() * self.1.sinh())
        }

        pub fn cos(&self) -> Complex {
            Complex(self.0.cos() * self.1.cosh(), -self.0.sin() * self.1.sinh())
        }

        pub fn tan(&self) -> Complex {
            let (r_2, i_2) = (self.0 + self.0, self.1 + self.1);
            Complex(r_2.sin(), i_2.sinh()) / (r_2.cos() + i_2.cosh())
        }

        pub fn to_polar(&self) -> (f64, f64) {
            (self.norm(), self.arg())
        }

        pub fn exp(&self) -> Complex {
            Complex::from_polar(self.0.exp(), self.1)
        }

        pub fn ln(&self) -> Complex {
            let (r, arg) = self.to_polar();
            Complex(r.ln(), arg)
        }
    }

    impl Neg for Complex { type Output = Complex; fn neg(self) -> Complex { Complex(-self.0, -self.1) } }

    impl AddAssign for Complex { fn add_assign(&mut self, rhs: Complex) { self.0 += rhs.0; self.1 += rhs.1; } }
    impl AddAssign<f64> for Complex { fn add_assign(&mut self, rhs: f64) { self.0 += rhs; } }
    impl Add for Complex { type Output = Complex; fn add(self, rhs: Complex) -> Complex { let mut res = self; res += rhs; res } }
    impl Add<f64> for Complex { type Output = Complex; fn add(self, rhs: f64) -> Complex { let mut res = self; res += rhs; res } }
    impl Add<Complex> for f64 { type Output = Complex; fn add(self, rhs: Complex) -> Complex { let mut res = rhs; res.0 += self; res } }

    impl SubAssign for Complex { fn sub_assign(&mut self, rhs: Complex) { self.0 -= rhs.0; self.1 -= rhs.1; } }
    impl SubAssign<f64> for Complex { fn sub_assign(&mut self, rhs: f64) { self.0 -= rhs; } }
    impl Sub for Complex { type Output = Complex; fn sub(self, rhs: Complex) -> Complex { let mut res = self; res -= rhs; res } }
    impl Sub<f64> for Complex { type Output = Complex; fn sub(self, rhs: f64) -> Complex { let mut res = self; res -= rhs; res } }
    impl Sub<Complex> for f64 { type Output = Complex; fn sub(self, rhs: Complex) -> Complex { Complex(self - rhs.0, - rhs.1) } }

    impl MulAssign for Complex { fn mul_assign(&mut self, rhs: Complex) { let r = self.0 * rhs.0 - self.1 * rhs.1; let i = self.0 * rhs.1 + self.1 * rhs.0; self.0 = r; self.1 = i; } }
    impl MulAssign<f64> for Complex { fn mul_assign(&mut self, rhs: f64) { self.0 *= rhs; self.1 *= rhs; } }
    impl Mul for Complex { type Output = Complex; fn mul(self, rhs: Complex) -> Complex { let mut res = self; res *= rhs; res } }
    impl Mul<f64> for Complex { type Output = Complex; fn mul(self, rhs: f64) -> Complex { let mut res = self; res *= rhs; res } }
    impl Mul<Complex> for f64 { type Output = Complex; fn mul(self, rhs: Complex) -> Complex { Complex(self * rhs.0, self * rhs.1) } }

    impl DivAssign for Complex { fn div_assign(&mut self, rhs: Complex) { let n = rhs.sqnorm(); let r = (self.0 * rhs.0 + self.1 * rhs.1) / n; let i = (self.1 * rhs.0 - self.0 * rhs.1) / n; self.0 = r; self.1 = i; } }
    impl DivAssign<f64> for Complex { fn div_assign(&mut self, rhs: f64) { self.0 /= rhs; self.1 /= rhs; } }
    impl Div for Complex { type Output = Complex; fn div(self, rhs: Complex) -> Complex { let mut res = self; res /= rhs; res } }
    impl Div<f64> for Complex { type Output = Complex; fn div(self, rhs: f64) -> Complex { let mut res = self; res /= rhs; res } }
    impl Div<Complex> for f64 { type Output = Complex; fn div(self, rhs: Complex) -> Complex { Complex(self, 0.0) / rhs } }

    impl PartialEq<Complex> for Complex { fn eq(&self, rhs: &Complex) -> bool { self.0 == rhs.0 && self.1 == rhs.1 } }
    impl PartialEq<f64> for Complex { fn eq(&self, rhs: &f64) -> bool { self.0 == *rhs } }
    impl PartialEq<Complex> for f64 { fn eq(&self, rhs: &Complex) -> bool { *self == rhs.0 } }

    impl From<f64> for Complex { fn from(x: f64) -> Complex { Complex(x, 0.0) }}

    impl std::fmt::Display for Complex { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}{}{}j)", self.0, if self.1 > 0.0 { "+" } else { "" }, self.1)
    }}
}

#[cfg(test)]
mod tests {
    use super::complex::*;

    fn approx_eq(a: Complex, b: Complex) -> bool {
        eprintln!("left: {}, right: {}", a, b);
        (a - b).norm() <= 1e-8
    }

    #[test]
    fn test_add() {
        let mut a = Complex(3.0, 4.0);
        let b = Complex(-3.0, 7.0);
        let c = Complex(4.0, -2.0);
        let d = Complex(-4.0, -3.0);
        assert_eq!(a+b, Complex(0.0, 11.0));
        assert_eq!(a+c, Complex(7.0, 2.0));
        assert_eq!(a+d, Complex(-1.0, 1.0));
        assert_eq!(b+c, Complex(1.0, 5.0));
        assert_eq!(b+d, Complex(-7.0, 4.0));
        assert_eq!(c+d, Complex(0.0, -5.0));
        a += Complex(1.0, 1.0);
        assert_eq!(a, Complex(4.0, 5.0));
        assert_eq!(a+1.0, Complex(5.0, 5.0));
        assert_eq!(1.0+a, Complex(5.0, 5.0));
        a += -1.0;
        assert_eq!(a, Complex(3.0, 5.0));
        assert_eq!(a+(-1.0), Complex(2.0, 5.0));
        assert_eq!(-1.0+a, Complex(2.0, 5.0));
    }

    #[test]
    fn test_sub() {
        let mut a = Complex(3.0, 4.0);
        let b = Complex(-3.0, 7.0);
        let c = Complex(4.0, -2.0);
        let d = Complex(-4.0, -3.0);
        assert_eq!(a-b, Complex(6.0, -3.0));
        assert_eq!(a-c, Complex(-1.0, 6.0));
        assert_eq!(a-d, Complex(7.0, 7.0));
        assert_eq!(b-c, Complex(-7.0, 9.0));
        assert_eq!(b-d, Complex(1.0, 10.0));
        assert_eq!(c-d, Complex(8.0, 1.0));
        a -= Complex(1.0, 1.0);
        assert_eq!(a, Complex(2.0, 3.0));
        assert_eq!(a-1.0, Complex(1.0, 3.0));
        assert_eq!(1.0-a, Complex(-1.0, -3.0));
        a -= -1.0;
        assert_eq!(a, Complex(3.0, 3.0));
        assert_eq!(a-(-1.0), Complex(4.0, 3.0));
        assert_eq!(-1.0-a, Complex(-4.0, -3.0));
    }

    #[test]
    fn test_mul() {
        let mut a = Complex(2.0, 3.0);
        let b = Complex(-3.0, 7.0);
        let c = Complex(4.0, -2.0);
        let d = Complex(-4.0, -3.0);
        assert_eq!(a*b, Complex(-27.0, 5.0));
        assert_eq!(a*c, Complex(14.0, 8.0));
        assert_eq!(a*d, Complex(1.0, -18.0));
        assert_eq!(b*c, Complex(2.0, 34.0));
        assert_eq!(b*d, Complex(33.0, -19.0));
        assert_eq!(c*d, Complex(-22.0, -4.0));
        a *= Complex(3.0, 1.0);
        assert_eq!(a, Complex(3.0, 11.0));
        assert_eq!(a*2.0, Complex(6.0, 22.0));
        assert_eq!(2.0*a, Complex(6.0, 22.0));
        a *= 5.0;
        assert_eq!(a, Complex(15.0, 55.0));
        assert_eq!(a*(-2.0), Complex(-30.0, -110.0));
        assert_eq!(-2.0*a, Complex(-30.0, -110.0));
    }

    #[test]
    fn test_div() {
        let mut a = Complex(5.0, 12.0);
        let b = Complex(4.0, -3.0);
        assert_eq!(a/b, Complex(-0.64, 2.52));
        a /= Complex(0.0, 1.0);
        assert_eq!(a, Complex(12.0, -5.0));
        assert_eq!(a/2.0, Complex(6.0, -2.5));
        assert_eq!(15.0/b, Complex(2.4, 1.8));
        a /= 2.0;
        assert_eq!(a, Complex(6.0, -2.5));
        assert_eq!(a/(-2.0), Complex(-3.0, 1.25));
        assert_eq!(-10.0/b, Complex(-1.6, -1.2));
    }

    #[test]
    fn test_real_imag() {
        let a = Complex(10.0, -3.0);
        assert_eq!(a.real(), 10.0);
        assert_eq!(a.imag(), -3.0);
    }

    #[test]
    fn test_norm_arg() {
        let r = 5.0;
        let (p, q) = (3.0f64, 4.0f64);
        let arg = p.atan2(q);
        let a = Complex::from_polar(r, arg);
        assert_eq!(a, Complex(q, p));
        let a = Complex(q, p);
        assert_eq!(a.sqnorm(), r*r);
        assert_eq!(a.norm(), r);
        assert_eq!(a.arg(), arg);
        assert_eq!(a.to_polar(), (r, arg));
    }

    #[test]
    fn test_conjugate() {
        let a = Complex(1.0, 2.0);
        assert_eq!(a.conj(), Complex(1.0, -2.0));
    }

    #[test]
    fn test_exp_ln() {
        let a = Complex(2.0, -1.0);
        assert!(approx_eq(a.exp(), Complex(3.992324048441272, -6.217676312367968)));
        assert!(approx_eq(a.ln(), Complex(0.8047189562170503, -0.4636476090008061)))
    }

    #[test]
    fn test_sin_cos_tan() {
        let a = Complex(2.0, -1.0);
        assert!(approx_eq(a.sin(), Complex(1.4031192506220405, 0.4890562590412937)));
        assert!(approx_eq(a.cos(), Complex(-0.64214812471552, 1.0686074213827783)));
        assert!(approx_eq(a.tan(), Complex(-0.24345820118572534, -1.16673625724092)));
        assert!(approx_eq(a.sinh(), Complex(1.959601041421606, -3.165778513216168)));
        assert!(approx_eq(a.cosh(), Complex(2.0327230070196656, -3.0518977991517997)));
        assert!(approx_eq(a.tanh(), Complex(1.0147936161466335, -0.0338128260798967)));
    }

    #[test]
    fn test_neg_eq_from() {
        let i = Complex::i();
        assert_eq!(-i, Complex(0.0, -1.0));
        assert_eq!(Complex::from(3.0), Complex(3.0, 0.0));
        assert_eq!(Complex(1.0, 0.0), 1.0);
        assert_eq!(1.0, Complex(1.0, 0.0));
    }
}