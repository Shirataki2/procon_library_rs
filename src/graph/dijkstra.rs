//! Verified [AOJ GRL 1A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5083581#1)
use crate::math::algebra::num_trait;
use crate::graph::base::graph;

pub mod dijkstra {
    use super::num_trait::*;
    use super::graph::*;
    use std::cmp::*;
    use std::collections::BinaryHeap;

    #[derive(Debug, Clone)]
    pub struct Dijkstra<'a, T> {
        graph: &'a Graph<T>,
        pub dists: Vec<T>,
        backs: Vec<isize>,
    }

    impl<'a, T: AbelGroup + Bounded + Eq + Ord> Dijkstra<'a, T> {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let n = graph.0.len();
            let dists = vec![T::max_value(); n];
            let backs = vec![-1; n];
            Self { graph, dists, backs }
        }

        pub fn build_graph(&mut self, s: usize) {
            let mut pq = BinaryHeap::new();
            self.dists[s] = T::zero();
            pq.push(Edge::new(s, self.dists[s]));
            while !pq.is_empty() {
                let p = pq.pop().unwrap();
                let v = p.to;
                if self.dists[v] < p.cost { continue; }
                for edge in self.graph[v].iter() {
                    if self.dists[edge.to] > self.dists[v] + edge.cost {
                        self.dists[edge.to] = self.dists[v] + edge.cost;
                        self.backs[edge.to] = v as isize;
                        pq.push(Edge::new(edge.to, self.dists[edge.to]))
                    }
                }
            }
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
    use super::dijkstra::*;

    #[test]
    fn test_dijkstra_1() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 5);
        let mut g = Dijkstra::new(&g);
        g.build_graph(0);
        assert_eq!(g.dists, vec![0, 1, 3, 4]);
        assert_eq!(g.restore(3), vec![1, 2, 3]);
        assert_eq!(g.restore(2), vec![1, 2]);
    }

    #[test]
    fn test_dijkstra_2() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(2, 0, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(3, 1, 1);
        g.add_edge(3, 2, 5);
        let mut g = Dijkstra::new(&g);
        g.build_graph(1);
        assert_eq!(g.dists, vec![3, 0, 2, std::i32::MAX]);
        assert_eq!(g.restore(3), vec![]);
    }
}