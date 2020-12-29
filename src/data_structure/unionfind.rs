//! Verified [AOJ DSL 1A](http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5082152)
//! Verified [Library Checker](https://judge.yosupo.jp/submission/34132)

pub mod unionfind {
    pub struct UnionFind {
        pub parent: Vec<usize>,
        pub sizes: Vec<usize>,
        pub size: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                sizes: vec![1; n],
                size: n,
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x == self.parent[x] {
                x
            } else {
                let p = self.parent[x];
                self.parent[x] = self.find(p);
                self.parent[x]
            }
        }

        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let (px, py) = (self.find(x), self.find(y));
            if px == py { return false }
            let (l, r) = if self.sizes[px] > self.sizes[py] {
                (px, py)
            } else {
                (py, px)
            };
            self.parent[l] = r;
            self.sizes[r] += self.sizes[l];
            self.sizes[l] = 0;
            self.size -= 1;
            true
        }

        pub fn is_same(&mut self, x: usize, y: usize) -> bool {
            let (px, py) = (self.find(x), self.find(y));
            px == py
        }
    }
}

#[cfg(test)]
mod tests {
    use super::unionfind::*;

    #[test]
    fn unionfind_test() {
        let mut uf = UnionFind::new(5);
        uf.unite(0, 1);
        uf.unite(0, 2);
        uf.unite(3, 4);
        assert!(uf.find(1) == uf.find(2));
        assert!(uf.find(1) != uf.find(3));
        assert_eq!(uf.sizes.iter().max(), Some(&3));
    }
}