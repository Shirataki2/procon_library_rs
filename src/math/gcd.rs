pub fn gcd(a: u64, b: u64) -> u64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b > 0 {
        let (gcd, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (gcd, x, y)
    } else {
        (a, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_small() {
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(60, 75), 15);
        assert_eq!(gcd(19, 48), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 5), 1);
    }

    #[test]
    fn test_lcm_small() {
        assert_eq!(lcm(12, 15), 60);
        assert_eq!(lcm(60, 75), 300);
        assert_eq!(lcm(19, 48), 912);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 5), 5);
    }

    #[test]
    fn test_gcd_large() {
        assert_eq!(gcd(4785420, 4478120), 20);
        assert_eq!(gcd(187812024, 563436072), 187812024);
    }

    #[test]
    fn test_lcm_large() {
        assert_eq!(lcm(4785420, 4478120), 1071484250520);
        assert_eq!(lcm(187812024, 563436072), 563436072);
    }

    #[test]
    fn test_extgcd() {
        assert_eq!(extgcd(12, 15), (3, -1, 1));
        assert_eq!(extgcd(78947, 67465), (1, 29643, -34688));
        assert_eq!(extgcd(47, 998244353), (1, 191153174, -9));
    }
}