//! Verified [AOJ GRL 6A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5091586#1)
use crate::math::algebra::num_trait;

pub mod ford_fulkerson {
    use super::num_trait::*;
    use std::cmp::*;

    #[derive(Debug, Clone, Copy)]
    struct Edge<T> {
        to: usize,
        cap: T,
        rev: i64,
    }

    #[derive(Debug)]
    pub struct FordFulkerson<T> {
        graph: Vec<Vec<Edge<T>>>,
        used: Vec<bool>,
    }

    impl<T: AbelGroup + Bounded + Eq + Ord + std::fmt::Debug> FordFulkerson<T> {
        pub fn new(size: usize) -> Self {
            let graph = vec![vec![]; size];
            let used = vec![false; size];
            Self { graph, used }
        }

        pub fn add_flow(&mut self, from: usize, to: usize, cap: T) {
            let len_to = self.graph[to].len() as i64;
            let len_from = self.graph[from].len() as i64;
            self.graph[from].push(Edge { to, cap, rev: len_to });
            self.graph[to].push(Edge { to: from, cap: T::zero(), rev: len_from });
        }

        fn dfs(&mut self, idx: usize, t: usize, flow: T) -> T {
            if idx == t { return flow }
            self.used[idx] = true;
            for i in 0..self.graph[idx].len() {
                let edge = self.graph[idx][i];
                if edge.cap <= T::zero() || self.used[edge.to] { continue }
                let d = self.dfs(edge.to, t, min(flow, edge.cap));
                if d > T::zero() {
                    self.graph[idx][i].cap -= d;
                    self.graph[edge.to][edge.rev as usize].cap += d;
                    return d
                }
            }
            T::zero()
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> T {
            let mut flow = T::zero();
            loop {
                self.used = vec![false; self.graph.len()];
                let f = self.dfs(s, t, T::max_value());
                if f == T::zero() { return flow }
                flow += f;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ford_fulkerson::*;

    #[test]
    fn test_max_flow() {
        let mut g = FordFulkerson::new(4);
        g.add_flow(0, 1, 2);
        g.add_flow(0, 2, 1);
        g.add_flow(1, 2, 1);
        g.add_flow(1, 3, 1);
        g.add_flow(2, 3, 2);
        let mf = g.max_flow(0, 3);
        assert_eq!(mf, 3);
    }
}