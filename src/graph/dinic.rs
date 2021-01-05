//! Verified [AOJ GRL 6A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5091592#1)
use crate::math::algebra::num_trait;

pub mod dinic {
    use super::num_trait::*;
    use std::cmp::*;
    use std::collections::VecDeque;

    #[derive(Debug, Clone, Copy)]
    struct Edge<T> {
        to: usize,
        cap: T,
        rev: i64,
    }

    #[derive(Debug)]
    pub struct Dinic<T> {
        graph: Vec<Vec<Edge<T>>>,
        min_cost: Vec<T>,
        iter: Vec<usize>,
    }

    impl<T: Ring + Bounded + Eq + Ord> Dinic<T> {
        pub fn new(size: usize) -> Self {
            let graph = vec![vec![]; size];
            let min_cost = vec![];
            let iter = vec![];
            Self { graph, min_cost, iter }
        }

        pub fn add_flow(&mut self, from: usize, to: usize, cap: T) {
            let len_to = self.graph[to].len() as i64;
            let len_from = self.graph[from].len() as i64;
            self.graph[from].push(Edge { to, cap, rev: len_to });
            self.graph[to].push(Edge { to: from, cap: T::zero(), rev: len_from });
        }

        fn bfs(&mut self, s: usize, t: usize) -> bool {
            let size = self.graph.len();
            self.min_cost = vec![T::max_value(); size];
            let mut queue = VecDeque::new();
            self.min_cost[s] = T::zero();
            queue.push_back(s);
            while !queue.is_empty() {
                let p = queue.pop_front().unwrap();
                for edge in self.graph[p].iter() {
                    if edge.cap > T::zero() && self.min_cost[edge.to] == T::max_value() {
                        self.min_cost[edge.to] = self.min_cost[p] + T::one();
                        queue.push_back(edge.to);
                    }
                }
            }
            self.min_cost[t] != T::max_value()
        }

        fn dfs(&mut self, idx: usize, t: usize, flow: T) -> T {
            if idx == t { return flow }
            let n = self.graph[idx].len();
            while self.iter[idx] < n {
                let edge = self.graph[idx][self.iter[idx]];
                if edge.cap > T::zero() && self.min_cost[idx] < self.min_cost[edge.to] {
                    let d = self.dfs(edge.to, t, min(flow, edge.cap));
                    if d > T::zero() {
                        self.graph[idx][self.iter[idx]].cap -= d;
                        self.graph[edge.to][edge.rev as usize].cap += d;
                        return d;
                    }
                }
                self.iter[idx] += 1;
            }
            T::zero()
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> T {
            let mut flow = T::zero();
            while self.bfs(s, t) {
                self.iter = vec![0; self.graph.len()];
                loop {
                    let f = self.dfs(s, t, T::max_value());
                    if f <= T::zero() { break }
                    flow += f;
                }
            }
            flow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dinic::*;

    #[test]
    fn test_max_flow() {
        let mut g = Dinic::new(4);
        g.add_flow(0, 1, 2);
        g.add_flow(0, 2, 1);
        g.add_flow(1, 2, 1);
        g.add_flow(1, 3, 1);
        g.add_flow(2, 3, 2);
        let mf = g.max_flow(0, 3);
        assert_eq!(mf, 3);
    }
}