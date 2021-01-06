//! Verified [Library Checker](https://judge.yosupo.jp/submission/34224)

pub mod segtree {
    pub struct SegTree<T>
    where
        T: Copy,
    {
        size: usize,
        data: Vec<T>,
        f: fn(&T, &T) -> T,
        id: T,
    }

    impl<T> SegTree<T>
    where
        T: Copy,
    {
        pub fn new(n: usize, f: fn(&T, &T) -> T, id: T) -> SegTree<T> {
            let mut size = 1;
            while n > size { size <<= 1; }
            let data = vec![id; 2*size];
            Self { size, data, f, id }
        }

        pub fn set(&mut self, k: usize, v: T) {
            self.data[k + self.size] = v;
        }

        pub fn get(&self, k: usize) -> T {
            self.data[k + self.size]
        }

        pub fn build(&mut self) {
            for k in (1..self.size).rev() {
                self.data[k] = (self.f)(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }

        pub fn update(&mut self, k: usize, v: T) {
            let mut k = k + self.size;
            self.data[k] = v;
            while k > 1 {
                self.data[k >> 1] = (self.f)(&self.data[k], &self.data[k^1]);
                k >>= 1;
            }
        }

        pub fn query(&self, left: usize, right: usize) -> T {
            let mut s = self.id;
            let mut l = left + self.size;
            let mut r = right + self.size;
            while l < r {
                if (l & 1) > 0 {
                    s = (self.f)(&s, &self.data[l]);
                    l += 1;
                }
                if (r & 1) > 0{
                    s = (self.f)(&s, &self.data[r - 1]);
                }
                l >>= 1;
                r >>= 1;
            }
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::segtree::*;

    #[test]
    fn test_small_segtree_sum() {
        let mut seg = SegTree::new(
            10, |&a, &b| a + b, 0
        );
        for i in 0..10 {
            seg.update(i, i);
        }
        assert_eq!(seg.query(0, 9), 36);
        assert_eq!(seg.query(3, 10), 42);
        assert_eq!(seg.query(4, 6), 9);
        seg.update(6, 1);
        assert_eq!(seg.query(0, 9), 31);
        assert_eq!(seg.query(3, 10), 37);
        assert_eq!(seg.query(4, 6), 9);
    }

    #[test]
    fn test_small_segtree_sum2() {
        let mut seg = SegTree::new(
            10, |&a, &b| a + b, 0
        );
        for i in 0..10 {
            seg.set(i, i);
        }
        seg.build();
        assert_eq!(seg.query(0, 9), 36);
        assert_eq!(seg.query(3, 10), 42);
        assert_eq!(seg.query(4, 6), 9);
        seg.update(6, 1);
        assert_eq!(seg.query(0, 9), 31);
        assert_eq!(seg.query(3, 10), 37);
        assert_eq!(seg.query(4, 6), 9);
        assert_eq!(seg.get(0), 0);
    }
}
