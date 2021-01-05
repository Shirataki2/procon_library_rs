//! Verified [AtCoder Typical Contest 001 C - 高速フーリエ変換](https://atcoder.jp/contests/atc001/submissions/19077522)
use crate::math::complex::complex;

pub mod fft {
    use super::complex::*;

    pub struct FastFourierTransform;

    impl FastFourierTransform {
        fn bit_reverse(f: &mut Vec<Complex>) {
            let mut i = 0;
            for j in 1..f.len()-1 {
                let mut k = f.len() >> 1;
                while { i ^= k; k > i } { k >>= 1; }
                if i > j { f.swap(i, j); }
            }
        }

        fn dft(f: &mut Vec<Complex>, inverse: bool) {
            let size = f.len();
            let pi = std::f64::consts::PI * if inverse { -1.0 } else { 1.0 };
            FastFourierTransform::bit_reverse(f);
            for i in (0..).map(|i| 1 << i).take_while(|&i| i < size) {
                for k in 0..i {
                    let w = Complex::from_polar(1.0, k as f64 * pi / i as f64);
                    for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < size) {
                        let s = f[j + k];
                        let t = f[j + k + i] * w;
                        f[j + k] = s + t;
                        f[j + k + i] = s - t;
                    }
                }
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

    #[test]
    fn test_bit_reverse() {
        let mut v = vec![1, 2, 4, 8, 16, 32, 64, 128];
        let mut i = 0;
        for j in 1..v.len()-1 {
            let mut k = v.len() >> 1;
            while { i ^= k; k > i } { k >>= 1; }
            if i > j { v.swap(i, j); }
        }
        assert_eq!(v, vec![1, 16, 4, 64, 2, 32, 8, 128]);
    }
}