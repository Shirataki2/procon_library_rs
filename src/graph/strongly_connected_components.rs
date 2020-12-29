//! Verified [AOJ GRL 3C](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5089244#1)
use crate::graph::base::graph;
use crate::math::algebra::num_trait;

pub mod strongly_connected_components {
    use super::graph::*;
    use super::num_trait::*;

    #[derive(Debug)]
    pub struct StronglyConnectedComponents<'a, T> {
        graph: &'a Graph<T>,
        rgraph: Graph<T>,
        visited: Vec<bool>,
        cmp: Vec<isize>,
        ord: Vec<usize>,
        components: Vec<Vec<usize>>,
    }

    impl<'a, T> StronglyConnectedComponents<'a, T>
    where
        T: AbelGroup + Eq + Ord
    {
        pub fn new(graph: &'a Graph<T>) -> Self {
            let n = graph.0.len();
            let mut rgraph = Graph::<T>::new(n);
            for v in 0..n {
                for edge in graph[v].iter() {
                    rgraph[edge.to].push(Edge { to: v, cost: edge.cost })
                }
            }
            let visited = vec![false; n];
            let cmp = vec![-1; n];
            let ord = vec![];
            let components = vec![];
            Self { graph, rgraph, visited, cmp, ord, components }
        }

        pub fn same(&self, s: usize, t: usize) -> bool {
            self.cmp[s] == self.cmp[t]
        }

        fn dfs(&mut self, s: usize) {
            if self.visited[s] { return }
            self.visited[s] = true;
            for edge in self.graph[s].iter() {
                self.dfs(edge.to);
            }
            self.ord.push(s);
        }

        fn rdfs(&mut self, s: usize, ctr: isize) {
            if self.cmp[s] != -1 { return }
            self.cmp[s] = ctr;
            if self.components.len() <= ctr as usize {
                self.components.push(Vec::new());
            }
            self.components[ctr as usize].push(s);
            for edge in self.rgraph[s].clone().iter() {
                self.rdfs(edge.to, ctr);
            }
        }

        pub fn build(&mut self) {
            for i in 0..self.graph.0.len() {
                self.dfs(i);
            }
            self.ord.reverse();
            let mut ctr = 0;
            for v in self.ord.clone().iter() {
                if self.cmp[*v] != -1 { continue }
                self.rdfs(*v, ctr);
                if self.components.len() > 1 {
                    self.components[ctr as usize].reverse();
                }
                ctr += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::Graph;
    use super::strongly_connected_components::*;

    #[test]
    fn test_strongly_connected_components() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 4, 1);
        g.add_edge(4, 3, 1);
        g.add_edge(3, 2, 1);
        let mut scc = StronglyConnectedComponents::new(&g);
        scc.build();
        assert!(scc.same(0, 1));
        assert!(!scc.same(0, 3));
        assert!(scc.same(2, 3));
        assert!(scc.same(3, 4));
    }
}