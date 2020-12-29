//! Verified [AOJ DSL 1B](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5082302)

use crate::math::algebra::num_trait;

pub mod weighted_unionfind {
    use super::num_trait::*;
    use std::mem::swap;

    #[derive(Debug)]
    pub struct WeightedUnionFind<T> {
        pub parent: Vec<usize>,
        pub rank: Vec<usize>,
        pub weights: Vec<T>,
    }

    impl<T> WeightedUnionFind<T>
    where
        T: AbelGroup
    {
        pub fn new(n: usize) -> Self {
            let parent = (0..n).collect();
            let rank = vec![0; n];
            let weights = vec![T::zero(); n];
            Self { parent, rank, weights }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let r = self.root(self.parent[x]);
                self.weights[x] = self.weights[x] + self.weights[self.parent[x]];
                self.parent[x] = r;
                self.parent[x]
            }
        }

        pub fn weight(&mut self, x: usize) -> T {
            self.root(x);
            self.weights[x]
        }

        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn merge(&mut self, mut x: usize, mut y: usize, mut w: T) -> bool {
            w = w + self.weight(x);
            w = w + -self.weight(y);
            x = self.root(x);
            y = self.root(y);
            if x == y { return false }
            if self.rank[x] < self.rank[y] {
                swap(&mut x, &mut y);
                w = -w;
            }
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
            self.parent[y] = x;
            self.weights[y] = w;
            true
        }

        pub fn diff(&mut self, x: usize, y: usize) -> T {
            self.weight(y) + -self.weight(x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::weighted_unionfind::*;

    #[test]
    fn test_weighted_unionfind() {
        let mut uf = WeightedUnionFind::<i64>::new(5);
        uf.merge(0, 2, 5);
        uf.merge(1, 2, 3);
        assert_eq!(uf.diff(0, 1), 2);
        uf.merge(1, 4, 8);
        assert_eq!(uf.diff(0, 4), 10);
    }
}