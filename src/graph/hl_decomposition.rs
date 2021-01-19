pub mod hl_decomposition {
    type UnweightedGraph = Vec<Vec<usize>>;

    pub struct HeavyLightDecomposition {
        graph: UnweightedGraph,
        size: Vec<isize>,
        in_idx: Vec<isize>,
        out_idx: Vec<isize>,
        head: Vec<isize>,
        rev: Vec<isize>,
        par: Vec<isize>,
    }

    impl HeavyLightDecomposition {
        pub fn new(graph: UnweightedGraph) -> Self{
            let n = graph.len();
            Self {
                graph,
                size: vec![0; n],
                in_idx: vec![0; n],
                out_idx: vec![0; n],
                head: vec![0; n],
                rev: vec![0; n],
                par: vec![0; n],
            }
        }

        fn dfs_sz(&mut self, idx: usize, p: isize) {
            self.par[idx] = p;
            self.size[idx] = 1;
            let n = self.graph[idx].len();
            if n > 0 && self.graph[idx][0] == p as usize {
                self.graph[idx].swap(0, n-1);
            }
            for &to in self.graph[idx].clone().iter() {
                if to != p as usize {
                    self.dfs_sz(to, idx as isize);
                    self.size[idx] += self.size[to];
                    if self.size[self.graph[idx][0]] < self.size[to] {
                        self.graph[idx].swap(0, to);
                    }
                }
            }
        }

        fn dfs_hld(&mut self, idx: usize, p: isize, mut t: &mut isize) {
            self.in_idx[idx] = *t;
            *t += 1;
            self.rev[self.in_idx[idx] as usize] = idx as isize;
            for &to in self.graph[idx].clone().iter() {
                if to != p as usize {
                    self.head[to] = if self.graph[idx][0] == to {
                        self.head[idx]
                    } else {
                        to as isize
                    };
                    self.dfs_hld(to, idx as isize, &mut t);
                }
            }
            self.out_idx[idx] = *t;
        }

        pub fn build(&mut self) {
            self.dfs_sz(0, -1);
            let mut t = 0;
            self.dfs_hld(0, -1, &mut t);
        }

        pub fn ancestor(&self, mut v: usize, mut k: usize) -> usize {
            loop {
                let u = self.head[v] as usize;
                if self.in_idx[v] - k as isize >= self.in_idx[u] {
                    return self.rev[self.in_idx[v] as usize - k] as usize;
                }
                k -= (self.in_idx[v] + 1 - self.in_idx[u]) as usize;
                v = self.par[u] as usize;
            }
        }

        pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
            loop {
                if self.in_idx[u] > self.in_idx[v] {
                    std::mem::swap(&mut u, &mut v);
                }
                if self.head[u] == self.head[v] {
                    return u;
                }
                v = self.par[self.head[v] as usize] as usize;
            }
        }
    }
}
