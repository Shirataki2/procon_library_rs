//! Verified [AOJ GRL 4A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5089290#1)
//! Verified [AOJ GRL 4B](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5089295#1)

pub mod topological_sort {
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct TopologicalSortGraph {
        graph: Vec<Vec<usize>>,
        degree: Vec<isize>,
    }

    impl TopologicalSortGraph {
        pub fn new(size: usize) -> Self {
            let graph = vec![vec![]; size];
            let degree = vec![0; size];
            Self { graph, degree }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.graph[from].push(to);
            self.degree[to] += 1;
        }

        pub fn sort(&mut self) -> Option<Vec<usize>> {
            let mut q = VecDeque::new();
            let mut ret = vec![];
            for i in 0..self.graph.len() {
                if self.degree[i] == 0 {
                    q.push_back(i);
                }
            }
            while !q.is_empty() {
                let v = q.pop_front().unwrap();
                ret.push(v);
                for to in self.graph[v].iter() {
                    self.degree[*to] -= 1;
                    if self.degree[*to] == 0 {
                        q.push_back(*to);
                    }
                }
            }
            match *self.degree.iter().max().unwrap() {
                0 => Some(ret),
                _  => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::topological_sort::*;

    #[test]
    fn test_topological_sort() {
        let mut g = TopologicalSortGraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 2);
        assert_eq!(g.sort(), Some(vec![0, 1, 2]));
        let mut g = TopologicalSortGraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        assert_eq!(g.sort(), None);
    }
}