use crate::math::algebra::num_trait;

pub mod graph {
    use super::num_trait::*;
    use std::cmp::*;
    use std::ops::*;

    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
    pub struct Edge<T> {
        pub to: usize,
        pub cost: T,
    }
    impl<T: AbelGroup + Eq + Ord> Edge<T> {
        pub fn new(to: usize, cost: T) -> Self {
            Self { to, cost }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Graph<T>(pub Vec<Vec<Edge<T>>>);

    impl<T: AbelGroup + Eq + Ord> Graph<T> {
        pub fn new(n: usize) -> Self {
            let graph = vec![vec![]; n];
            Self(graph)
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
            self[from].push(Edge::new(to, cost));
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, cost: T) {
            self[from].push(Edge::new(to, cost));
            self[to].push(Edge::new(from, cost));
        }
    }

    impl<T> Index<usize> for Graph<T> {
        type Output = Vec<Edge<T>>;
        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl<T> IndexMut<usize> for Graph<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.0[index]
        }
    }
}