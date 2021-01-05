//! Verified [AOJ GRL 1C](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5085105#1)
use crate::math::algebra::num_trait;
use crate::{min, chmin};

pub mod warshall_floyd {
    use super::{min, chmin};
    use super::num_trait::*;
    use std::ops::*;

    #[derive(Debug)]
    pub struct WarshallFloyd<T>(Vec<Vec<T>>);

    impl<T: AbelGroup + Bounded + Signed> WarshallFloyd<T> {
        #[allow(clippy::needless_range_loop)]
        pub fn new(n: usize) -> Self {
            let mut d = vec![vec![T::max_value(); n]; n];
            for i in 0..n {
                d[i][i] = T::zero();
            }
            Self(d)
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self[from][to] = cost;
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self[from][to] = cost;
            self[to][from] = cost;
        }

        pub fn build_graph(&mut self) {
            let n = self.0.len();
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        if self[i][k] != T::max_value() && self[k][j] != T::max_value() {
                            chmin!(self[i][j], self[i][k] + self[k][j]);
                        }
                    }
                }
            }
        }

        pub fn has_negative_cycle(&self) -> bool {
            let n = self.0.len();
            (0..n).any(|i| self[i][i] < T::zero())
        }
    }

    impl<T> Index<usize> for WarshallFloyd<T> {
        type Output = Vec<T>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<T> IndexMut<usize> for WarshallFloyd<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::warshall_floyd::*;

    #[test]
    fn test_warshall_floyd() {
        let mut wf = WarshallFloyd::new(4);
        wf[0][1] = 1;
        wf[0][2] = 5;
        wf[1][2] = 2;
        wf[1][3] = 4;
        wf[2][3] = 1;
        wf[3][2] = 7;
        wf.build_graph();
        let inf = std::i32::MAX;
        assert_eq!(wf[0], vec![  0,   1, 3, 4]);
        assert_eq!(wf[1], vec![inf,   0, 2, 3]);
        assert_eq!(wf[2], vec![inf, inf, 0, 1]);
        assert_eq!(wf[3], vec![inf, inf, 7, 0]);
    }

    #[test]
    fn test_warshall_floyd_with_negative_cycle() {
        let mut wf = WarshallFloyd::new(4);
        wf[0][1] = 1;
        wf[0][2] = 5;
        wf[1][2] = 2;
        wf[1][3] = 4;
        wf[2][3] = 1;
        wf[3][2] = -7;
        wf.build_graph();
        assert!(wf.has_negative_cycle());
    }
}