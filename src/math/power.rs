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

#[cfg(test)]
mod tests {
    use super::*;

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
}