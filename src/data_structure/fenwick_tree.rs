use crate::math::algebra::num_trait;

pub mod fenwick_tree {
    use super::num_trait::*;

    pub struct FenwickTree<T> {
        data: Vec<T>,
        f: fn(&T, &T) -> T,
    }

    impl<T: Monoid> FenwickTree<T> {
        pub fn new(size: usize, f: fn(&T, &T) -> T) -> Self {
            let data = vec![T::zero(); size+1];
            Self { data, f }
        }

        pub fn add(&mut self, k: usize, v: T) {
            let mut k = k as isize;
            k += 1;
            while k < self.data.len() as isize {
                self.data[k as usize] = (self.f)(&self.data[k as usize], &v);
                k += k & -k;
            }
        }

        pub fn sum(&self, k: usize) -> T {
            let mut ret = T::zero();
            let mut k = k as isize;
            k += 1;
            while k > 0 {
                ret = (self.f)(&ret, &self.data[k as usize]);
                k -= k & -k;
            }
            ret
        }
    }

    impl<T: AbelGroup> FenwickTree<T> {
        pub fn lower_bound(&self, mut v: T) -> usize {
            if v <= T::zero() { return 0; }
            let mut i = 0;
            let mut k = ((self.data.len() - 1).next_power_of_two() + 1) as i64;
            while k > 0 {
                if i + k < self.data.len() as i64 && self.data[(i + k) as usize] < v {
                    v -= self.data[(i + k) as usize];
                    i += k;
                }
                k >>= 1;
            }
            i as usize
        }

    }
}

#[cfg(test)]
mod tests {
    use super::fenwick_tree::*;

    #[test]
    fn test_small_query() {
        let mut ft = FenwickTree::<i64>::new(5, |&x, &y| x + y);
        ft.add(0, 1);
        ft.add(1, 2);
        ft.add(2, 3);
        ft.add(3, 4);
        ft.add(4, 5);
        assert_eq!(ft.sum(4), 15);
        assert_eq!(ft.sum(3), 10);
        assert_eq!(ft.sum(2), 6);
        assert_eq!(ft.sum(1), 3);
        assert_eq!(ft.sum(0), 1);
        ft.add(0, -1);
        assert_eq!(ft.sum(4), 14);
    }
}
