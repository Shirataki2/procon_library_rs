pub mod lazy_segtree {
    /// ## Cheetsheet
    ///
    /// ### Range Minimum Query + Range Add Query
    /// ```no_test
    /// v: [0; n]
    /// f(a, b) = min(a, b)
    /// g(a, b) = a + b
    /// h(a, b) = a + b
    /// t0 = inf, u0 = 0
    /// ```
    ///
    /// ### Range Sum Query + Range Add Query
    /// ```no_test
    /// v: [(0, 1); n]
    /// f(a, b) = (a.0 + b.0, a.1 + b.1)
    /// g(a, b) = (a.0 + b * a.1, a.1)
    /// h(a, b) = a + b
    /// t0 = inf, u0 = 0
    /// ```
    pub struct LazySegTree<T, U>
    {
        size: usize,
        height: usize,
        pub data: Vec<T>,
        pub lazy: Vec<U>,
        f: fn(&T, &T) -> T,
        g: fn(&T, &U) -> T,
        h: fn(&U, &U) -> U,
        t0: T,
        u0: U,
    }

    impl<T, U> LazySegTree<T, U>
    where
        T: Clone + Copy,
        U: Clone + Copy + Eq,
    {
        pub fn new(v: Vec<T>, f: fn(&T, &T) -> T, g: fn(&T, &U) -> T, h: fn(&U, &U) -> U, t0: T, u0: U) -> Self {
            let size = v.len();
            let size = size.next_power_of_two();
            // height = log_2 size
            let height = {
                let mut v = 0;
                let mut sz = size;
                while sz > 0 { sz >>= 1; v += 1; }
                v
            };
            let mut data = vec![t0; 2 * size];
            data[size..(v.len() + size)].clone_from_slice(&v[..]);
            let lazy = vec![u0; 2 * size];
            for k in (1..size).rev() {
                data[k] = (f)(&data[2 * k], &data[2 * k + 1]);
            }
            Self { size, height, data, lazy, f, g, h, t0, u0 }
        }

        pub fn set(&mut self, k: usize, v: T) {
            self.data[k + self.size] = v;
        }

        pub fn build(&mut self) {
            for k in (1..self.size).rev() {
                self.data[k] = (self.f)(&self.data[2 * k], &self.data[2 * k + 1]);
            }
        }

        #[inline]
        fn propagate(&mut self, k: usize) {
            if self.lazy[k] != self.u0 {
                self.lazy[2 * k] = (self.h)(&self.lazy[2 * k], &self.lazy[k]);
                self.lazy[2 * k + 1] = (self.h)(&self.lazy[2 * k + 1], &self.lazy[k]);
                self.data[k] = self.reflect(k);
                self.lazy[k] = self.u0;
            }
        }

        #[inline]
        fn reflect(&mut self, k: usize) -> T {
            if self.lazy[k] == self.u0 {
                self.data[k]
            } else {
                (self.g)(&self.data[k], &self.lazy[k])
            }
        }

        #[inline]
        fn recalc(&mut self, mut k: usize) {
            k >>= 1;
            while k > 0 {
                self.data[k] = (self.f)(&self.reflect(2 * k), &self.reflect(2 * k + 1));
                k >>= 1;
            }
        }

        #[inline]
        fn thrust(&mut self, k: usize) {
            for i in (1..=self.height).rev() {
                self.propagate(k >> i);
            }
        }

        pub fn update(&mut self, mut left: usize, mut right: usize, value: U) {
            left += self.size;
            right += self.size - 1;
            self.thrust(left);
            self.thrust(right);
            let mut l = left; let mut r = right + 1;
            while l < r {
                if l & 1 > 0 {
                    self.lazy[l] = (self.h)(&self.lazy[l], &value);
                    l += 1;
                }
                if r & 1 > 0 {
                    r -= 1;
                    self.lazy[r] = (self.h)(&self.lazy[r], &value);
                }
                l >>= 1; r >>= 1;
            }
            self.recalc(left);
            self.recalc(right);
        }

        pub fn query(&mut self, mut a: usize, mut b: usize) -> T {
            a += self.size;
            b += self.size - 1;
            self.thrust(a);
            self.thrust(b);
            let mut l = a; let mut r = b + 1;
            let mut lv = self.t0; let mut rv = self.t0;
            while l < r {
                if l & 1 > 0 {
                    lv = (self.f)(&lv, &self.reflect(l));
                    l += 1;
                }
                if r & 1 > 0 {
                    r -= 1;
                    rv = (self.f)(&self.reflect(l), &rv);
                }
                l >>= 1; r >>= 1;
            }
            (self.f)(&lv, &rv)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lazy_segtree::*;

    #[test]
    fn test_rsq_raq() {
        let mut seg = LazySegTree::new(
            vec![(0, 1); 3],
            |&a, &b| (a.0 + b.0, a.1 + b.1),
            |&a, &b| (a.0 + b * a.1, a.1),
            |&a, &b| a + b,
            (0, 0),
            0
        );
        seg.update(0, 2, 1);
        seg.update(1, 3, 2);
        seg.update(2, 3, 3);
        assert_eq!(seg.query(0, 2).0, 4);
        assert_eq!(seg.query(1, 3).0, 8);
    }

    #[test]
    fn test_raq() {
        let h = vec![2, 4, 2];
        let mut seg = LazySegTree::new(
            h,
            |&a, &b| a + b,
            |&a, &b| a + b,
            |&a, &b| a + b,
            0, 0
        );
        seg.update(0, 2, -2);
        assert_eq!(seg.query(0, 1), 0);
        assert_eq!(seg.query(1, 2), 2);
        assert_eq!(seg.query(2, 3), 2);
        seg.update(1, 3, 5);
        assert_eq!(seg.query(0, 1), 0);
        assert_eq!(seg.query(1, 2), 7);
        assert_eq!(seg.query(2, 3), 7);
    }
}
