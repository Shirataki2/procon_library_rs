//! Verified [Library Checker](https://judge.yosupo.jp/submission/34201)

pub mod sparse_table {
    #[derive(Debug)]
    pub struct SparseTable<T> {
        pub data: Vec<T>,
        table: Vec<Vec<usize>>,
        logs: Vec<usize>,
        op: OperationType,
    }

    #[derive(Debug)]
    pub enum OperationType {
        Min, Max
    }

    impl<T: Ord + Clone + Copy> SparseTable<T> {
        pub fn new(v: &[T], op: OperationType) -> Self {
            let n = v.len();
            let data = v.to_vec();
            let mut logs = vec![0; n+1];
            for i in 2..=n {
                logs[i] = logs[i >> 1] + 1;
            }
            let mut table = vec![vec![0; logs[n]+1]; n];
            for i in 0..n {
                table[i][0] = i;
            }
            for k in (1..n).take_while(|j| (1 << j) <= n) {
                for i in (0..n).take_while(|j| j + (1 << k) <= n) {
                    let v1 = table[i][k - 1];
                    let v2 = table[i + (1 << (k - 1))][k - 1];
                    let f = match op {
                        OperationType::Min => { data[v1] < data[v2] }
                        OperationType::Max => { data[v1] > data[v2] }
                    };
                    table[i][k] = if f { v1 } else { v2 };
                }
            }
            Self { data, table, logs, op }
        }

        /// Returns the index of the maximum/minimum value of
        /// the array in the **closed interval** [s..t].
        pub fn query(&self, s: usize, t: usize) -> usize {
            let d = t - s + 1;
            let k = self.logs[d];
            let (v1, v2) = (self.table[s][k], self.table[t + 1 - (1 << k)][k]);
            let f = match self.op {
                OperationType::Min => { self.data[v1] < self.data[v2] },
                OperationType::Max => { self.data[v1] > self.data[v2] },
            };
            if f { v1 } else { v2 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sparse_table::*;

    #[test]
    fn test_min_sparse_table() {
        let v = vec![-7, 4, 8, 1, 6, 7, 10, -1, 0, 4, 9, 11];
        let st = SparseTable::new(&v, OperationType::Min);
        for i in 0..v.len()-1 {
            for j in i..v.len()-1 {
                let m = (i..=j).fold(100, |acc, x| std::cmp::min(acc, v[x]));
                assert_eq!(v[st.query(i, j)], m);
            }
        }
    }
    #[test]
    fn test_max_sparse_table() {
        let v = vec![-7, 4, 8, 1, 6, 7, 10, -1, 0, 4, 9, 11];
        let st = SparseTable::new(&v, OperationType::Max);
        for i in 0..v.len()-1 {
            for j in i..v.len()-1 {
                let m = (i..=j).fold(-100, |acc, x| std::cmp::max(acc, v[x]));
                eprintln!("{} {}", i, j);
                assert_eq!(v[st.query(i, j)], m);
            }
        }
    }
}