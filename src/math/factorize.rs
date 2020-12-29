pub fn factorize(n: u64) -> Vec<u64> {
    let mut ret = vec![];
    let mut n = n;
    while n % 2 == 0 {
        ret.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            ret.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 { ret.push(n) }
    ret
}

pub fn factorize_pair(n: u64) -> std::collections::HashMap<u64, u64> {
    let mut ret = std::collections::HashMap::new();
    let mut n = n;
    while n % 2 == 0 {
        *ret.entry(2).or_insert(0) += 1;
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            *ret.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 2;
    }
    if n > 2 { *ret.entry(n).or_insert(0) += 1;}
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(24), vec![2, 2, 2, 3]);
        assert_eq!(factorize(498640), vec![2, 2, 2, 2, 5, 23, 271]);
    }

    #[test]
    fn test_factorize_large_prime() {
        assert_eq!(factorize(1_000_000_000_039), vec![1_000_000_000_039]);
    }
}