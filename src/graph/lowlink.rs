//! Verified [AOJ GRL 3A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5089112#1)
//! Verified [AOJ GRL 3B](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5089118#1)
use crate::graph::base::graph;

pub mod lowlink {
    use super::graph::*;
    use std::cmp::*;

    #[derive(Debug)]
    pub struct LowLink<'a, T> {
        graph: &'a Graph<T>,
        used: Vec<bool>,
        ord: Vec<usize>,
        low: Vec<usize>,
        pub articulations: Vec<usize>,
        pub bridges: Vec<(usize, usize)>,
    }

    impl<'a, T: std::fmt::Debug> LowLink<'a, T> {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let used = vec![false; graph.0.len()];
            let ord = vec![0; graph.0.len()];
            let low = vec![0; graph.0.len()];
            let articulations = vec![];
            let bridges = vec![];
            Self { graph, used, ord, low, articulations, bridges }
        }

        fn dfs(&mut self, idx: usize, mut k: usize, par: isize) -> usize {
            self.used[idx] = true;
            self.ord[idx] = k;
            k += 1;
            self.low[idx] = self.ord[idx];
            let mut is_articulation = false;
            let mut ctr = 0;
            for edge in self.graph[idx].iter() {
                if !self.used[edge.to] {
                    ctr += 1;
                    k = self.dfs(edge.to, k, idx as isize);
                    self.low[idx] = min(self.low[idx], self.low[edge.to]);
                    is_articulation |= par != -1 && self.low[edge.to] >= self.ord[idx];
                    if self.ord[idx] < self.low[edge.to] {
                        self.bridges.push((min(idx, edge.to), max(idx, edge.to)));
                    }
                } else if edge.to != (par as usize) {
                    self.low[idx] = min(self.low[idx], self.ord[edge.to]);
                }
            }
            is_articulation |= par == -1 && ctr > 1;
            if is_articulation {
                self.articulations.push(idx);
            }
            k
        }

        pub fn build(&mut self) {
            let mut k = 0;
            for i in 0..self.graph.0.len() {
                if !self.used[i] {
                    k = self.dfs(i, k, -1);
                }
            }
            self.articulations.sort();
            self.bridges.sort();
        }
    }
}

#[cfg(test)]
mod tests {
    //! ```
    //!   0 - 1
    //!  / \ / \
    //! 3   2   5
    //! |
    //! 4
    //! ```
    use super::lowlink::*;
    use super::graph::Graph;
    
    #[test]
    fn test_find_articulation_point() {
        let mut g = Graph::new(6);
        g.add_edge_undirected(0, 1, 0);
        g.add_edge_undirected(0, 2, 0);
        g.add_edge_undirected(0, 3, 0);
        g.add_edge_undirected(1, 2, 0);
        g.add_edge_undirected(1, 5, 0);
        g.add_edge_undirected(3, 4, 0);
        let mut g = LowLink::new(&g);
        g.build();
        assert_eq!(g.articulations, vec![0, 1, 3]);
    }

    #[test]
    fn test_find_bridge() {
        let mut g = Graph::new(6);
        g.add_edge_undirected(0, 1, 0);
        g.add_edge_undirected(0, 2, 0);
        g.add_edge_undirected(0, 3, 0);
        g.add_edge_undirected(1, 2, 0);
        g.add_edge_undirected(1, 5, 0);
        g.add_edge_undirected(3, 4, 0);
        let mut g = LowLink::new(&g);
        g.build();
        assert_eq!(g.bridges, vec![(0, 3), (1, 5), (3, 4)]);
    }
}
