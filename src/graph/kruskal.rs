//! Verified [AOJ GRL 2A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5085970)
use crate::math::algebra::num_trait;
use crate::data_structure::unionfind::unionfind;

pub mod kruskal {
    use super::num_trait::*;
    use super::unionfind::UnionFind;
    use std::cmp::*;

    pub struct Edge<T: AbelGroup + Eq + Ord> {
        from: usize,
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

    pub struct Kruskal<T: AbelGroup + Eq + Ord> {
        uft: UnionFind,
        edges: Vec<Edge<T>>,
    }

    impl<T: AbelGroup + Eq + Ord> Kruskal<T> {
        pub fn new(size: usize) -> Self {
            let uft = UnionFind::new(size);
            let edges = vec![];
            Self { uft, edges }
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self.edges.push(Edge { from, to, cost });
            self.edges.push(Edge { to, from, cost });
        }

        pub fn build(&mut self) -> T {
            self.edges.sort();
            let mut weight_sum = T::zero();
            for edge in self.edges.iter() {
                if !self.uft.is_same(edge.from, edge.to) {
                    self.uft.unite(edge.from, edge.to);
                    weight_sum += edge.cost;
                }
            }
            weight_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::kruskal::*;

    #[test]
    fn test_kruskal() {
        let mut kr = Kruskal::new(6);
        kr.add_edge_undirected(0, 1, 1);
        kr.add_edge_undirected(0, 2, 3);
        kr.add_edge_undirected(1, 2, 1);
        kr.add_edge_undirected(1, 3, 7);
        kr.add_edge_undirected(2, 4, 1);
        kr.add_edge_undirected(1, 4, 3);
        kr.add_edge_undirected(3, 4, 1);
        kr.add_edge_undirected(3, 5, 1);
        kr.add_edge_undirected(4, 5, 6);
        let w = kr.build();
        assert_eq!(w, 5);
    }
}