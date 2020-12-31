//! Verified [AtCoder Typical Contest 001 C - 高速フーリエ変換](https://atcoder.jp/contests/atc001/submissions/19075529)
// TODO: バタフライ演算で高速化
use crate::math::complex::complex;

pub mod fft {
    use super::complex::*;

    pub struct FastFourierTransform;

    impl FastFourierTransform {
        fn dft(f: &mut Vec<Complex>, inverse: bool) {
            let n = f.len();
            if n == 1 { return }
            let pi = std::f64::consts::PI * if inverse { -1.0 } else { 1.0 };
            let mut f0 = f.iter().skip(0).step_by(2).copied().collect::<Vec<_>>();
            let mut f1 = f.iter().skip(1).step_by(2).copied().collect::<Vec<_>>();
            Self::dft(&mut f0, inverse);
            Self::dft(&mut f1, inverse);
            let mut a = Complex(1.0, 0.0);
            let zeta = Complex::from_polar(1.0f64, 2.0 * pi / (n as f64));
            for i in 0..n {
                f[i] = f0[i % (n / 2)] + a * f1[i % (n / 2)];
                a *= zeta;
            }
        }

        pub fn multiply(f: Vec<f64>, g: Vec<f64>) -> Vec<f64> {
            let m = f.len() + g.len() + 1;
            let n = m.next_power_of_two();
            let mut ff = vec![Complex(0.0, 0.0); n];
            let mut gg = vec![Complex(0.0, 0.0); n];
            for i in 0..f.len() { ff[i] += f[i]; }
            for i in 0..g.len() { gg[i] += g[i]; }
            FastFourierTransform::dft(&mut ff, false);
            FastFourierTransform::dft(&mut gg, false);
            for i in 0..n { ff[i] *= gg[i]; }
            FastFourierTransform::dft(&mut ff, true);
            let mut res = vec![0.0; m];
            for i in 0..m { res[i] = ff[i].real() / (n as f64); }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fft::FastFourierTransform;

    #[test]
    fn test_atc001c() {
        let f = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let g = vec![0.0, 1.0, 2.0, 4.0, 8.0];
        let x = FastFourierTransform::multiply(f, g).iter().map(|x| x.round() as i64).collect::<Vec<_>>();
        assert_eq!(x, vec![0, 0, 1, 4, 11, 26, 36, 40, 32, 0, 0]);
    }
}