//! Verified [AOJ GRL 2A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5086976#1)
use crate::math::algebra::num_trait;

pub mod prim {
    use super::num_trait::*;
    use std::cmp::*;
    use std::collections::BinaryHeap;

    #[derive(Debug, Clone)]
    pub struct Edge<T: AbelGroup + Eq + Ord> {
        to: usize,
        cost: T,
    }

    impl<T: AbelGroup + Eq + Ord> PartialEq for Edge<T> {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }
    impl<T: AbelGroup + Eq + Ord> PartialOrd for Edge<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.cost.partial_cmp(&other.cost)
        }
    }
    impl<T: AbelGroup + Eq + Ord> Eq for Edge<T> {}
    impl<T: AbelGroup + Eq + Ord> Ord for Edge<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost.cmp(&other.cost)
        }
    }

    pub struct Prim<T: AbelGroup + Eq + Ord> {
        edges: Vec<Vec<Edge<T>>>,
    }

    impl<T: AbelGroup + Eq + Ord> Prim<T> {
        pub fn new(size: usize) -> Self {
            let edges = vec![vec![]; size];
            Self { edges }
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self.edges[from].push(Edge { to, cost });
            self.edges[to].push(Edge { to: from, cost });
        }

        pub fn build(&mut self) -> T {
            let n = self.edges.len();
            let mut visited = vec![false; n];
            let mut heap = BinaryHeap::new();
            let s = Edge { to: 0, cost: T::zero() };
            heap.push(Reverse(&s));
            let mut weight_sum = T::zero();
            while !heap.is_empty() {
                let Edge { to, cost } = heap.pop().unwrap().0;
                if visited[*to] { continue }
                visited[*to] = true;
                weight_sum += *cost;
                for edge in self.edges[*to].iter() {
                    heap.push(Reverse(edge));
                }
            }
            weight_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::prim::*;
    #[test]
    fn test_prim() {
        let mut pr = Prim::new(6);
        pr.add_edge_undirected(0, 1, 1);
        pr.add_edge_undirected(0, 2, 3);
        pr.add_edge_undirected(1, 2, 1);
        pr.add_edge_undirected(1, 3, 7);
        pr.add_edge_undirected(2, 4, 1);
        pr.add_edge_undirected(1, 4, 3);
        pr.add_edge_undirected(3, 4, 1);
        pr.add_edge_undirected(3, 5, 1);
        pr.add_edge_undirected(4, 5, 6);
        let w = pr.build();
        assert_eq!(w, 5);
    }
}