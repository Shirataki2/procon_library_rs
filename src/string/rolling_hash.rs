//! Verified [AOJ ALDS1 14B](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5081493#1)

pub mod rolling_hash {
    use std::marker::PhantomData;

    type Num = u128;
    
    pub trait Hash {
        fn modulo() -> Num;
        fn base() -> Num;
    }

    macro_rules! define_hash {
        ($id:ident, $base:tt, $modulo:tt) => {
            #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
            pub struct $id;
            impl Hash for $id {
                fn modulo() -> Num { $modulo }
                fn base() -> Num { $base }
            }
        }
    }

    // M = 2 ^ 61 - 1 (Prime)
    // b = 1024578101 (Primitive Root of M)
    define_hash!(Hash61, 1_024_578_101, 2_305_843_009_213_693_951);
    
    define_hash!(Hash1e9p7, 10_007, 1_000_000_007);
    
    pub struct RollingHash<H> {
        size: usize,
        pow: Vec<Num>,
        hash: Vec<Num>,
        __phantom: PhantomData<H>
    }

    impl<H> RollingHash<H>
    where
        H: Hash
    {
        pub fn new(s: &[u8]) -> Self {
            let n = s.len();
            let mut pow = vec![1; n + 1];
            let mut hash = vec![0; n + 1];
            for i in 0..n {
                pow[i+1] = pow[i] * H::base() % H::modulo();
                hash[i+1] = (hash[i] * H::base() + s[i] as Num) % H::modulo();
            }
            Self { size: n, pow, hash, __phantom: PhantomData }
        }

        pub fn get(&self, l: usize, r: usize) -> Num {
            (self.hash[r] + H::modulo() - (self.hash[l] * self.pow[r - l]) % H::modulo()) % H::modulo()
        }
    }

    pub fn find_substring<H>(s: &RollingHash<H>, t: &RollingHash<H>) -> Vec<usize>
    where
        H: Hash
    {
        assert!(s.size >= t.size);
        let th = t.get(0, t.size);
        let mut indices = vec![];
        for i in 0..=(s.size - t.size) {
            let sh = s.get(i, i+t.size);
            if sh == th {
                indices.push(i);
            }
        }
        indices
    }
}

#[cfg(test)]
mod tests {
    use super::rolling_hash::*;

    #[test]
    fn test_simple_rolling_hash() {
        let s = "unvhusmjlvieloveuybouqvnqjygutqlovedkfsdfgheaiuloveaeiuvaygayfg".as_bytes();
        let t = "love".as_bytes();
        let rhs = RollingHash::<Hash61>::new(&s);
        let rht = RollingHash::<Hash61>::new(&t);
        let indices = find_substring(&rhs, &rht);
        assert_eq!(indices, vec![12, 31, 47]);
    }
}