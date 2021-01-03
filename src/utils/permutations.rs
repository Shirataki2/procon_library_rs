pub trait LexicalPermutation {
    fn prev_permutation(&mut self) -> bool;
    fn next_permutation(&mut self) -> bool;
}

impl<T: Ord> LexicalPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] { i -= 1; }
        if i == 0 { return false; }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1] { j -= 1; }
        self.swap(j, i-1);
        self[i..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() <= 1 { return false; }
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] { i -= 1; }
        if i == 0 { return false; }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1] { j -= 1; }
        self.swap(i-1, j);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut v = vec![0, 5, 8, 10];
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 5, 10, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 8, 5, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 8, 10, 5]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 10, 5, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[0, 10, 8, 5]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 0, 8, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 0, 10, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 8, 0, 10]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 8, 10, 0]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 10, 0, 8]);
        assert!(v.next_permutation());
        assert_eq!(&v, &[5, 10, 8, 0]);
    }

    #[test]
    fn test_prev_permutation() {
        let mut v = vec![5, 10, 8, 0];
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 10, 0, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 8, 10, 0]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 8, 0, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 0, 10, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[5, 0, 8, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 10, 8, 5]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 10, 5, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 8, 10, 5]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 8, 5, 10]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 5, 10, 8]);
        assert!(v.prev_permutation());
        assert_eq!(&v, &[0, 5, 8, 10]);
        assert!(!v.prev_permutation());
    }
}