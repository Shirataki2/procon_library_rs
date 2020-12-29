use crate::math::gcd::extgcd;

pub fn signed_mod(a: i64, m: i64) -> i64 {
    (a % m + m) % m
}

pub fn invmod(a: i64, m: i64) -> i64 {
    let (_d, x, _y) = extgcd(a, m);
    signed_mod(x, m)
}

pub fn powmod(mut x: u64, mut n: u64, modulo: u64) -> u64 {
    let mut ret = 1;
    while n > 0 {
        if n & 1 > 0 {
            ret = (ret * x) % modulo;
        }
        x = (x * x) % modulo;
        n >>= 1;
    }
    ret
}

pub mod modbicoef {
    type Num = i64;

    pub struct Combination {
        m: Num,
        fac: Vec<Num>,
        ifac: Vec<Num>,
    }

    impl Combination {
        pub fn new(n: usize, m: Num) -> Self {
            let mut fac = vec![0; n];
            let mut inv = vec![0; n];
            let mut ifac = vec![0; n];
            fac[0] = 1;
            fac[1] = 1;
            ifac[0] = 1;
            ifac[1] = 1;
            inv[1] = 1;
            for i in 2..n {
                let iu = i as i64;
                fac[i] = fac[i - 1] * iu % m;
                inv[i] = m - inv[m as usize % i] * (m / iu) % m;
                ifac[i] = ifac[i - 1] * inv[i] % m;
            }
            Self { m, fac, ifac }
        }

        pub fn comb(&self, n: usize, r: usize) -> Num {
            let m = self.m;
            if n < r {
                0
            }
            else {
                self.fac[n] * (self.ifac[r] * self.ifac[n - r] % m) % m
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::modbicoef::*;

    #[test]
    fn test_signed_mod() {
        assert_eq!(signed_mod(-1, 7), 6);
        assert_eq!(signed_mod(-14, 7), 0);
        assert_eq!(signed_mod(-100, 7), 5);
        assert_eq!(signed_mod(4, 7), 4);
        assert_eq!(signed_mod(9, 7), 2);
    }

    #[test]
    fn test_invmod() {
        assert_eq!(invmod(3, 7), 5);
        assert_eq!(invmod(2, 429), 215);
        assert_eq!(invmod(123_456_789, 1_000_000_007), 18_633_540);
    }

    #[test]
    fn test_powmod_mod7() {
        assert_eq!(powmod(2, 2, 7), 4);
        assert_eq!(powmod(2, 3, 7), 1);
        assert_eq!(powmod(2, 0, 7), 1);
    }

    #[test]
    fn test_powmod_mod1e9p7() {
        let m = 1_000_000_007;
        assert_eq!(powmod(18, 75, m), 879190096);
        assert_eq!(powmod(977812, 8877774, m), 758213842);
    }

    #[test]
    fn test_combination_small() {
        let c = Combination::new(20, 419);
        assert_eq!(c.comb(12, 1), 12);
        assert_eq!(c.comb(12, 4), 76);
    }

    #[test]
    fn test_combination_large() {
        let c = Combination::new(100_001, 1_000_000_007);
        assert_eq!(c.comb(100_000, 50_000), 149_033_233);
        assert_eq!(c.comb(77_777, 7_777), 508_121_884);
    }
}