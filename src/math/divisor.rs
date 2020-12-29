pub fn divisor(n: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    for i in 1.. {
        if i * i > n { break }
        if n % i == 0 {
            ret.push(i);
            if i * i != n { ret.push(n / i); }
        }
    }
    ret.sort();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor() {
        let s = divisor(108);
        assert_eq!(s, vec![1, 2, 3, 4, 6, 9, 12, 18, 27, 36, 54, 108]);
        let s = divisor(1);
        assert_eq!(s, vec![1]);
        let s = divisor(25);
        assert_eq!(s, vec![1, 5, 25]);
        let s = divisor(65536);
        assert_eq!(s, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536]);
    }
}