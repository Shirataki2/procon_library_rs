//! Verified [AOJ GRL 1B](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5083744)
use crate::math::algebra::num_trait;
use crate::graph::base::graph;

pub mod bellmanford {
    use super::num_trait::*;
    use super::graph::*;
    use std::cmp::*;

    #[derive(Debug, Clone)]
    pub struct BellmanFord<'a, T> {
        graph: &'a Graph<T>,
        pub dists: Vec<T>,
        backs: Vec<isize>,
    }

    impl<'a, T: AbelGroup + Bounded + Eq + Ord> BellmanFord<'a, T> {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let n = graph.0.len();
            let dists = vec![T::max_value(); n];
            let backs = vec![-1; n];
            Self { graph, dists, backs }
        }

        pub fn build_graph(&mut self, s: usize) -> bool {
            self.dists[s] = T::zero();
            let n = self.dists.len();
            for i in 0..n {
                let mut updated = false;
                for from in 0..n {
                    let edges = &self.graph[from];
                    for edge in edges.iter() {
                        if self.dists[from] != T::max_value() &&
                           self.dists[edge.to] > self.dists[from] + edge.cost {
                            self.dists[edge.to] = self.dists[from] + edge.cost;
                            self.backs[edge.to] = from as isize;
                            updated = true;
                        }
                    }
                }
                if !updated {
                    break
                }
                if i == n - 1 {
                    return false
                }
            }
            true
        }

        pub fn restore(&self, mut to: isize) -> Vec<isize> {
            let mut path = vec![];
            if self.backs[to as usize] < 0 {
                path
            } else {
                while to > 0 {
                    path.push(to);
                    to = self.backs[to as usize];
                }
                path.reverse();
                path
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::*;
    use super::bellmanford::*;

    #[test]
    fn test_bellmanford_1() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 5);
        let mut g = BellmanFord::new(&g);
        g.build_graph(0);
        assert_eq!(g.dists, vec![0, 1, 3, 4]);
        assert_eq!(g.restore(3), vec![1, 2, 3]);
        assert_eq!(g.restore(2), vec![1, 2]);
    }

    #[test]
    fn test_bellmanford_2() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(2, 0, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(3, 1, 1);
        g.add_edge(3, 2, 5);
        let mut g = BellmanFord::new(&g);
        g.build_graph(1);
        assert_eq!(g.dists, vec![3, 0, 2, std::i32::MAX]);
        assert_eq!(g.restore(3), vec![]);
    }

    #[test]
    fn test_bellmanford_with_negative_edge() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 3, 4);
        g.add_edge(2, 3, -2);
        g.add_edge(2, 5, 2);
        g.add_edge(3, 5, 4);
        g.add_edge(3, 4, 2);
        g.add_edge(4, 5, 1);
        let mut g = BellmanFord::new(&g);
        assert!(g.build_graph(0));
        assert_eq!(g.dists, vec![0, 2, 5, 3, 5, 6]);
        assert_eq!(g.restore(5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bellmanford_with_negative_circuit() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 3, 4);
        g.add_edge(2, 3, -2);
        g.add_edge(2, 5, 2);
        g.add_edge(3, 5, 4);
        g.add_edge(3, 4, 2);
        g.add_edge(4, 5, 1);
        g.add_edge(3, 1, -2);
        let mut g = BellmanFord::new(&g);
        assert!(!g.build_graph(0));
    }
}